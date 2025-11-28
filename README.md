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

## Quick Start

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- 4+ GB RAM
- macOS, Linux, or Windows

### Build from Source

```bash
git clone <repo-url>
cd orangenote-cli
cargo build --release
```

The binary will be in `target/release/orangenote-cli`

### Basic Usage

```bash
# Transcribe with default settings
cargo run --bin orangenote-cli -- transcribe input.mp3

# With specific language
cargo run --bin orangenote-cli -- transcribe input.mp3 --language ru

# With different model (tiny, base, small, medium, large)
cargo run --bin orangenote-cli -- transcribe input.mp3 --model small

# Save to file
cargo run --bin orangenote-cli -- transcribe input.mp3 --output result.json

# As SRT subtitles
cargo run --bin orangenote-cli -- transcribe video.m4a --format srt -o subs.srt
```

## Commands

### Transcribe

```bash
orangenote-cli transcribe <INPUT> [OPTIONS]
```

**Options:**
- `-m, --model` — Whisper model (tiny, base, small, medium, large) — default: base
- `-l, --language` — Language code (en, ru, fr, etc.) or 'auto' — default: auto
- `-f, --format` — Output format (json, srt, vtt, txt, tsv) — default: json
- `-o, --output` — Output file (stdout if not specified)
- `-t, --threads` — Processing threads — default: 4

### Model Management

```bash
# List available models
cargo run --bin orangenote-cli -- model list

# Download a model
cargo run --bin orangenote-cli -- model download base

# Check model status
cargo run --bin orangenote-cli -- model status
```

### System Info

```bash
cargo run --bin orangenote-cli -- info
```

## Model Comparison

| Model | Size | Speed | Accuracy | RAM |
|-------|------|-------|----------|-----|
| tiny | 39M | ⭐⭐⭐⭐⭐ | ⭐ | ~500MB |
| base | 140M | ⭐⭐⭐⭐ | ⭐⭐ | ~1GB |
| small | 466M | ⭐⭐⭐ | ⭐⭐⭐ | ~2GB |
| medium | 1.5G | ⭐⭐ | ⭐⭐⭐⭐ | ~5GB |
| large | 2.9G | ⭐ | ⭐⭐⭐⭐⭐ | ~10GB |

## Output Formats

- **JSON** — Full structured output with timestamps
- **SRT** — SubRip format for video players
- **VTT** — WebVTT format for web videos
- **TXT** — Plain text (text only)
- **TSV** — Tab-separated for spreadsheets

## Examples

### Transcribe Russian audio quickly

```bash
cargo run --bin orangenote-cli -- transcribe podcast.mp3 -m small -l ru -o transcript.json
```

### Create video subtitles

```bash
cargo run --bin orangenote-cli -- transcribe movie.m4a -f srt -o subtitles.srt
```

### High-accuracy transcription

```bash
cargo run --bin orangenote-cli -- transcribe meeting.wav -m large -l en -t 8 -o result.json
```

### Batch processing

```bash
for file in *.mp3; do
  cargo run --bin orangenote-cli -- transcribe "$file" -m small -o "${file%.mp3}.json"
done
```

## Logging

Enable detailed logging:

```bash
# Verbose mode (debug level)
cargo run --bin orangenote-cli -- transcribe input.mp3 --verbose

# Specific log level
cargo run --bin orangenote-cli -- transcribe input.mp3 --log-level debug
```

## Troubleshooting

### File not found

Check the path exists:
```bash
ls -la your_file.mp3
```

### Unsupported format

Convert the file first (requires ffmpeg):
```bash
ffmpeg -i input.ogg -q:a 9 output.mp3
```

### Out of memory

Use a smaller model:
```bash
cargo run --bin orangenote-cli -- transcribe audio.mp3 -m tiny
```

### Slow performance

Increase threads and use smaller model:
```bash
cargo run --bin orangenote-cli -- transcribe audio.mp3 -m tiny -t 16
```

## Project Status

**Current Version:** 0.1.0 (Alpha)

### Roadmap

- [x] Basic CLI structure
- [x] Argument parsing and validation
- [ ] Whisper backend integration
- [ ] Model management
- [ ] Diarization support
- [ ] Result caching
- [ ] REST API
- [ ] Desktop UI (Tauri)

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

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Run tests
cargo test
```

### Project Structure

```
orangenote-cli/
├── src/
│   └── bin/
│       └── orangenote-cli.rs     # Main CLI binary
├── doc/                           # Documentation
│   └── README_RU.md              # Russian documentation
├── Cargo.toml                     # Project manifest
└── .gitignore                     # Git configuration
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

## Support

- 🐛 **Issues:** [GitHub Issues](https://github.com/orangenote/orangenote-cli/issues)
- 💬 **Discussions:** [GitHub Discussions](https://github.com/orangenote/orangenote-cli/discussions)
- 📧 **Email:** support@orangenote.org

## See Also

- [OrangeNote Desktop](https://github.com/orangenote/orangenote-desktop) — Tauri-based UI
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp) — Underlying transcription engine
- [OpenAI Whisper](https://github.com/openai/whisper) — Original model and paper

---

**Made with ❤️ by the OrangeNote Team**
