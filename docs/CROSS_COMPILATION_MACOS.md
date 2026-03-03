# macOS Cross-Compilation Guide

## Problem: x86_64 Cross-Compilation on Apple Silicon

### Symptom
When cross-compiling for `x86_64-apple-darwin` on Apple Silicon (aarch64), the build fails with:
```
error: unknown target CPU 'apple-m1'
```

### Root Cause

The error occurs in `ggml.c` compilation because:

1. **CMake doesn't auto-detect cross-compilation** without a toolchain file or explicit `CMAKE_SYSTEM_NAME`
2. When `CMAKE_CROSSCOMPILING` is `FALSE`, ggml sets `GGML_NATIVE_DEFAULT=ON`
3. When `GGML_NATIVE=ON`, ggml executes the compiler with `-mcpu=native` to detect CPU features
4. On Apple Silicon host, this returns `-mcpu=apple-m1`
5. This flag is added to CFLAGS, **overriding** our `-arch x86_64` flags
6. The x86_64 compiler rejects `-mcpu=apple-m1` as invalid

### Code Flow

**File**: `vendor/whisper.cpp/ggml/CMakeLists.txt:97-102`
```cmake
if (CMAKE_CROSSCOMPILING OR DEFINED ENV{SOURCE_DATE_EPOCH})
    message(STATUS "Setting GGML_NATIVE_DEFAULT to OFF")
    set(GGML_NATIVE_DEFAULT OFF)
else()
    set(GGML_NATIVE_DEFAULT ON)  # ← Problem: ON during cross-compilation
endif()
```

**File**: `vendor/whisper.cpp/ggml/src/ggml-cpu/CMakeLists.txt:116-144`
```cmake
if (GGML_NATIVE)
    # Runs: clang -mcpu=native -E -v -
    # On Apple Silicon returns: -mcpu=apple-m1
    execute_process(
        COMMAND ${CMAKE_C_COMPILER} -mcpu=native -E -v -
        ...
    )
    # Adds -mcpu=apple-m1 to ARCH_FLAGS
endif()
```

### Solution

**File**: `build.rs:214-217`

Explicitly disable `GGML_NATIVE` when cross-compiling:

```rust
// CRITICAL: Disable GGML_NATIVE to prevent -mcpu=apple-m1 from being added
// When GGML_NATIVE=ON, ggml runs the compiler with -mcpu=native which returns
// -mcpu=apple-m1 on Apple Silicon host, breaking x86_64 cross-compilation
cmake_configure_cmd.arg("-DGGML_NATIVE=OFF");
```

### Complete Fix

The fix includes:
1. `-DCMAKE_SYSTEM_NAME=Darwin` - Helps CMake detect cross-compilation
2. `-DCMAKE_SYSTEM_PROCESSOR=x86_64` - Sets target processor
3. `-DCMAKE_OSX_ARCHITECTURES=x86_64` - Sets macOS architecture
4. `-DCMAKE_C_FLAGS=-arch x86_64` - Forces x86_64 compilation
5. `-DCMAKE_CXX_FLAGS=-arch x86_64` - Forces x86_64 compilation
6. **`-DGGML_NATIVE=OFF`** - **Critical**: Prevents host CPU detection

### Alternative Solutions

#### Option A: Use Intel Runner (Simpler)
In `.github/workflows/release.yml`:
```yaml
- target: x86_64-apple-darwin
  os: macos-13  # Intel runner, no cross-compilation needed
```

**Pros**: No cross-compilation complexity
**Cons**: Requires Intel runner availability

#### Option B: CMake Toolchain File (More Complex)
Create a toolchain file for cross-compilation:
```cmake
set(CMAKE_SYSTEM_NAME Darwin)
set(CMAKE_SYSTEM_PROCESSOR x86_64)
set(CMAKE_OSX_ARCHITECTURES x86_64)
set(CMAKE_C_FLAGS "-arch x86_64")
set(CMAKE_CXX_FLAGS "-arch x86_64")
```

**Pros**: Standard CMake approach
**Cons**: More complex, requires file management

### Performance Impact

Disabling `GGML_NATIVE=OFF` means:
- No CPU-specific optimizations (no `-mcpu=native`)
- Generic x86_64 code generation
- Slightly slower performance vs native build

For production x86_64 builds, prefer **Option A** (Intel runner) for optimal performance.

### Testing

To test the fix locally on Apple Silicon:
```bash
cargo clean
cargo build --target x86_64-apple-darwin --features whisper --release
```

Expected output:
```
warning: orangenote-cli@0.2.0: Configuring for x86_64 cross-compilation on Apple Silicon
warning: orangenote-cli@0.2.0: CMake configuration succeeded
warning: orangenote-cli@0.2.0: CMake build succeeded
```

### References

- ggml CMake configuration: `vendor/whisper.cpp/ggml/CMakeLists.txt`
- ggml CPU detection: `vendor/whisper.cpp/ggml/src/ggml-cpu/CMakeLists.txt`
- Build script: `build.rs`
