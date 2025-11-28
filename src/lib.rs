//! OrangeNote CLI Library
//!
//! Core library for audio transcription using whisper.cpp.
//! Provides audio decoding, metadata extraction, and transcription pipeline.

pub mod infrastructure;

pub use infrastructure::audio::{AudioDecoder, AudioFormat, AudioMetadata};

#[cfg(feature = "whisper")]
pub use infrastructure::{
    ModelSize, ModelSource, Segment, Token, TranscriptionResult, WhisperContextWrapper,
    WhisperModelManager,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Represents the result of operations in this library
pub type Result<T> = std::result::Result<T, anyhow::Error>;
