# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-30

### Added

- **Audio Chunking** - Automatic splitting of long audio files into smaller chunks for reliable transcription
  - New `--chunk-size` option to set chunk duration in minutes (default: 0 = disabled)
  - New `--chunk-overlap` option to set overlap between chunks in seconds (default: 5)
  - Intelligent merging of results with duplicate detection using Jaccard similarity
  - Confidence-based segment selection for overlapping regions

- **Improved M4A Support** - Fixed channel detection for M4A files that don't include channel info in metadata
  - Channels are now detected from the first decoded audio packet as fallback

- **Russian Documentation** - Comprehensive documentation in Russian
  - Updated `doc/README_RU.md` with chunking guide, overlap explanation
  - Build instructions for cross-platform compilation
  - macOS installation guide for global CLI access

### Changed

- Improved audio processing pipeline with better error handling
- Enhanced progress reporting during chunked transcription
- Updated model management with clearer status messages

### Fixed

- Fixed "Channel count unknown" error for certain M4A files
- Fixed duplicate segment detection in overlapping chunk regions

## [0.1.0] - 2025-01-28

### Added

- **Core Transcription** - Full audio-to-text transcription using whisper.cpp
  - Support for 99+ languages with automatic detection
  - Manual language specification with `--language` option
  - Translation to English with `--translate` flag

- **Audio Format Support**
  - MP3, WAV, FLAC, M4A, OGG, WMA input formats
  - Automatic resampling to 16kHz mono (whisper.cpp requirement)
  - Audio metadata extraction and display

- **Output Formats**
  - JSON with full segment details and timestamps
  - SRT (SubRip) for video subtitles
  - VTT (WebVTT) for web video subtitles
  - TXT plain text output
  - TSV tab-separated values for spreadsheets

- **Model Management**
  - `model list` - Show available Whisper models
  - `model download <model>` - Download models (tiny, base, small, medium, large)
  - `model status` - Check installed models
  - Automatic model caching in `~/.cache/orangenote/models/`

- **CLI Interface**
  - Built with clap for robust argument parsing
  - Configurable thread count with `--threads`
  - Verbose logging with `--verbose` and `--log-level`
  - Output to file with `--output` or stdout

- **Performance**
  - Metal GPU acceleration on Apple Silicon
  - Multi-threaded CPU processing
  - Efficient memory usage with streaming decode

### Technical

- Built with Rust for performance and reliability
- whisper.cpp integration via FFI bindings
- Symphonia for audio decoding
- Async runtime with Tokio

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.2.0 | 2025-01-30 | Audio chunking, M4A fix, Russian docs |
| 0.1.0 | 2025-01-28 | Initial release with full transcription |

[0.2.0]: https://github.com/rodnoy/orangenote-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rodnoy/orangenote-cli/releases/tag/v0.1.0