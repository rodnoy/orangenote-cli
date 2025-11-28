# OrangeNote CLI - Development Roadmap

## Overview

This document outlines the complete development path for **Variant A: Quick CLI Prototype**, breaking down the implementation into atomic, independently testable steps.

## Phase A1: ✅ COMPLETED - Basic CLI Structure

**Status:** Done  
**Completion Date:** 2024-11-28

### What was accomplished:
- CLI argument parsing with `clap`
- Command structure (transcribe, model, info)
- Input validation and error handling
- Logging infrastructure
- Documentation (English + Russian)

### Test command:
```bash
cargo run --bin orangenote-cli -- --help
cargo run --bin orangenote-cli -- transcribe test.mp3 -l ru -m small
```

**Success Criteria:** ✅ All met
- CLI runs and parses arguments
- Validates files and shows errors
- Outputs help information

---

## Phase A2: Backend Integration - Audio Processing

**Estimated Timeline:** 1-2 weeks  
**Dependencies:** A1 (completed)

### A2.1: Add Whisper Backend Dependency

**Goal:** Integrate whisper.cpp via Rust FFI

**Tasks:**
- [ ] Research whisper.cpp Rust bindings (whisper-rs crate)
- [ ] Add dependency to `Cargo.toml`
- [ ] Create `src/lib.rs` with backend module structure
- [ ] Create basic wrapper around whisper initialization

**Test:**
```bash
cargo build
# Should compile without errors
```

**Success Criteria:**
- Whisper library loads without panics
- Can initialize with a model path
- Error handling works properly

---

### A2.2: Implement Model Loading

**Goal:** Load Whisper models from disk

**Tasks:**
- [ ] Create models directory structure
- [ ] Implement model path resolution
- [ ] Add model validation (check file format)
- [ ] Integrate into CLI: `transcribe` command loads model

**Test:**
```bash
# Assume base.bin exists in models/
cargo run --bin orangenote-cli -- transcribe test.mp3 -m base
# Should log: "Loading model: base.bin"
```

**Success Criteria:**
- Model files are found correctly
- Invalid models produce clear errors
- Model path can be configured via env var

---

### A2.3: Implement Audio Loading

**Goal:** Load and prepare audio for transcription

**Tasks:**
- [ ] Add audio decoding dependency (ffmpeg or similar)
- [ ] Implement audio format detection
- [ ] Convert audio to required format (WAV/PCM)
- [ ] Extract audio metadata (duration, channels, sample rate)

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe test.mp3 -m base
# Should log: "Audio loaded: 120.5 seconds, 16kHz, mono"
```

**Success Criteria:**
- Supports MP3, WAV, FLAC, M4A formats
- Resamples to 16kHz if needed
- Duration extracted and logged

---

### A2.4: Implement Basic Transcription

**Goal:** Perform actual speech-to-text transcription

**Tasks:**
- [ ] Call whisper.cpp to transcribe audio
- [ ] Capture transcription output
- [ ] Measure transcription time
- [ ] Handle transcription errors

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe test.mp3 -m base
# Output:
# ✓ Transcription complete!
# Duration: 2m 30s
# Text: [full transcription]
```

**Success Criteria:**
- Produces text output
- Shows timing information
- Handles errors gracefully
- Works with different models (tiny, base, small)

---

## Phase A3: Output Formatting

**Estimated Timeline:** 3-5 days  
**Dependencies:** A2.4 (basic transcription)

### A3.1: JSON Output Format

**Goal:** Export results as structured JSON

**Tasks:**
- [ ] Define JSON schema with timestamps
- [ ] Parse whisper segments
- [ ] Add confidence scores if available
- [ ] Pretty-print JSON output

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe test.mp3 -f json -o output.json
cat output.json
# Should contain: text, language, segments with timings
```

**Success Criteria:**
- Valid JSON structure
- Timestamps in ISO 8601 format
- Includes language detection result
- File saving works

---

### A3.2: SRT Subtitle Format

**Goal:** Create subtitle files for video players

**Tasks:**
- [ ] Implement SRT format writer
- [ ] Convert timestamps to SRT format
- [ ] Split long segments appropriately
- [ ] Handle special characters encoding

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe video.mp4 -f srt -o subs.srt
# Should produce valid SRT file readable by VLC/ffmpeg
```

**Success Criteria:**
- Valid SRT syntax
- Correct timecode formatting (HH:MM:SS,mmm)
- Sequential numbering
- Compatible with major video players

---

### A3.3: VTT and Plain Text Formats

**Goal:** Support additional output formats

