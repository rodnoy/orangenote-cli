# Step A3: Whisper.cpp Integration (Preparation)

**Objective**: Integrate whisper.cpp as a transcription backend and prepare FFI bindings for audio transcription.

**Status**: ✅ Complete (Preparation Phase)

---

## Overview

Step A3 focuses on setting up the infrastructure for whisper.cpp integration. This includes:

1. **FFI Bindings** - Low-level C bindings to whisper.cpp
2. **Safe Rust Wrapper** - High-level, idiomatic Rust API
3. **Build Configuration** - Compilation setup with cmake
4. **Feature Gating** - Optional feature flag for whisper support

The project now compiles with whisper support enabled, but the transcription pipeline isn't yet integrated into the CLI commands.

---

## What Was Added

### 1. Dependencies (`Cargo.toml`)

```toml
[dependencies]
whisper-rs = { version = "0.1", optional = true }

[features]
default = []
whisper = ["whisper-rs"]
```

- **whisper-rs**: Rust wrapper for whisper.cpp (optional)
- **Feature Flag**: `whisper` feature gates all whisper functionality

### 2. FFI Bindings (`src/infrastructure/transcription/whisper/ffi.rs`)

Low-level C bindings to whisper.cpp library:

- **WhisperContext**: Opaque pointer to whisper model context
- **WhisperFullParams**: Configuration for transcription pipeline
- **WhisperSegment**: Result data for a transcribed segment
- **FFI Functions**:
  - `whisper_init_from_file()` - Load model from file
  - `whisper_init_from_buffer()` - Load model from memory
  - `whisper_full()` - Run full transcription pipeline
  - `whisper_full_n_segments()` - Get number of segments
  - `whisper_full_get_segment_*()` - Extract segment data
  - `whisper_lang_str()` - Get language name

### 3. Safe Wrapper (`src/infrastructure/transcription/whisper/context.rs`)

High-level Rust API around FFI bindings:

**WhisperContextWrapper** - Main struct
- `new(model_path)` - Initialize from model file
- `from_buffer(buffer)` - Initialize from memory buffer
- `transcribe(samples, language, translate)` - Run transcription

**Segment** - Transcribed segment data
- `id` - Segment index
- `start_ms` / `end_ms` - Timing information
- `text` - Transcribed text
- `confidence` - Confidence score (0.0-1.0)
- `tokens` - Individual tokens with probabilities
- `start_time_formatted()` / `end_time_formatted()` - HH:MM:SS.mmm format

**TranscriptionResult** - Complete result
- `language` - Detected language
- `segments` - List of segments
- `full_text()` - Concatenated transcript
- `average_confidence()` - Mean confidence score

**Token** - Individual token data
- `text` - Token string
- `probability` - Token probability

### 4. Module Organization

```
src/infrastructure/
├── audio/                    (existing)
│   ├── mod.rs
│   └── decoder.rs
└── transcription/            (new)
    ├── mod.rs
    └── whisper/             (new)
        ├── mod.rs
        ├── ffi.rs           (FFI bindings)
        └── context.rs       (Safe wrapper)
```

### 5. Build Script (`build.rs`)

Handles compilation of whisper.cpp:

- **CMake Integration**: Uses cmake to build whisper.cpp
- **Conditional Compilation**: Only runs when `whisper` feature is enabled
- **Platform Support**: Links to system libraries on macOS, Linux, Windows
- **Fallback Handling**: Graceful degradation if whisper.cpp not found

```bash
cmake -DCMAKE_BUILD_TYPE=Release \
      -DBUILD_SHARED_LIBS=OFF \
      -DWHISPER_CPP_ONLY=ON \
      vendor/whisper.cpp
```

---

## Getting whisper.cpp

### Option 1: Git Submodule (Recommended)

```bash
cd orangenote-cli
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive
```

### Option 2: Manual Download

```bash
mkdir -p vendor
cd vendor
git clone https://github.com/ggerganov/whisper.cpp
cd ..
```

---

## Building with Whisper Support

### Compile without whisper (default)

```bash
cargo build
```

### Compile with whisper support

```bash
cargo build --features whisper
```

### Release build with optimizations

```bash
cargo build --release --features whisper
```

---

## Architecture Diagram

```
Audio Input (WAV/MP3/etc)
    ↓
[AudioDecoder] (Step A2)
    ↓
PCM Samples (float32)
    ↓
[WhisperContextWrapper]
    ↓
FFI Layer [ffi.rs]
    ↓
whisper.cpp (C++)
    ↓
Transcription Results
    ↓
[Segment + Token Data]
```

---

## Key Design Decisions

### 1. Feature Gating
Whisper support is optional to:
- Keep dependencies minimal
- Allow other transcription backends in the future
- Enable compilation on systems without C++ toolchain

### 2. Safe Wrapper
The `WhisperContextWrapper` provides:
- **Memory Safety**: Automatic cleanup with `Drop` trait
- **Error Handling**: `Result` types instead of null pointers
- **Idiomatic Rust**: Owned types, no unsafe code in public API

