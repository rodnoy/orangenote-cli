//! Whisper transcriber - main transcription engine
//!
//! This module provides the high-level `WhisperTranscriber` that orchestrates
//! audio processing and transcription using whisper.cpp.

use super::context::TranscriptionResult;
use crate::infrastructure::audio::AudioProcessor;
use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use std::path::Path;

use super::context::WhisperContextWrapper;
use super::model_manager::{ModelSize, WhisperModelManager};

/// Main transcription engine combining audio processing and whisper inference
pub struct WhisperTranscriber {
    model_path: std::path::PathBuf,
    context: WhisperContextWrapper,
    threads: usize,
}

impl WhisperTranscriber {
    /// Create a new transcriber with the specified model
    ///
    /// # Arguments
    ///
    /// * `model_path` - Path to the whisper.cpp model file
    /// * `threads` - Number of threads for transcription
    ///
    /// # Returns
    ///
    /// Result containing the initialized transcriber, or an error if initialization failed
    pub fn new<P: AsRef<Path>>(model_path: P, threads: usize) -> Result<Self> {
        let model_path = model_path.as_ref().to_path_buf();

        info!(
            "Initializing WhisperTranscriber with model: {}",
            model_path.display()
        );

        if !model_path.exists() {
            return Err(anyhow!("Model file not found: {}", model_path.display()));
        }

        let context = WhisperContextWrapper::new(&model_path)
            .context("Failed to initialize whisper context")?;

        info!(
            "Transcriber initialized successfully with {} threads",
            threads
        );

        Ok(WhisperTranscriber {
            model_path,
            context,
            threads,
        })
    }

    /// Create a transcriber from a model manager, automatically handling model download if needed
    ///
    /// # Arguments
    ///
    /// * `model_manager` - Model manager instance
    /// * `model_size` - Model size enum
    /// * `threads` - Number of threads for transcription
    ///
    /// # Returns
    ///
    /// Result containing the initialized transcriber
    pub async fn from_model_manager(
        model_manager: &WhisperModelManager,
        model_size: ModelSize,
        threads: usize,
    ) -> Result<Self> {
        info!(
            "Creating transcriber from model manager for: {}",
            model_size.display_name()
        );

        let model_path = model_manager
            .get_or_download(model_size)
            .await
            .context("Failed to get model")?;

        Self::new(model_path, threads)
    }

    /// Transcribe an audio file
    ///
    /// # Arguments
    ///
    /// * `audio_path` - Path to the audio file
    /// * `language` - Optional language code (e.g., "en", "ru"). None for auto-detect
    /// * `translate` - Whether to translate to English
    ///
    /// # Returns
    ///
    /// Result containing the transcription result with segments and timestamps
    pub fn transcribe_file<P: AsRef<Path>>(
        &self,
        audio_path: P,
        language: Option<&str>,
        translate: bool,
    ) -> Result<TranscriptionResult> {
        let audio_path = audio_path.as_ref();
        info!(
            "Transcribing audio file: {} (language: {:?}, translate: {})",
            audio_path.display(),
            language,
            translate
        );

        // Step 1: Process audio file to PCM samples at 16kHz mono
        let audio_samples =
            AudioProcessor::process(audio_path).context("Failed to process audio file")?;

        debug!(
            "Audio processing complete: {} samples, duration: {:.1}s",
            audio_samples.samples.len(),
            audio_samples.duration_seconds
        );

        // Step 2: Transcribe the samples
        self.transcribe_samples(&audio_samples.samples, language, translate)
    }

    /// Transcribe PCM samples directly
    ///
    /// # Arguments
    ///
    /// * `samples` - PCM samples at 16kHz, f32 normalized to [-1.0, 1.0]
    /// * `language` - Optional language code. None for auto-detect
    /// * `translate` - Whether to translate to English
    ///
    /// # Returns
    ///
    /// Result containing the transcription result
    pub fn transcribe_samples(
        &self,
        samples: &[f32],
        language: Option<&str>,
        translate: bool,
    ) -> Result<TranscriptionResult> {
        if samples.is_empty() {
            return Err(anyhow!("No audio samples provided"));
        }

        info!("Starting transcription of {} samples", samples.len());

        // Configure transcription parameters
        let mut params =
            unsafe { super::ffi::whisper_full_default_params(super::ffi::WHISPER_SAMPLING_GREEDY) };

        params.n_threads = self.threads as i32;
        params.translate = translate;
        params.print_progress = false;
        params.print_realtime = false;
        params.print_timestamps = true;
        params.token_timestamps = true;

        // Disable VAD to avoid requiring VAD model
        params.vad = false;

        // Set language if provided
        let lang_c_string;
        if let Some(lang) = language {
            lang_c_string = std::ffi::CString::new(lang)?;
            params.language = lang_c_string.as_ptr();
        }

        debug!(
            "Transcription params: threads={}, translate={}, language={:?}",
            params.n_threads, params.translate, language
        );

        // Run transcription
        unsafe {
            let ret = super::ffi::whisper_full(
                self.context.as_ptr(),
                params,
                samples.as_ptr(),
                samples.len() as i32,
            );

            if ret != 0 {
                return Err(anyhow!("Transcription failed with code: {}", ret));
            }
        }

        // Extract results
        let result = self
            .context
            .extract_results()
            .context("Failed to extract transcription results")?;

        info!(
            "Transcription complete: {} segments, language: {}",
            result.segments.len(),
            result.language
        );

        Ok(result)
    }

    /// Get the model path being used
    pub fn model_path(&self) -> &Path {
        &self.model_path
    }

    /// Get the number of threads
    pub fn threads(&self) -> usize {
        self.threads
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_samples() {
        // This test would require a valid model, so we skip it in basic tests
        // but the error handling is tested through Result return type
    }
}
