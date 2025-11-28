//! Whisper.cpp transcription backend
//!
//! This module provides integration with whisper.cpp for audio transcription.
//! It includes FFI bindings, safe wrappers, and high-level transcription APIs.
//!
//! # Feature Gate
//!
//! This module is only available when the `whisper` feature is enabled:
//!
//! ```toml
//! [features]
//! whisper = ["whisper-rs"]
//! ```

#[cfg(feature = "whisper")]
pub mod ffi;

#[cfg(feature = "whisper")]
pub mod context;

#[cfg(feature = "whisper")]
pub use context::{Segment, Token, TranscriptionResult, WhisperContextWrapper};
