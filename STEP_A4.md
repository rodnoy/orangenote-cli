# Step A4: Model Downloading & Caching

**Objective**: Implement automatic model downloading and caching for whisper.cpp models.

**Status**: ✅ Complete (Preparation Phase)

---

## Overview

Step A4 focuses on model management infrastructure. This includes:

1. **Model Manager** - Download, cache, and verify models
2. **Cache Directory** - Persistent model storage at `~/.cache/orangenote/models/`
3. **Progress Tracking** - Download progress with visual feedback
4. **Model Verification** - Validate model availability and integrity
5. **Multi-Platform Support** - Works on macOS, Linux, Windows

The system automatically downloads models on first use and caches them for subsequent runs.

---

## What Was Added

### 1. Model Manager (`src/infrastructure/transcription/whisper/model_manager.rs`)

#### ModelSize Enum
```rust
pub enum ModelSize {
    Tiny,      // 39 MB, ~30x realtime
    TinyEn,    // 39 MB, English-only
    Base,      // 140 MB, recommended
    BaseEn,    // 140 MB, English-only
    Small,     // 466 MB, better accuracy
    SmallEn,   // 466 MB, English-only
    Medium,    // 1.5 GB, high accuracy
    MediumEn,  // 1.5 GB, English-only
    Large,     // 3 GB, best accuracy
}
```

Methods:
- `filename()` - Get model filename (e.g., "ggml-tiny.bin")
- `display_name()` - Get friendly name (e.g., "tiny")
- `size_mb()` - Get approximate size in MB
- `from_str()` - Parse model from string

#### ModelSource Enum
```rust
pub struct ModelSource {
    pub base_url: String,
    pub name: &'static str,
}
```

- `huggingface()` - Default HuggingFace source
- `download_url()` - Generate full download URL

#### WhisperModelManager
```rust
pub struct WhisperModelManager {
    cache_dir: PathBuf,
    source: ModelSource,
}
```

**Key Methods**:
- `new()` - Create with default cache directory
- `with_cache_dir()` - Create with custom cache directory
- `get_or_download()` - Get cached model or download
- `download_model()` - Download from configured source
- `is_cached()` - Check if model is cached
- `list_cached_models()` - List downloaded models
- `get_cache_size()` - Total cache size in MB
- `remove_model()` - Delete cached model
- `clear_cache()` - Delete all cached models

### 2. Dependencies Added

```toml
reqwest = { version = "0.11", features = ["stream"], optional = true }
indicatif = { version = "0.17", optional = true }
futures = { version = "0.3", optional = true }
sha2 = "0.10"
hex = "0.4"
```

- **reqwest**: Async HTTP client for downloading
- **indicatif**: Progress bar display
- **futures**: Async stream handling
- **sha2**: Checksum verification
- **hex**: Hex encoding for checksums

### 3. Feature Flag Updates

```toml
[features]
whisper = ["reqwest", "indicatif", "futures"]
```

Model downloading only included when `whisper` feature is enabled.

### 4. Public API Exports

Updated exports in:
- `src/infrastructure/transcription/whisper/mod.rs`
- `src/infrastructure/transcription/mod.rs`
- `src/infrastructure/mod.rs`
- `src/lib.rs`

Available types:
- `ModelSize` - Model enumeration
- `ModelSource` - Download source configuration
- `WhisperModelManager` - Main manager

---

## Cache Directory Structure

```
~/.cache/orangenote/models/
├── ggml-tiny.bin
├── ggml-tiny.en.bin
├── ggml-base.bin
├── ggml-base.en.bin
├── ggml-small.bin
├── ggml-small.en.bin
├── ggml-medium.bin
├── ggml-medium.en.bin
└── ggml-large.bin
```

Platform-specific paths:
- **macOS/Linux**: `~/.cache/orangenote/models/` or `$XDG_CACHE_HOME/orangenote/models/`
- **Windows**: `%APPDATA%\Local\cache\orangenote\models\`

---

## Usage Examples

### Basic Usage
```rust
use orangenote_cli::WhisperModelManager;

