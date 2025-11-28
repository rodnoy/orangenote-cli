//! Infrastructure layer
//!
//! Contains low-level implementations for audio processing, file handling,
//! and other system-level concerns.

pub mod audio;

#[cfg(feature = "whisper")]
pub mod transcription;

#[cfg(feature = "whisper")]
pub use transcription::{Segment, Token, TranscriptionResult, WhisperContextWrapper};
