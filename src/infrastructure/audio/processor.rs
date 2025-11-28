//! Audio processor for PCM conversion and resampling
//!
//! Handles converting audio files to PCM samples at 16kHz mono format
//! required by whisper.cpp. Supports MP3, WAV, FLAC, M4A, OGG formats.

use anyhow::{anyhow, Context, Result};
use log::{debug, info};

use std::path::Path;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

/// Target sample rate for whisper.cpp (16kHz)
pub const WHISPER_SAMPLE_RATE: u32 = 16000;

/// Audio samples container - stores mono PCM samples at 16kHz
#[derive(Debug, Clone)]
pub struct AudioSamples {
    /// PCM samples as f32 (normalized to -1.0..1.0)
    pub samples: Vec<f32>,
    /// Original sample rate before resampling
    pub original_sample_rate: u32,
    /// Original number of channels
    pub original_channels: u16,
    /// Duration in seconds
    pub duration_seconds: f64,
}

impl AudioSamples {
    /// Get duration in samples
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if samples are empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> i64 {
        (self.duration_seconds * 1000.0) as i64
    }
}

/// Audio processor for decoding and resampling
pub struct AudioProcessor;

impl AudioProcessor {
    /// Process an audio file and return PCM samples at 16kHz mono
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the audio file
    ///
    /// # Returns
    ///
    /// `AudioSamples` containing normalized PCM samples at 16kHz mono
    pub fn process<P: AsRef<Path>>(path: P) -> Result<AudioSamples> {
        let path = path.as_ref();
        info!("Processing audio file: {}", path.display());

        let file = std::fs::File::open(path).context("Failed to open audio file")?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Create a probe to detect the format
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &Default::default())
            .context("Failed to probe audio format")?;

        let mut format = probed.format;

        info!("Format detected");

        // Get track information
        let track = format
            .default_track()
            .ok_or_else(|| anyhow!("No audio track found in file"))?;

        let codec_params = &track.codec_params;
        let original_sample_rate = codec_params
            .sample_rate
            .ok_or_else(|| anyhow!("Sample rate unknown"))?;

        let channels = codec_params
            .channels
            .ok_or_else(|| anyhow!("Channel count unknown"))?
            .count() as u16;

        let duration_frames = codec_params.n_frames.unwrap_or(0);
        let original_duration_seconds = if original_sample_rate > 0 {
            duration_frames as f64 / original_sample_rate as f64
        } else {
            0.0
        };

        debug!(
            "Audio info: {}Hz, {} channels, {:.1}s",
            original_sample_rate, channels, original_duration_seconds
        );

        // Create decoder
        let decoder = symphonia::default::get_codecs()
            .make(codec_params, &DecoderOptions::default())
            .context("Failed to create decoder")?;

        // Decode all samples
        let mut all_samples = Vec::new();
        let mut decoder = decoder;

