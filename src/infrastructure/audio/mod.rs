//! Audio processing module
//!
//! Handles audio file reading, metadata extraction, and format detection.
//! Supports MP3, WAV, FLAC, M4A, OGG formats.

pub mod decoder;

pub use decoder::{AudioDecoder, AudioFormat, AudioMetadata};
