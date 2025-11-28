# Whisper.cpp Setup Guide for OrangeNote CLI

## Overview

To build OrangeNote CLI with whisper transcription support, you need to set up whisper.cpp on your system. There are two approaches:

1. **Easy Way**: Install via Homebrew (macOS only)
2. **Flexible Way**: Clone whisper.cpp as a git submodule (all platforms)

---

## Option 1: Easy Setup (macOS with Homebrew)

### Step 1: Install whisper.cpp via Homebrew

```bash
brew install whisper-cpp
```

This installs:
- The whisper C library to `/opt/homebrew/lib/libwhisper.a`
- Header files to `/opt/homebrew/include/whisper.h`

### Step 2: Build OrangeNote CLI

```bash
cd orangenote-cli
cargo build --features whisper --release
```

**That's it!** The build script will find the system-installed whisper library.

---

## Option 2: Complete Setup (All Platforms)

### Prerequisites

You need:
- Git
- CMake (version 3.15+)
- C/C++ compiler (Clang, GCC, or MSVC)
- Rust toolchain

#### macOS
```bash
brew install cmake
# Clang is included with Xcode Command Line Tools
xcode-select --install
```

#### Ubuntu/Debian
```bash
sudo apt-get install cmake build-essential git
```

#### Fedora/RHEL
```bash
sudo dnf install cmake gcc g++ git
```

#### Windows
- Install CMake from https://cmake.org/download/
- Install Visual Studio Build Tools with C++ support

### Step 1: Clone whisper.cpp as Submodule

```bash
cd orangenote-cli
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive
```

This creates a `vendor/whisper.cpp` directory with the complete whisper.cpp source.

### Step 2: Build OrangeNote CLI

```bash
cargo build --features whisper --release
```

The build script will:
1. Detect the whisper.cpp directory
2. Configure CMake
3. Build whisper.cpp
4. Link it into OrangeNote CLI

### Step 3: Verify Installation

```bash
./target/release/orangenote-cli --version
./target/release/orangenote-cli info
```

You should see:
```
OrangeNote CLI v0.2.0
System Information:
  • OS: macos
  • Arch: aarch64
  • Family: unix
  • Whisper support: ✓ Enabled
```

---

## Testing the Installation

### Download a Model

```bash
./target/release/orangenote-cli model download base
```

The model downloads to `~/.cache/orangenote/models/`

### Transcribe Audio

```bash
./target/release/orangenote-cli transcribe audio.mp3 --format json
```

If you see transcription output with timestamps, whisper is working!

---

## Troubleshooting

### Error: "ld: library 'whisper' not found"

**Cause**: Whisper library not found in standard locations.

**Solutions**:

1. **Try Homebrew** (easiest):
   ```bash
   brew install whisper-cpp
   cargo clean
   cargo build --features whisper --release
   ```

2. **Set custom search path**:
   ```bash
   export RUSTFLAGS="-l /path/to/libwhisper.a"
   cargo build --features whisper --release
   ```

3. **Rebuild from submodule**:
   ```bash
   git submodule update --init --recursive
   cargo clean
   cargo build --features whisper --release
   ```

### Error: "CMake not found"

**Solution**: Install CMake
```bash
# macOS
brew install cmake

# Linux
sudo apt-get install cmake

# or download from https://cmake.org/download/
```

### Error: "C compiler not found"

**Solution**: Install build tools
```bash
# macOS
xcode-select --install

# Ubuntu
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc g++ make
```

### Warning: "whisper.cpp not found at vendor/whisper.cpp"

**Cause**: You're trying to build with whisper feature but don't have the submodule.

**Solutions**:

1. **Skip whisper feature**:
   ```bash
   cargo build --release
   ```

2. **Add submodule**:
   ```bash
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   cargo build --features whisper --release
   ```

3. **Install system-wide** (macOS):
   ```bash
   brew install whisper-cpp
   cargo build --features whisper --release
   ```

---

## Building Without Whisper Support

If you want to build the CLI without transcription support (audio metadata only):

```bash
cargo build --release
```

The CLI will still work but will show:
```
❌ Error: Whisper feature not enabled!
Please rebuild with: cargo build --features whisper
```

---