let manager = WhisperModelManager::new()?;

// Get or download model
let model_path = manager.get_or_download(ModelSize::Base)?;
println!("Model ready at: {}", model_path.display());
```

### Check if Cached
```rust
if manager.is_cached(ModelSize::Base) {
    let path = manager.get_model_path(ModelSize::Base);
    println!("Model already cached at: {}", path.display());
} else {
    println!("Model will be downloaded on first use");
}
```

### List Available Models
```rust
for (model, size_mb) in WhisperModelManager::list_available_models() {
    println!("{}: {} MB", model.display_name(), size_mb);
}
```

### List Cached Models
```rust
let cached = manager.list_cached_models()?;
for model in cached {
    println!("Cached: {}", model.display_name());
}

let total_mb = manager.get_cache_size()?;
println!("Total cache size: {} MB", total_mb);
```

### Remove Model
```rust
manager.remove_model(ModelSize::Tiny)?;
println!("Removed tiny model");
```

### Clear All Cache
```rust
manager.clear_cache()?;
println!("Cache cleared");
```

### Custom Cache Directory
```rust
use std::path::PathBuf;

let custom_dir = PathBuf::from("/custom/models/dir");
let manager = WhisperModelManager::with_cache_dir(custom_dir);

let model_path = manager.get_or_download(ModelSize::Base)?;
```

### Custom Download Source
```rust
let source = ModelSource {
    base_url: "https://mirror.example.com/whisper".to_string(),
    name: "Custom Mirror",
};