**Tasks:**
- [ ] Implement WebVTT (Web Video Text) format
- [ ] Implement plain text output (no timestamps)
- [ ] TSV format for spreadsheet import

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe audio.mp3 -f vtt -o output.vtt
cargo run --bin orangenote-cli -- transcribe audio.mp3 -f txt -o output.txt
cargo run --bin orangenote-cli -- transcribe audio.mp3 -f tsv -o output.tsv
```

**Success Criteria:**
- All formats produce valid output
- Formats are correct for their type
- Spreadsheets can import TSV

---

## Phase A4: Model Management

**Estimated Timeline:** 1 week  
**Dependencies:** A2.1 (backend structure)

### A4.1: Model Discovery and Listing

**Goal:** Know what models are available and installed

**Tasks:**
- [ ] Define models directory structure
- [ ] Scan for installed models
- [ ] Fetch available models from OpenAI/Hugging Face
- [ ] Display models with metadata (size, accuracy, speed)

**Test:**
```bash
cargo run --bin orangenote-cli -- model list
# Output:
# Available models:
#   ✓ tiny   (installed, 39M)
#   ✗ base   (available, 140M)
#   ✗ small  (available, 466M)
```

**Success Criteria:**
- Shows installed and available models
- Displays accurate file sizes
- Can distinguish local vs remote

---

### A4.2: Model Downloading

**Goal:** Download models automatically

**Tasks:**
- [ ] Implement HTTP download with progress bar
- [ ] Validate downloaded files (checksums)
- [ ] Store in proper directory
- [ ] Handle network errors and retries

**Test:**
```bash
cargo run --bin orangenote-cli -- model download base
# Output:
# Downloading base model...
# ████████████████████ 100%  140M/140M
# ✓ Model installed successfully
```

**Success Criteria:**
- Downloads complete successfully
- Shows progress
- Validates file integrity
- Resumes partial downloads
- Works offline after download

---

### A4.3: Model Management Commands

**Goal:** Remove and update models

**Tasks:**
- [ ] Implement model removal with confirmation
- [ ] Implement model update checking
- [ ] Show model cache statistics (used space)
- [ ] Allow cache cleanup

**Test:**
```bash
cargo run --bin orangenote-cli -- model status
# Output:
# Installed models: 2
# Total size: 600M
# Available space: 50G

cargo run --bin orangenote-cli -- model remove base
# Confirm? (y/n)
```

**Success Criteria:**
- Clean removal without artifacts
- Safe deletion (requires confirmation)
- Accurate size calculations
- Shows cache statistics

---

## Phase A5: Performance and Language Support

**Estimated Timeline:** 1 week  
**Dependencies:** A2.4 (basic transcription)

### A5.1: Multi-threaded Processing

**Goal:** Improve transcription speed with parallelization

**Tasks:**
- [ ] Implement thread pool for parallel processing
- [ ] Allow configurable thread count
- [ ] Benchmark single vs multi-threaded
- [ ] Optimize for different CPU counts

**Test:**
```bash
time cargo run --bin orangenote-cli -- transcribe audio.mp3 -t 1
time cargo run --bin orangenote-cli -- transcribe audio.mp3 -t 8
# Second should be faster
```

**Success Criteria:**
- Scales with thread count
- Default thread count matches CPU cores
- Shows improvement on multi-core systems
- No performance regression on single-core

---

### A5.2: Language Auto-detection

**Goal:** Automatically identify the language being spoken

**Tasks:**
- [ ] Implement language detection from audio
- [ ] Support manual language override
- [ ] Cache language detection result
- [ ] Display detected language in output

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe russian_audio.mp3
# Output includes: Detected language: Russian (ru)

cargo run --bin orangenote-cli -- transcribe russian_audio.mp3 -l en
# Override: Uses English model
```

**Success Criteria:**
- Auto-detection works for common languages
- Manual override works correctly
- Shows confidence in detection
- Supports 50+ languages

---

### A5.3: Language-Specific Features

**Goal:** Handle language-specific transcription needs

**Tasks:**
- [ ] Support language-specific punctuation
- [ ] Implement script-specific processing (Cyrillic, CJK, etc.)
- [ ] Handle mixed-language audio
- [ ] Time zone handling for timestamps

**Test:**
```bash
cargo run --bin orangenote-cli -- transcribe multilingual.mp3
# Properly segments by language
```

**Success Criteria:**
- Cyrillic characters preserved
- Chinese/Japanese characters work
- Punctuation appropriate per language
- Time format respects language preference

---

## Phase A6: Testing and Documentation

**Estimated Timeline:** 5 days  
**Dependencies:** A1-A5 (all previous phases)

### A6.1: Unit Tests

**Goal:** Test individual components

**Tasks:**
- [ ] Test file validation logic
- [ ] Test format conversion functions
- [ ] Test error handling paths
- [ ] Aim for 70%+ code coverage

**Test:**
```bash
cargo test
# All tests pass
```

---

### A6.2: Integration Tests

**Goal:** Test end-to-end workflows

**Tasks:**
- [ ] Test transcription pipeline with sample audio
- [ ] Test all output formats
- [ ] Test model management workflows
- [ ] Test error recovery

