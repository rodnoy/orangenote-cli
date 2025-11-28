//! Build script for orangenote-cli
//!
//! This script handles compilation of whisper.cpp when the `whisper` feature is enabled.
//! It uses cmake to build the whisper.cpp library and links it to the Rust crate.

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let _out_path = PathBuf::from(&out_dir);

    // Only compile whisper.cpp if the whisper feature is enabled
    #[cfg(feature = "whisper")]
    {
        build_whisper(&_out_path);
    }

    // Print cargo directives
    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(feature = "whisper")]
fn build_whisper(out_dir: &PathBuf) {
    use std::process::Command;

    let whisper_dir = PathBuf::from("vendor/whisper.cpp");

    // Check if whisper.cpp directory exists
    if !whisper_dir.exists() {
        println!(
            "cargo:warning=whisper.cpp not found at {}. \
            To enable whisper support, clone it as a git submodule:\n\
            git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp\n\
            git submodule update --init --recursive",
            whisper_dir.display()
        );
        // Don't link - whisper library not available
        return;
    }

    // Build whisper.cpp using cmake
    let build_dir = out_dir.join("whisper_build");
    std::fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    // Run cmake configure
    let cmake_status = Command::new("cmake")
        .current_dir(&build_dir)
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DWHISPER_CPP_ONLY=ON")
        .arg(&whisper_dir)
        .status()
        .expect("Failed to run cmake configure");

    if !cmake_status.success() {
        eprintln!("CMake configuration failed");
        std::process::exit(1);
    }

    // Run cmake build
    let build_status = Command::new("cmake")
        .arg("--build")
        .arg(&build_dir)
        .arg("--config")
        .arg("Release")
        .status()
        .expect("Failed to run cmake build");

    if !build_status.success() {
        eprintln!("CMake build failed");
        std::process::exit(1);
    }

    // Check if library was built successfully
    let lib_path = build_dir.join("bin").join("libwhisper.a");
    if !lib_path.exists() {
        eprintln!("Whisper library not found at {}", lib_path.display());
        eprintln!("Build may have failed");
        std::process::exit(1);
    }

    // Link the whisper library
    let lib_dir = build_dir.join("bin");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=whisper");

    // Also try to find the library in common locations
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    println!("cargo:rerun-if-changed={}", whisper_dir.display());
}