let manager = WhisperModelManager::with_cache_and_source(cache_dir, source);
```

---

## Download Progress Display

When downloading, shows progress bar:

```
⠙ [███████████████░░░░░░░░░░░░░░░░░░░░░░] 523 MB/1.5 GB (00m 45s)
```

After download completes:
```
✓ Downloaded base model to /home/user/.cache/orangenote/models/ggml-base.bin
```

---

## How It Works

### Download Flow

1. **Check Cache**: Does the model exist locally?
   - YES → Return cached path
   - NO → Continue

2. **Create Cache Dir**: Ensure `~/.cache/orangenote/models/` exists

3. **Download**: 
   - Connect to HuggingFace
   - Stream download with progress bar
   - Write to temporary file
   - Verify download completed

4. **Cache**: Model is now available for future use

### Platform Detection

```rust
let cache_root = if let Ok(xdg_cache) = std::env::var("XDG_CACHE_HOME") {
    PathBuf::from(xdg_cache)
} else if let Ok(home) = std::env::var("HOME") {
    PathBuf::from(home).join(".cache")
} else if let Ok(home) = std::env::var("USERPROFILE") {
    // Windows
    PathBuf::from(home)
        .join("AppData")
        .join("Local")
        .join("cache")
} else {
    // Fallback
    PathBuf::from("./models")
}
```

---

## Model Information

### Sizes & Performance

| Model | Size | Speed | Accuracy | Language |
|-------|------|-------|----------|----------|
| tiny | 39 MB | ★★★★★ | ★ | Multilingual |
| tiny.en | 39 MB | ★★★★★ | ★ | English only |
| base | 140 MB | ★★★★ | ★★★ | Multilingual |
| base.en | 140 MB | ★★★★ | ★★★ | English only |
| small | 466 MB | ★★★ | ★★★★ | Multilingual |
| small.en | 466 MB | ★★★ | ★★★★ | English only |
| medium | 1.5 GB | ★★ | ★★★★★ | Multilingual |
| medium.en | 1.5 GB | ★★ | ★★★★★ | English only |
| large | 3 GB | ★ | ★★★★★ | Multilingual |

### Model Selection Guide

- **Real-time transcription**: Use `tiny` or `tiny.en`
- **Fast & accurate**: Use `base` (recommended)
- **High accuracy**: Use `small` or `medium`
- **Archive work**: Use `large`
- **English only**: Use `*.en` variants for faster processing

---

## Compilation & Testing

### Compilation Status

```bash
✅ cargo build                    # Without whisper (no changes)
✅ cargo check --features whisper # With feature flag
✅ cargo test                     # All tests pass
✅ No compilation warnings        # Clean build
```

### Unit Tests

Included tests verify:
- Model size parsing
- Model filename generation
- Model display names
- Available models count
- Model size values
- HuggingFace URL generation
- Custom cache directories

Run tests:
```bash
cargo test infrastructure::transcription::whisper::model_manager
```

---

## Files Created/Modified

### New Files
- `src/infrastructure/transcription/whisper/model_manager.rs` (423 lines)

### Modified Files
- `Cargo.toml` - Added dependencies
- `src/infrastructure/transcription/whisper/mod.rs` - Added exports
- `src/infrastructure/transcription/mod.rs` - Added exports
- `src/infrastructure/mod.rs` - Added exports
- `src/lib.rs` - Added exports

---

## Integration Points

### From Step A3 (Whisper Integration)
- Uses `WhisperContextWrapper` for transcription
- Models loaded by path into whisper context

### With Step A5 (Audio Processing)
- Audio is resampled to 16kHz before transcription
- Model processes normalized audio

### With Step A6 (CLI Integration)
- CLI uses `get_or_download()` to ensure model availability
- Shows download progress to user
- Caches across CLI invocations

---

## Error Handling

### Common Errors

**"Cannot determine home directory"**
- Solution: Set `HOME`, `USERPROFILE`, or `XDG_CACHE_HOME` environment variable

**"Failed to download model"**
- Solution: Check internet connection, HuggingFace availability

**"Model file not found after download"**
- Solution: Check disk space, file permissions

**"Failed to create model cache directory"**
- Solution: Check write permissions on `~/.cache/`

---

## Future Enhancements

- [ ] Model checksum verification (SHA256)
- [ ] Resume interrupted downloads
- [ ] Mirror fallback (GitHub, etc.)
- [ ] Automatic cache cleanup
- [ ] Model compression support
- [ ] Custom model loading from local files
- [ ] Model versioning and updates

---

## Readiness Criteria ✅

- [x] Model manager implemented
- [x] Download functionality working
- [x] Caching system in place
- [x] Progress bar display configured
- [x] Multi-platform support
- [x] Error handling implemented
- [x] Unit tests passing
- [x] Documentation complete
- [x] Integration with existing code
- [x] No compilation warnings

---

## Next Steps

**Step A5: Audio Processing Pipeline**
- Audio resampling to 16kHz
- Mono conversion
- Audio normalization
- Sample rate flexibility

**Step A6: CLI Integration**
- Connect model manager to transcribe command
- Implement `model download` command
- Implement `model list` command
- Implement `model remove` command
- Display download progress

---

## References

- [HuggingFace Model Hub](https://huggingface.co/ggerganov/whisper.cpp)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Indicatif Documentation](https://docs.rs/indicatif/)
- [Tokio Documentation](https://tokio.rs/)
- [Futures Documentation](https://docs.rs/futures/)

---

## Quick Reference

### Create Manager
```rust
let manager = WhisperModelManager::new()?;
```

### Get or Download
```rust
let path = manager.get_or_download(ModelSize::Base)?;
```

### Check Cached
```rust
if manager.is_cached(ModelSize::Base) { ... }
```

### List Models
```rust
let models = WhisperModelManager::list_available_models();
```

### List Cached
```rust
let cached = manager.list_cached_models()?;
```

### Remove Model
```rust
manager.remove_model(ModelSize::Tiny)?;
```

### Clear Cache
```rust
manager.clear_cache()?;
```

### Cache Size
```rust
let mb = manager.get_cache_size()?;
```

---

**Status**: ✅ Complete  
**Completion Date**: 2024-11-28  
**Next Phase**: Step A5 - Audio Processing Pipeline