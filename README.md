# OrangeNote CLI

A fast, offline audio transcription tool using whisper.cpp. Transcribe audio files to text without internet or cloud services.

## Features

- 🎙️ **Audio Transcription** — Convert speech to text accurately
- 🌍 **Multi-language Support** — Auto-detect or specify language
- 📁 **Multiple Formats** — MP3, WAV, FLAC, M4A, OGG, WMA
- ⚡ **Fast Processing** — Multi-threaded, optimized for performance
- 💾 **Output Formats** — JSON, SRT, VTT, TXT, TSV
- 🔌 **Modular Design** — Easy integration with other applications
- 📦 **Offline-First** — All processing happens locally
- ✂️ **Audio Chunking** — Split long files for better transcription quality

## Quick Start

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- 4+ GB RAM
- macOS, Linux, or Windows

### Build from Source

```bash
git clone <repo-url>
cd orangenote-cli
cargo build --release --features whisper
```

The binary will be in `target/release/orangenote-cli`

### Basic Usage

```bash
# Transcribe with default settings
./orangenote-cli transcribe input.mp3

# With specific language
./orangenote-cli transcribe input.mp3 --language ru

# With different model (tiny, base, small, medium, large)
./orangenote-cli transcribe input.mp3 --model small

# Save to file
./orangenote-cli transcribe input.mp3 --output result.json

# As SRT subtitles
./orangenote-cli transcribe video.m4a --format srt -o subs.srt
```

## Commands

### Transcribe

```bash
orangenote-cli transcribe <INPUT> [OPTIONS]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-m, --model` | Whisper model (tiny, base, small, medium, large) | base |
| `-l, --language` | Language code (en, ru, fr, etc.) or auto-detect | auto |
| `-f, --format` | Output format (json, srt, vtt, txt, tsv) | json |
| `-o, --output` | Output file (stdout if not specified) | - |
| `-t, --threads` | Processing threads | 4 |
| `--translate` | Translate to English | false |
| `--chunk-size` | Chunk size in minutes (0 = disabled) | 0 |
| `--chunk-overlap` | Overlap between chunks in seconds | 5 |

### Model Management

```bash
# List available models
./orangenote-cli model list

# Download a model
./orangenote-cli model download medium

# Check model status
./orangenote-cli model status

# Remove a model
./orangenote-cli model remove base
```

### System Info

```bash
./orangenote-cli info
```

## Audio Chunking for Long Files

When transcribing long audio files (podcasts, lectures, meetings), whisper.cpp may produce poor results — repeating noise labels like "[Music]" or hallucinating text. The solution is to split audio into smaller chunks.

### How It Works

```
┌────────────────────────────────────────────────────────────┐
│                    Original Audio File                      │
└────────────────────────────────────────────────────────────┘
                              ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Chunk 1    │  │   Chunk 2    │  │   Chunk 3    │
│   0:00-5:00  │  │  4:55-9:55   │  │  9:50-12:00  │
└──────────────┘  └──────────────┘  └──────────────┘
        ↓                 ↓                 ↓
   Transcribe        Transcribe        Transcribe
        ↓                 ↓                 ↓
┌────────────────────────────────────────────────────────────┐
│           Merged Result (duplicates removed)                │
└────────────────────────────────────────────────────────────┘
```

### Usage

```bash
# Transcribe a 1-hour podcast with 5-minute chunks
./orangenote-cli transcribe podcast.mp3 \
  --model medium \
  --language ru \
  --chunk-size 5

# With custom overlap (10 seconds)
./orangenote-cli transcribe lecture.mp3 \
  --model small \
  --chunk-size 10 \
  --chunk-overlap 10

# Long meeting recording
./orangenote-cli transcribe meeting.wav \
  --model medium \
  --language en \
  --chunk-size 5 \
  --output meeting_transcript.json
```

### Recommendations

| Audio Duration | Recommended Chunk Size | Model |
|----------------|------------------------|-------|
| < 10 minutes | No chunking needed | base/small |
| 10-30 minutes | 5 minutes | small/medium |
| 30-60 minutes | 5-10 minutes | medium |
| > 1 hour | 5 minutes | medium/large |

### Output Example

```
🎵 Processing audio...
  📦 Using chunked transcription (5 min chunks, 5s overlap)
  Processing chunk 1/12...
  Processing chunk 2/12...
  ...
  Processing chunk 12/12...
✓ Transcription complete!
  Detected language: ru
  Segments: 156
  Average confidence: 94.32%
```

## Model Comparison

