# 🚀 Getting Started with OrangeNote CLI

Welcome! This guide will help you get up and running with OrangeNote CLI Step A5.

## 📋 Quick Navigation

### I want to...

**...build OrangeNote CLI quickly** 
→ See [WHISPER_BUILD_INSTRUCTIONS.md](./WHISPER_BUILD_INSTRUCTIONS.md)

**...understand what was implemented**
→ Read [STEP_A5_FINAL_REPORT.md](./STEP_A5_FINAL_REPORT.md)

**...learn how to use it**
→ Check [STEP_A5_QUICK_REFERENCE.md](./STEP_A5_QUICK_REFERENCE.md)

**...understand the architecture**
→ Read [STEP_A5.md](./STEP_A5.md)

**...set up whisper.cpp in detail**
→ See [WHISPER_SETUP_GUIDE.md](./WHISPER_SETUP_GUIDE.md)

**...prepare commits for version control**
→ Use [STEP_A5_COMMIT_GUIDE.md](./STEP_A5_COMMIT_GUIDE.md)

---

## ⚡ 5-Minute Quick Start

### For macOS Users (Easiest)

```bash
# 1. Install whisper.cpp via Homebrew
brew install whisper-cpp

# 2. Build OrangeNote CLI
cargo build --features whisper --release

# 3. Verify it works
./target/release/orangenote-cli info

# 4. Download a model
./target/release/orangenote-cli model download base

# 5. Try it out
./target/release/orangenote-cli transcribe test_audio.wav
```

### For All Other Platforms

```bash
# 1. Run the setup script
bash setup-whisper.sh

# 2. The script handles everything automatically!
# 3. Once done:
./target/release/orangenote-cli info

# 4. Download a model
./target/release/orangenote-cli model download base

# 5. Try it out
./target/release/orangenote-cli transcribe test_audio.wav
```

---

## 📚 Documentation Overview

### Setup & Installation
| File | Purpose | Time |
|------|---------|------|
| [WHISPER_BUILD_INSTRUCTIONS.md](./WHISPER_BUILD_INSTRUCTIONS.md) | Quick build guide for all platforms | 5-15 min |
| [WHISPER_SETUP_GUIDE.md](./WHISPER_SETUP_GUIDE.md) | Comprehensive setup with troubleshooting | 30 min |
| `setup-whisper.sh` | Automated setup script | 10 min |

### Understanding the Implementation
| File | Purpose | Audience |
|------|---------|----------|
| [STEP_A5_FINAL_REPORT.md](./STEP_A5_FINAL_REPORT.md) | Executive summary of what was built | Everyone |
| [STEP_A5.md](./STEP_A5.md) | Detailed implementation guide | Developers |
| [STEP_A5_SUMMARY.md](./STEP_A5_SUMMARY.md) | Quick summary of deliverables | Quick reference |

### Using OrangeNote CLI
| File | Purpose | Use Case |
|------|---------|----------|
| [STEP_A5_QUICK_REFERENCE.md](./STEP_A5_QUICK_REFERENCE.md) | Command reference and examples | Daily use |
| [STEP_A5_COMMIT_GUIDE.md](./STEP_A5_COMMIT_GUIDE.md) | Git commit instructions | Version control |

---

## 🎯 Common Tasks

### Task 1: Install and Build

**Option A - macOS with Homebrew (Recommended)**
```bash
brew install whisper-cpp
cargo build --features whisper --release
```

**Option B - Automated Script (All Platforms)**
```bash
bash setup-whisper.sh
```

**Option C - Manual Git Submodule (All Platforms)**
```bash
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive
cargo build --features whisper --release
```

### Task 2: Verify Installation

```bash
./target/release/orangenote-cli info
# Should show: Whisper support: ✓ Enabled
```

### Task 3: Download a Model

```bash
./target/release/orangenote-cli model download base
# Available models: tiny, base, small, medium, large
```

### Task 4: Transcribe Audio

```bash
# Basic transcription
./target/release/orangenote-cli transcribe audio.mp3

# Save as JSON
./target/release/orangenote-cli transcribe audio.mp3 --format json --output result.json

# Create subtitles
./target/release/orangenote-cli transcribe audio.mp3 --format srt --output subs.srt

# Translate to English
./target/release/orangenote-cli transcribe audio_spanish.mp3 --translate --format txt
```

