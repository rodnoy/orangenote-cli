# Whisper.cpp Build Instructions for OrangeNote CLI

## Quick Start

### Option 1: Homebrew (macOS) - 5 minutes ⭐ Recommended

```bash
brew install whisper-cpp
cd orangenote-cli
cargo build --features whisper --release
```

### Option 2: Automated Setup Script

```bash
cd orangenote-cli
bash setup-whisper.sh
cargo build --features whisper --release
```

### Option 3: Manual Git Submodule (All Platforms) - 15 minutes

```bash
cd orangenote-cli
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive
cargo build --features whisper --release
```

---

## Detailed Setup Instructions

### macOS with Homebrew (Easiest)

1. **Install whisper.cpp**:
   ```bash
   brew install whisper-cpp
   ```

2. **Build OrangeNote CLI**:
   ```bash
   cd orangenote-cli
   cargo build --features whisper --release
   ```

3. **Verify**:
   ```bash
   ./target/release/orangenote-cli info
   # Should show: Whisper support: ✓ Enabled
   ```

### macOS with Git Submodule

1. **Prerequisites**:
   ```bash
   xcode-select --install  # Install Xcode Command Line Tools
   brew install cmake
   ```

2. **Clone whisper.cpp**:
   ```bash
   cd orangenote-cli
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   ```

3. **Build**:
   ```bash
   cargo build --features whisper --release
   ```

### Linux (Ubuntu/Debian)

1. **Install dependencies**:
   ```bash
   sudo apt-get update
   sudo apt-get install -y cmake build-essential git
   ```

2. **Clone whisper.cpp**:
   ```bash
   cd orangenote-cli
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   ```

3. **Build**:
   ```bash
   cargo build --features whisper --release
   ```

### Linux (Fedora/RHEL)

1. **Install dependencies**:
   ```bash
   sudo dnf install cmake gcc g++ make git
   ```

2. **Clone whisper.cpp**:
   ```bash
   cd orangenote-cli
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   ```

3. **Build**:
   ```bash
   cargo build --features whisper --release
   ```

### Windows

1. **Install prerequisites**:
   - Visual Studio Build Tools (with C++ support): https://visualstudio.microsoft.com/downloads/
   - CMake: https://cmake.org/download/

2. **Clone whisper.cpp** (in PowerShell):
   ```powershell
   cd orangenote-cli
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   ```

3. **Build**:
   ```powershell
   cargo build --features whisper --release
   ```

---

## Build Without Whisper Support

If you only want audio metadata extraction (no transcription):

```bash
cargo build --release
```

The CLI will work but will show an error when attempting transcription.

---

## Troubleshooting

### Error: "ld: library 'whisper' not found"

**Solution 1: Install via Homebrew (macOS)**
```bash
brew install whisper-cpp
cargo clean
cargo build --features whisper --release
```

**Solution 2: Add git submodule**
```bash
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive
cargo clean
cargo build --features whisper --release
```

### Error: "CMake not found"

```bash
# macOS
brew install cmake

# Ubuntu/Debian
sudo apt-get install cmake

# Fedora/RHEL
sudo dnf install cmake
```

### Error: "C compiler not found"

```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc g++ make
```

### Warning: "whisper.cpp not found at vendor/whisper.cpp"

This warning appears when:
- You're building with `--features whisper`
- But don't have whisper.cpp available

**Solutions**:
1. Skip the whisper feature: `cargo build`
2. Install system-wide (macOS): `brew install whisper-cpp`
3. Add submodule: `git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp`

---

## Verification

After successful build:

```bash
# Check whisper is enabled
./target/release/orangenote-cli info
# Output should include: Whisper support: ✓ Enabled

# Download a model
./target/release/orangenote-cli model download base

# Test transcription
./target/release/orangenote-cli transcribe test_audio.wav --format json
```

---

## Build Performance

### Development Builds (Faster)
```bash
cargo build --features whisper
# Unoptimized, builds quickly for testing
```

### Release Builds (Optimized)
```bash
cargo build --features whisper --release
# Optimized, slower to build but faster to run
```

### With Maximum Optimization
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --features whisper --release
```

---

## Clean Build

If you encounter build issues:

```bash
# Clean all artifacts
cargo clean

# Rebuild submodules
git submodule update --init --recursive

# Build fresh
cargo build --features whisper --release
```

---

## Platform Support

| Platform | Status | Method |
|----------|--------|--------|
| macOS (Intel) | ✅ Tested | Homebrew or Submodule |
| macOS (Apple Silicon) | ✅ Tested | Homebrew or Submodule |
| Ubuntu 20.04+ | ✅ Supported | Submodule |
| Ubuntu 22.04+ | ✅ Supported | Submodule |
| Debian | ✅ Supported | Submodule |
| Fedora | ✅ Supported | Submodule |
| RHEL | ✅ Supported | Submodule |
| Windows 10+ | ✅ Supported | Submodule |
| WSL 2 | ✅ Supported | Submodule |

---

## Next Steps

After successful build:

1. **Download a transcription model**:
   ```bash
   ./target/release/orangenote-cli model download base
   ```

2. **Transcribe an audio file**:
   ```bash
   ./target/release/orangenote-cli transcribe audio.mp3 --format json
   ```

3. **See all options**:
   ```bash
   ./target/release/orangenote-cli transcribe --help
   ```

4. **Read documentation**:
   - [Step A5 Implementation Guide](./STEP_A5.md)
   - [Quick Reference](./STEP_A5_QUICK_REFERENCE.md)
   - [Whisper Setup Guide](./WHISPER_SETUP_GUIDE.md)

---

## Automated Setup Script

For convenience, use the provided setup script:

```bash
bash setup-whisper.sh
```

This script:
- Detects your operating system
- Installs required dependencies
- Sets up whisper.cpp
- Builds OrangeNote CLI
- Verifies the installation

---

## Getting Help

- **GitHub Issues**: https://github.com/ggerganov/whisper.cpp/issues
- **Whisper.cpp Docs**: https://github.com/ggerganov/whisper.cpp
- **OrangeNote Issues**: Create an issue in this repository
- **Detailed Guide**: See [WHISPER_SETUP_GUIDE.md](./WHISPER_SETUP_GUIDE.md)

---

## Summary

| Method | Time | Difficulty | Best For |
|--------|------|-----------|----------|
| Homebrew | 5 min | ⭐ Easy | macOS users |
| Setup Script | 10 min | ⭐ Easy | First-time users |
| Submodule | 15 min | ⭐⭐ Medium | All platforms |
| Manual | 30+ min | ⭐⭐⭐ Hard | Advanced users |

**Recommended for most users**: Homebrew on macOS, or the automated setup script on other platforms.