**Test:**
```bash
cargo test --test integration_tests
```

---

### A6.3: Documentation Updates

**Goal:** Keep documentation current

**Tasks:**
- [ ] Update README with new features
- [ ] Add usage examples for each format
- [ ] Document model management
- [ ] Create troubleshooting guide
- [ ] Add performance tips

**Success Criteria:**
- All commands documented
- Examples for common use cases
- Performance benchmarks included
- FAQ covers common issues

---

## Phase A7: Performance Optimization

**Estimated Timeline:** 1 week  
**Dependencies:** A6 (testing complete)

### A7.1: Profiling and Optimization

**Goal:** Improve speed and reduce memory usage

**Tasks:**
- [ ] Profile memory usage
- [ ] Identify performance bottlenecks
- [ ] Optimize hot paths
- [ ] Reduce allocations

**Benchmarks:**
```bash
# Before:  30s for 1 hour audio (base model)
# After:  15s for 1 hour audio (base model)
```

---

### A7.2: Model Quantization (Optional)

**Goal:** Support smaller, faster models

**Tasks:**
- [ ] Investigate quantized model support
- [ ] Implement loading of quantized models
- [ ] Benchmark speed/accuracy tradeoff
- [ ] Document recommendations

---

## Phase A8: Polish and Release

**Estimated Timeline:** 3-5 days  
**Dependencies:** A7 (optimization complete)

### A8.1: Error Messages and Help

**Goal:** Excellent user experience

**Tasks:**
- [ ] Review all error messages
- [ ] Add helpful suggestions
- [ ] Implement shell completion
- [ ] Create cheat sheet

**Test:**
```bash
# Error messages should be clear
cargo run --bin orangenote-cli -- transcribe
# Error: Missing required argument <INPUT>
# Try: orangenote-cli transcribe --help
```

---

### A8.2: Release Preparation

**Goal:** Production-ready binary

**Tasks:**
- [ ] Clean up code and comments
- [ ] Verify all features work
- [ ] Create release checklist
- [ ] Build and test release binary

**Test:**
```bash
cargo build --release
./target/release/orangenote-cli --version
```

---

### A8.3: First Release (v0.1.0)

**Goal:** Publish stable release

**Tasks:**
- [ ] Tag version in git
- [ ] Create release notes
- [ ] Build binaries for macOS/Linux/Windows
- [ ] Publish to crates.io (optional)

---

## Timeline Summary

| Phase | Description | Duration | Dependencies |
|-------|-------------|----------|--------------|
| A1 | Basic CLI Structure | ✅ Done | - |
| A2 | Backend Integration | 1-2 weeks | A1 |
| A3 | Output Formatting | 3-5 days | A2.4 |
| A4 | Model Management | 1 week | A2.1 |
| A5 | Performance & Language | 1 week | A2.4 |
| A6 | Testing & Docs | 5 days | A1-A5 |
| A7 | Optimization | 1 week | A6 |
| A8 | Polish & Release | 3-5 days | A7 |

**Total Estimated Time:** 5-7 weeks for MVP (v0.1.0)

---

## Success Metrics

- [ ] CLI compiles without warnings
- [ ] All commands work correctly
- [ ] Transcription accuracy >95% (matching OpenAI Whisper)
- [ ] Processing speed: 1 hour audio in <5 minutes (base model, CPU)
- [ ] Memory usage: <2GB for base model
- [ ] Support for 50+ languages
- [ ] All 5 output formats working
- [ ] Model management fully functional
- [ ] Documentation complete (ENG + RUS)
- [ ] Test coverage >70%

---

## Notes for Developers

### Key Design Decisions

1. **Async/Await:** Use tokio for async operations to handle multiple transcriptions
2. **Error Handling:** Use anyhow for flexible error handling with context
3. **Logging:** Use env_logger for configurable logging in all phases
4. **Modularity:** Keep components separated for easy testing

### Dependencies to Add

**Phase A2:**
- `whisper-rs` or similar Whisper binding
- Audio processing library (ffmpeg-rust or similar)

**Phase A4:**
- HTTP client (reqwest)
- Progress bar (indicatif)
- SHA256 for checksums

**Phase A5:**
- Rayon for thread pool (if needed)

### Testing Strategy

- Write tests as you implement features
- Use fixture audio files for consistent testing
- Test error paths, not just happy paths
- Benchmark performance at each phase

---

## Integration with Variant B

This CLI work directly supports Variant B (Desktop UI):

1. **Phase A1-A3 work** can be used as the backend for Tauri UI
2. **Model management (A4)** works unchanged in UI
3. **Output formatting (A3)** provides structured data for UI display
4. **Testing (A6)** ensures reliability for UI dependencies

The CLI remains a valuable standalone tool even after UI is built.

---

**Document Version:** 1.0  
**Last Updated:** 2024-11-28  
**Author:** OrangeNote Team