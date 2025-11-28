# Step A5 Quick Reference

## Build
```bash
# Library only
cargo build --lib

# CLI only (without whisper)
cargo build

# CLI with whisper support
cargo build --features whisper

# Release build
cargo build --features whisper --release
```

## Test
```bash
# All library tests
cargo test --lib

# Audio processor tests
cargo test --lib audio::processor

# Run without building
cargo test --lib -- --nocapture
```

## Usage

### Basic Transcription
```bash
orangenote-cli transcribe input.mp3
orangenote-cli transcribe input.wav --format json
orangenote-cli transcribe input.mp3 --output result.json
```

### With Options
```bash
# Specific model
orangenote-cli transcribe audio.mp3 --model small

# Language specification
orangenote-cli transcribe audio.mp3 --language en

# Translation to English
orangenote-cli transcribe audio.mp3 --translate

# Specific threads
orangenote-cli transcribe audio.mp3 --threads 8

# Output formats
orangenote-cli transcribe audio.mp3 --format json   # JSON
orangenote-cli transcribe audio.mp3 --format txt    # Text
orangenote-cli transcribe audio.mp3 --format srt    # SubRip
orangenote-cli transcribe audio.mp3 --format vtt    # WebVTT
orangenote-cli transcribe audio.mp3 --format tsv    # Tab-separated
```

### Model Management
```bash
# List models
orangenote-cli model list

# Download model
orangenote-cli model download base

# Remove model
orangenote-cli model remove base

# Check cache status
orangenote-cli model status
```

### System Information
```bash
orangenote-cli info
```

## Key Classes

### AudioProcessor
```rust
pub fn process<P: AsRef<Path>>(path: P) -> Result<AudioSamples>
```
Decodes audio file and returns normalized PCM samples.

### WhisperTranscriber
```rust
pub fn new<P: AsRef<Path>>(model_path: P, threads: usize) -> Result<Self>
pub fn transcribe_file<P: AsRef<Path>>(
    &self,
    audio_path: P,
    language: Option<&str>,
    translate: bool,
) -> Result<TranscriptionResult>
```
Main transcription engine.

## Supported Formats
- MP3, WAV, FLAC, M4A, OGG, WMA (input)
- JSON, TXT, SRT, VTT, TSV (output)

## Models Available
- tiny (39MB) - Fastest
- base (140MB) - Default
- small (466MB) - Better
- medium (1.5GB) - High accuracy
- large (2.9GB) - Best

## Common Tasks

### Transcribe and Save as JSON
```bash
orangenote-cli transcribe audio.mp3 \
  --format json \
  --output transcript.json
```

### Create Subtitles
```bash
orangenote-cli transcribe audio.mp3 \
  --format srt \
  --output subtitles.srt
```

### Translate Spanish to English
```bash
orangenote-cli transcribe audio_spanish.mp3 \
  --language es \
  --translate \
  --format txt
```

### Use Small Model for Speed
```bash
orangenote-cli transcribe audio.mp3 \
  --model small \
  --threads 8
```

## Performance Tips
- Use smaller models for speed (tiny, base)
- Use more threads for faster inference (--threads 8)
- Large files may use significant memory

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "whisper library not found" | Install whisper.cpp or rebuild with feature |
| "Model not found" | Download with `model download <name>` |
| "Feature whisper not enabled" | Rebuild with `--features whisper` |
| File not found | Check path, use absolute paths if needed |
| Out of memory | Use smaller model or process smaller files |

## Next Steps
- See STEP_A5.md for detailed documentation
- See STEP_A5_COMMIT_GUIDE.md for commit instructions
- See STEP_A5_FINAL_REPORT.md for comprehensive status