| Model | Size | Speed | Accuracy | RAM | Best For |
|-------|------|-------|----------|-----|----------|
| tiny | 39M | ⭐⭐⭐⭐⭐ | ⭐ | ~500MB | Quick tests |
| base | 140M | ⭐⭐⭐⭐ | ⭐⭐ | ~1GB | Short clips |
| small | 466M | ⭐⭐⭐ | ⭐⭐⭐ | ~2GB | General use |
| medium | 1.5G | ⭐⭐ | ⭐⭐⭐⭐ | ~5GB | Quality transcription |
| large | 2.9G | ⭐ | ⭐⭐⭐⭐⭐ | ~10GB | Maximum accuracy |

**Recommendation:** Use `small` or `medium` model with `--language` flag for best results.

## Output Formats

- **JSON** — Full structured output with timestamps and confidence scores
- **SRT** — SubRip format for video players
- **VTT** — WebVTT format for web videos
- **TXT** — Plain text (text only, no timestamps)
- **TSV** — Tab-separated for spreadsheets

## Examples

### Transcribe Russian podcast

```bash
./orangenote-cli transcribe podcast.mp3 \
  --model medium \
  --language ru \
  --chunk-size 5 \
  --output transcript.json
```

### Create video subtitles

```bash
./orangenote-cli transcribe movie.m4a \
  --model small \
  --format srt \
  --output subtitles.srt
```

### High-accuracy English transcription

```bash
./orangenote-cli transcribe interview.wav \
  --model large \
  --language en \
  --threads 8 \
  --chunk-size 5 \
  --output interview.json
```

### Translate French to English

```bash
./orangenote-cli transcribe french_audio.mp3 \
  --model medium \
  --language fr \
  --translate \
  --output english_translation.txt
```

### Batch processing

```bash
for file in *.mp3; do
  ./orangenote-cli transcribe "$file" \
    --model small \
    --chunk-size 5 \
    --output "${file%.mp3}.json"
done
```

## Logging

Enable detailed logging:

```bash
# Verbose mode (debug level)
./orangenote-cli transcribe input.mp3 --verbose

# Specific log level
./orangenote-cli transcribe input.mp3 --log-level debug
```

## Troubleshooting

### "[Music]" or repeated noise labels

Use chunking with a smaller chunk size:
```bash
./orangenote-cli transcribe audio.mp3 --model medium --chunk-size 5
```

### Poor transcription quality

1. Use a larger model (`medium` or `large`)
2. Specify the language explicitly with `--language`
3. Enable chunking for long files

### File not found

Check the path exists:
```bash
ls -la your_file.mp3
```

### Unsupported format

Convert the file first (requires ffmpeg):
```bash
ffmpeg -i input.ogg -ar 16000 -ac 1 output.wav
```

### Out of memory

Use a smaller model:
```bash
./orangenote-cli transcribe audio.mp3 --model tiny
```

### Slow performance

1. Increase threads: `--threads 8`
2. Use a smaller model: `--model small`
3. Use chunking to process in parallel (future feature)

## Project Status

**Current Version:** 0.2.0

### Implemented Features

- [x] Basic CLI structure
- [x] Argument parsing and validation
- [x] Whisper backend integration
- [x] Model management (download, list, remove)
- [x] Multiple output formats (JSON, SRT, VTT, TXT, TSV)
- [x] Audio chunking for long files
- [x] Duplicate segment removal
- [x] Confidence-based segment selection

### Roadmap

- [ ] Diarization support (speaker identification)
- [ ] Result caching
- [ ] REST API
- [ ] Desktop UI (Tauri)
- [ ] Parallel chunk processing

## Documentation

Full documentation is available in Russian: [doc/README_RU.md](doc/README_RU.md)

## Development

### Requirements

```bash
# Update Rust
rustup update

# Install development tools
rustup component add rustfmt clippy
```

### Build

```bash
# Debug build
cargo build --features whisper

# Release build
cargo build --release --features whisper

# Run tests
cargo test --features whisper
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy --features whisper

# Run all tests
cargo test --features whisper
```

### Project Structure

```
orangenote-cli/
├── src/
│   ├── lib.rs                    # Library exports
│   ├── bin/
│   │   └── orangenote-cli.rs     # Main CLI binary
│   └── infrastructure/
│       ├── audio/
│       │   ├── chunk.rs          # Audio chunking
│       │   ├── decoder.rs        # Audio decoding
│       │   └── processor.rs      # Audio processing
│       └── transcription/
│           └── whisper/
│               ├── transcriber.rs # Main transcription engine
│               ├── merger.rs      # Result merging
│               ├── context.rs     # Whisper context wrapper
│               └── model_manager.rs # Model management
├── vendor/
│   └── whisper.cpp/              # Whisper.cpp submodule
├── doc/                          # Documentation
├── Cargo.toml                    # Project manifest
└── build.rs                      # Build script for whisper.cpp
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details

## See Also

- [whisper.cpp](https://github.com/ggerganov/whisper.cpp) — Underlying transcription engine
- [OpenAI Whisper](https://github.com/openai/whisper) — Original model and paper

---

**Made with ❤️ by the OrangeNote Team**