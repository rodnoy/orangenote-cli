# Documentation Index

This directory contains technical documentation for orangenote-cli development and troubleshooting.

## Build & Compilation

### [CROSS_COMPILATION_MACOS.md](CROSS_COMPILATION_MACOS.md)
**Problem**: x86_64 cross-compilation fails on Apple Silicon with `error: unknown target CPU 'apple-m1'`

**Root Cause**: ggml's `GGML_NATIVE=ON` runs compiler with `-mcpu=native` on host, returning `-mcpu=apple-m1` which breaks x86_64 cross-compilation

**Solution**: Explicitly set `-DGGML_NATIVE=OFF` in CMake configuration when cross-compiling

**When to read**: 
- Cross-compilation errors on macOS
- Unknown CPU target errors in ggml/whisper.cpp build
- Setting up CI/CD for multi-architecture builds

---

## How to Use This Index

1. **Search by symptom**: Look for error messages or symptoms you're experiencing
2. **Search by component**: Find documentation related to specific parts (build system, whisper.cpp, etc.)
3. **Add new docs**: When creating new documentation, add an entry here with:
   - File link
   - Problem description
   - Root cause summary
   - Solution summary
   - When to read this doc