## Platform-Specific Notes

### macOS

**Best approach**: Use Homebrew
```bash
brew install whisper-cpp
cargo build --features whisper --release
```

**Alternative**: Build from submodule (requires Xcode)
```bash
xcode-select --install
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
cargo build --features whisper --release
```

### Linux (Ubuntu)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y cmake build-essential

# Add submodule
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive

# Build
cargo build --features whisper --release
```

### Linux (Fedora)

```bash
# Install dependencies
sudo dnf install cmake gcc g++ make

# Add submodule
git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
git submodule update --init --recursive

# Build
cargo build --features whisper --release
```

### Windows

1. Install Visual Studio Build Tools with C++ support
2. Install CMake from https://cmake.org
3. Open PowerShell and run:
   ```powershell
   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
   git submodule update --init --recursive
   cargo build --features whisper --release
   ```

---

## Advanced Configuration

### Build Only the Library (Developers)

If you want to build whisper.cpp separately:

```bash
# Build whisper.cpp manually
cd vendor/whisper.cpp
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF ..
cmake --build . --config Release

# Then build OrangeNote CLI
cd ../../
cargo build --features whisper
```

### Use Existing Whisper Installation

If you already have whisper.cpp built elsewhere:

```bash
# Tell cargo where to find it
export RUSTFLAGS="-L /path/to/whisper/build"
cargo build --features whisper
```

### Force Clean Build

```bash
cargo clean
rm -rf vendor/whisper.cpp/.git  # if you want to rebuild submodule
git submodule update --init --recursive
cargo build --features whisper --release
```

---

## Verification Checklist

After setup, verify everything works:

```bash
# 1. Check whisper support is enabled
./target/release/orangenote-cli info
# Should show: Whisper support: ✓ Enabled

# 2. Download a model
./target/release/orangenote-cli model download base
# Should complete successfully

# 3. Try transcription
./target/release/orangenote-cli transcribe test_audio.wav --format json
# Should produce JSON output with segments and timestamps
```

---

## Performance Tips

### Build Optimization

For best performance in release builds:

```bash
# Already optimized in Cargo.toml
cargo build --features whisper --release

# Or with additional optimization
RUSTFLAGS="-C target-cpu=native" cargo build --features whisper --release
```

### Faster Builds During Development

Use debug builds while developing:

```bash
cargo build --features whisper
# Much faster than release
```

### Parallel Compilation

Enable parallel compilation:

```bash
# Use all available cores (already default)
cargo build --features whisper -j 8
```

---

## Getting Help

### Documentation

- [Whisper.cpp GitHub](https://github.com/ggerganov/whisper.cpp)
- [OrangeNote CLI Docs](./STEP_A5.md)
- [Quick Reference](./STEP_A5_QUICK_REFERENCE.md)

### Common Issues

See [Step A5 Quick Reference](./STEP_A5_QUICK_REFERENCE.md) for troubleshooting common transcription issues.

### Build Logs

For detailed build information:

```bash
# Verbose build output
cargo build --features whisper -vv

# With environment debugging
RUSTFLAGS="-Z print-link-args" cargo build --features whisper
```

---

## Next Steps

Once setup is complete:

1. **Download a model**:
   ```bash
   ./target/release/orangenote-cli model download base
   ```

2. **Test with sample audio**:
   ```bash
   ./target/release/orangenote-cli transcribe test_audio.wav
   ```

3. **Try different formats**:
   ```bash
   ./target/release/orangenote-cli transcribe audio.mp3 --format srt --output subs.srt
   ```

4. **Read the docs**:
   - [Implementation Guide](./STEP_A5.md)
   - [Quick Reference](./STEP_A5_QUICK_REFERENCE.md)
   - [Commit Guide](./STEP_A5_COMMIT_GUIDE.md)

---

## Summary

| Method | Difficulty | Time | Supported Platforms |
|--------|-----------|------|-------------------|
| Homebrew (macOS) | ⭐ Easy | 5 min | macOS only |
| Git Submodule | ⭐⭐ Medium | 15 min | All platforms |
| Manual Build | ⭐⭐⭐ Hard | 30+ min | All platforms |

**Recommended**: Use Homebrew on macOS, git submodule on other platforms.