### 3. FFI Design
Low-level bindings in `ffi.rs`:
- Use `#[link(name = "whisper")]` for static linking
- Define all C types and functions
- Marked as `unsafe` to indicate direct C interaction

### 4. Build Integration
`build.rs` automates:
- CMake configuration
- Library compilation
- Link path setup
- Platform-specific linking

---

## Current Limitations

1. **Not Yet Integrated**: Transcription not connected to CLI commands
2. **Model Management**: No automatic model downloading
3. **Resampling**: Audio resampling to 16kHz not yet implemented
4. **No Streaming**: Full audio must be in memory
5. **Single Language**: Per-file language, not segment-level switching

---

## Next Steps (Step A4+)

### Step A4: Audio Processing Pipeline
- [ ] Add audio resampling to 16kHz
- [ ] Implement mono conversion
- [ ] Add audio normalization
- [ ] Handle various sample rates and formats

### Step A5: Output Formatting
- [ ] JSON output format
- [ ] SRT (SubRip) format
- [ ] VTT (WebVTT) format
- [ ] Plain text output

### Step A6: CLI Integration
- [ ] Connect transcription to `transcribe` command
- [ ] Display results with progress tracking
- [ ] Add model management (`model download`, `model list`)

### Step A7: Testing & Optimization
- [ ] Integration tests with real audio
- [ ] Performance profiling
- [ ] Error handling improvements
- [ ] Documentation of supported models

---

## Compilation Testing

### Test compilation without features
```bash
cargo check
cargo build
```

### Test compilation with whisper feature
```bash
cargo check --features whisper
cargo build --features whisper
```

### Run tests
```bash
cargo test
cargo test --features whisper
```

---

## Type Safety and Memory Management

### Ownership Model
- `WhisperContextWrapper` owns the FFI context
- Automatic cleanup on `Drop`
- No manual memory management needed in Rust code

### Error Handling
```rust
pub fn transcribe(
    &self,
    samples: &[c_float],
    language: Option<&str>,
    translate: bool,
) -> Result<TranscriptionResult>
```

All operations return `anyhow::Result` for ergonomic error handling.

### Zero-Copy Where Possible
- Sample data passed by reference
- CString created only for language parameter
- Results allocated on heap as owned Rust types

---

## FFI Safety Considerations

### Safe Wrappers
- CString conversion for null-terminated strings
- Null pointer checks after FFI calls
- Pointer dereference guarded in unsafe blocks

### Unsafe Code
- Limited to FFI calls in `ffi.rs` and `context.rs`
- All unsafe code has clear safety contracts
- No unsafe code exposed in public API

---

## Integration Points

### From AudioDecoder (Step A2)
```rust
let decoder = AudioDecoder::new("audio.wav")?;
let samples = decoder.decode_to_samples()?;  // → Vec<f32>
```

### To Output Formatting (Step A5)
```rust
let result = context.transcribe(&samples, Some("en"), false)?;
let json = format_as_json(&result)?;
let srt = format_as_srt(&result)?;
```

---

## Readiness Criteria ✅

- [x] FFI bindings created and documented
- [x] Safe wrapper implemented with proper error handling
- [x] Build script configured for cmake compilation
- [x] Feature gating implemented
- [x] Project compiles with `--features whisper`
- [x] Module structure organized and exported
- [x] Tests for timestamp formatting pass

---

## References

- [whisper.cpp Repository](https://github.com/ggerganov/whisper.cpp)
- [whisper.cpp API Documentation](https://github.com/ggerganov/whisper.cpp#api)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Previous Step A2 - Audio Decoding](STEP_A2.md)

---

## Files Modified/Created

### New Files
- `src/infrastructure/transcription/mod.rs`
- `src/infrastructure/transcription/whisper/mod.rs`
- `src/infrastructure/transcription/whisper/ffi.rs`
- `src/infrastructure/transcription/whisper/context.rs`
- `build.rs`
- `STEP_A3.md` (this file)

### Modified Files
- `Cargo.toml` - Added whisper-rs dependency and feature flag
- `src/lib.rs` - Exported transcription types
- `src/infrastructure/mod.rs` - Added transcription module

---

## Quick Start for Next Developer

1. Clone whisper.cpp as submodule:
   ```bash
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   ```

2. Build with whisper support:
   ```bash
   cargo build --features whisper
   ```

3. To use in code:
   ```rust
   use orangenote_cli::WhisperContextWrapper;
   
   let ctx = WhisperContextWrapper::new("models/ggml-base.bin")?;
   let result = ctx.transcribe(&samples, Some("en"), false)?;
   
   for segment in &result.segments {
       println!("{} - {}: {}", 
           segment.start_time_formatted(),
           segment.end_time_formatted(),
           segment.text
       );
   }
   ```

---

**Status**: Ready for Step A4 (Audio Processing Pipeline)

**Completion Date**: [Current Date]

**Notes**: 
- FFI design allows for future transcription backends
- Safe wrapper ensures memory safety and ergonomic API
- Build script handles platform-specific compilation
- Feature gate keeps whisper optional for flexibility