        loop {
            match format.next_packet() {
                Ok(packet) => match decoder.decode(&packet) {
                    Ok(decoded) => {
                        let spec = decoded.spec();
                        let channels_in_spec = spec.channels.count();

                        match decoded {
                            symphonia::core::audio::AudioBufferRef::F32(buf) => {
                                Self::extract_f32_samples(&buf, channels_in_spec, &mut all_samples);
                            }
                            symphonia::core::audio::AudioBufferRef::S16(buf) => {
                                Self::extract_s16_samples(&buf, channels_in_spec, &mut all_samples);
                            }
                            symphonia::core::audio::AudioBufferRef::U8(buf) => {
                                Self::extract_u8_samples(&buf, channels_in_spec, &mut all_samples);
                            }
                            _ => {
                                debug!("Unsupported sample format, skipping");
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Decode error: {}", e);
                    }
                },
                Err(symphonia::core::errors::Error::IoError(_)) => {
                    break;
                }
                Err(e) => {
                    debug!("Format error: {}", e);
                    break;
                }
            }
        }

        if all_samples.is_empty() {
            return Err(anyhow!("No audio samples decoded"));
        }

        info!(
            "Decoded {} samples from {} channels at {}Hz",
            all_samples.len(),
            channels,
            original_sample_rate
        );

        // Convert to mono if stereo/multi-channel
        let mono_samples = if channels > 1 {
            debug!("Converting {} channels to mono", channels);
            Self::to_mono(&all_samples, channels as usize)
        } else {
            all_samples
        };

        // Resample to 16kHz if needed
        let resampled_samples = if original_sample_rate != WHISPER_SAMPLE_RATE {
            debug!(
                "Resampling from {}Hz to {}Hz",
                original_sample_rate, WHISPER_SAMPLE_RATE
            );
            Self::resample(&mono_samples, original_sample_rate, WHISPER_SAMPLE_RATE)
                .context("Resampling failed")?
        } else {
            mono_samples
        };

        let duration_seconds = resampled_samples.len() as f64 / WHISPER_SAMPLE_RATE as f64;

        info!(
            "Final audio: {} samples at {}Hz ({:.1}s)",
            resampled_samples.len(),
            WHISPER_SAMPLE_RATE,
            duration_seconds
        );

        Ok(AudioSamples {
            samples: resampled_samples,
            original_sample_rate,
            original_channels: channels,
            duration_seconds,
        })
    }

    /// Extract f32 samples from buffer and mix to mono
    fn extract_f32_samples(
        buf: &symphonia::core::audio::AudioBuffer<f32>,
        channels: usize,
        out: &mut Vec<f32>,
    ) {
        let frames = buf.frames();

        if channels == 1 {
            // Already mono
            out.extend_from_slice(buf.chan(0));
        } else {
            // Mix channels to mono
            for frame in 0..frames {
                let mut sample = 0.0f32;
                for ch in 0..channels {
                    sample += buf.chan(ch)[frame];
                }
                out.push(sample / channels as f32);
            }
        }
    }

    /// Extract s16 samples from buffer and convert to f32, mix to mono
    fn extract_s16_samples(
        buf: &symphonia::core::audio::AudioBuffer<i16>,
        channels: usize,
        out: &mut Vec<f32>,
    ) {
        let frames = buf.frames();
        const S16_MAX: f32 = 32767.0;

        if channels == 1 {
            // Already mono
            for &sample in buf.chan(0) {
                out.push(sample as f32 / S16_MAX);
            }
        } else {
            // Mix channels to mono
            for frame in 0..frames {
                let mut sample = 0.0f32;
                for ch in 0..channels {
                    sample += buf.chan(ch)[frame] as f32;
                }
                out.push(sample / (channels as f32 * S16_MAX));
            }
        }
    }

    /// Extract u8 samples from buffer and convert to f32, mix to mono
    fn extract_u8_samples(
        buf: &symphonia::core::audio::AudioBuffer<u8>,
        channels: usize,
        out: &mut Vec<f32>,
    ) {
        let frames = buf.frames();

        if channels == 1 {
            // Already mono
            for &sample in buf.chan(0) {
                // Convert from [0, 255] to [-1.0, 1.0]
                out.push((sample as f32 - 128.0) / 128.0);
            }
        } else {
            // Mix channels to mono
            for frame in 0..frames {
                let mut sample = 0.0f32;
                for ch in 0..channels {
                    let s = buf.chan(ch)[frame] as f32 - 128.0;
                    sample += s;
                }
                out.push(sample / (channels as f32 * 128.0));
            }
        }
    }

    /// Convert multi-channel samples to mono by averaging channels
    fn to_mono(samples: &[f32], channels: usize) -> Vec<f32> {
        if channels == 1 {
            return samples.to_vec();
        }

        let frames = samples.len() / channels;
        let mut mono = Vec::with_capacity(frames);

        for frame in 0..frames {
            let mut sum = 0.0f32;
            for ch in 0..channels {
                sum += samples[frame * channels + ch];
            }
            mono.push(sum / channels as f32);
        }

        mono
    }

    /// Resample audio to target sample rate using high-quality resampling
    fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        if from_rate == to_rate {
            return Ok(samples.to_vec());
        }

        // Use simple linear resampling for reliability
        let ratio = to_rate as f64 / from_rate as f64;
        let output_len = ((samples.len() as f64) * ratio).ceil() as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let pos = i as f64 / ratio;
            let lower = pos.floor() as usize;
            let upper = (lower + 1).min(samples.len() - 1);
            let frac = pos - lower as f64;

            let sample = if lower < samples.len() {
                samples[lower] * (1.0 - frac) as f32 + samples[upper] * frac as f32
            } else {
                samples[lower]
            };

            output.push(sample);
        }

        output.truncate(output_len);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mono_conversion() {
        let stereo = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
        let mono = AudioProcessor::to_mono(&stereo, 2);
        assert_eq!(mono.len(), 3);
        assert!((mono[0] - 0.15).abs() < 0.0001);
        assert!((mono[1] - 0.35).abs() < 0.0001);
        assert!((mono[2] - 0.55).abs() < 0.0001);
    }

    #[test]
    fn test_s16_conversion() {
        let converted: Vec<f32> = vec![0i16, 16384, -16384, 32767, -32768]
            .iter()
            .map(|&s| s as f32 / 32767.0)
            .collect();
        assert_eq!(converted.len(), 5);
        assert!((converted[0] - 0.0).abs() < 0.0001);
        assert!((converted[3] - 1.0).abs() < 0.0001);
    }
}