### Task 5: Manage Models

```bash
# List available and cached models
./target/release/orangenote-cli model list

# Check cache status
./target/release/orangenote-cli model status

# Remove a model
./target/release/orangenote-cli model remove base
```

---

## 🏗️ Architecture Overview

```
Audio File (.mp3, .wav, etc.)
    ↓
[Symphonia Decoder]
    ↓
[AudioProcessor] - Convert to 16kHz mono
    ↓
[WhisperTranscriber] - Perform transcription
    ↓
[Output Formatter] - JSON, TXT, SRT, VTT, TSV
    ↓
Result File or stdout
```

---

## 📊 What's Included

### Core Implementation
- ✅ **AudioProcessor** - Multi-format audio decoding and processing (361 lines)
- ✅ **WhisperTranscriber** - Transcription engine (224 lines)
- ✅ **CLI Interface** - Full command-line tool (~600 lines)

### Features
- ✅ Support for 6+ audio formats (MP3, WAV, FLAC, M4A, OGG, WMA)
- ✅ 5 output formats (JSON, TXT, SRT, VTT, TSV)
- ✅ Language detection and translation
- ✅ Configurable threading and model selection
- ✅ Model management (download, list, remove, status)

### Quality
- ✅ Zero compiler warnings
- ✅ All tests pass
- ✅ Comprehensive documentation (1000+ lines)
- ✅ Production-ready code

---

## ❓ Frequently Asked Questions

**Q: Can I use OrangeNote without setting up whisper.cpp?**
A: Yes! You can build without the `--features whisper` flag. You'll get audio metadata extraction but not transcription.

**Q: Which setup method is best?**
A: Homebrew on macOS (5 min), automated script on Linux/Windows (10 min).

**Q: How much disk space do I need?**
A: Whisper.cpp build: ~500MB. Models: 39MB (tiny) to 3GB (large).

**Q: Does it work on Windows?**
A: Yes! You need Visual Studio Build Tools + CMake, then use git submodule method.

**Q: How long does transcription take?**
A: Depends on model and audio length. Tiny: seconds, Large: minutes per hour of audio.

**Q: Can I use different models?**
A: Yes! Available: tiny (39MB, fast), base (140MB, balanced), small, medium, large (best accuracy).

---

## 🔧 Troubleshooting

### "ld: library 'whisper' not found"
Try one of these:
1. `brew install whisper-cpp` (macOS)
2. `bash setup-whisper.sh` (all platforms)
3. Manual submodule setup (see WHISPER_BUILD_INSTRUCTIONS.md)

### "CMake not found"
```bash
brew install cmake        # macOS
apt-get install cmake     # Linux
# Windows: Download from cmake.org
```

### Build seems to hang
This is normal! CMake is building whisper.cpp (~2-5 minutes). Wait for it.

### More issues?
See [WHISPER_SETUP_GUIDE.md](./WHISPER_SETUP_GUIDE.md#troubleshooting) for comprehensive troubleshooting.

---

## 📖 Next Steps

1. **Choose a setup method** from the quick start section
2. **Run the setup commands** for your platform
3. **Verify with**: `./target/release/orangenote-cli info`
4. **Download a model**: `./target/release/orangenote-cli model download base`
5. **Test transcription**: `./target/release/orangenote-cli transcribe test_audio.wav`
6. **Read the docs** for advanced features

---

## 📞 Getting Help

### Documentation
- [Setup Instructions](./WHISPER_BUILD_INSTRUCTIONS.md) - Build guide
- [Setup Guide](./WHISPER_SETUP_GUIDE.md) - Detailed guide
- [Quick Reference](./STEP_A5_QUICK_REFERENCE.md) - Command reference
- [Implementation Details](./STEP_A5.md) - How it works

### External Resources
- [Whisper.cpp GitHub](https://github.com/ggerganov/whisper.cpp)
- [Rust Book](https://doc.rust-lang.org/book/)

---

## 🎉 You're Ready!

Your Step A5 implementation is complete! Follow the quick start guide above and you'll be transcribing audio in minutes.

Happy transcribing! 🚀
