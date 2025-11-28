#!/bin/bash

# OrangeNote CLI - Whisper Setup Script
# This script automates the setup of whisper.cpp for building OrangeNote CLI

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper functions
print_info() {
    echo -e "${BLUE}ℹ ${1}${NC}"
}

print_success() {
    echo -e "${GREEN}✓ ${1}${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${1}${NC}"
}

print_error() {
    echo -e "${RED}✗ ${1}${NC}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Main setup flow
main() {
    echo ""
    print_info "OrangeNote CLI - Whisper.cpp Setup"
    echo ""

    OS=$(detect_os)
    print_info "Detected OS: $OS"
    echo ""

    # Check prerequisites
    print_info "Checking prerequisites..."
    echo ""

    # Check for Rust
    if ! command_exists cargo; then
        print_error "Rust toolchain not found"
        print_info "Install from: https://rustup.rs/"
        exit 1
    fi
    print_success "Rust toolchain found"

    # Check for Git
    if ! command_exists git; then
        print_error "Git not found"
        exit 1
    fi
    print_success "Git found"

    # Platform-specific setup
    case "$OS" in
        macos)
            setup_macos
            ;;
        linux)
            setup_linux
            ;;
        windows)
            setup_windows
            ;;
        *)
            print_error "Unsupported OS: $OS"
            exit 1
            ;;
    esac

    # Build OrangeNote CLI
    build_orangenote

    # Verify installation
    verify_installation
}

setup_macos() {
    echo ""
    print_info "Setting up on macOS..."
    echo ""

    # Check for Homebrew
    if ! command_exists brew; then
        print_warning "Homebrew not found"
        print_info "Install from: https://brew.sh"
        print_info "Then run: brew install whisper-cpp"
        setup_from_submodule
        return
    fi

    print_success "Homebrew found"
    echo ""

    # Try installing whisper-cpp via Homebrew
    print_info "Attempting to install whisper-cpp via Homebrew..."
    if brew install whisper-cpp 2>/dev/null || brew upgrade whisper-cpp 2>/dev/null; then
        print_success "Installed whisper-cpp via Homebrew"
        echo ""
        print_info "Whisper library location:"
        brew --prefix whisper-cpp
        echo ""
    else
        print_warning "Failed to install via Homebrew, using git submodule instead..."
        setup_from_submodule
    fi
}

setup_linux() {
    echo ""
    print_info "Setting up on Linux..."
    echo ""

    # Detect Linux distribution
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS_NAME=$NAME
    else
        OS_NAME="Unknown Linux"
    fi

    print_info "Detected: $OS_NAME"
    echo ""

    # Check for build tools
    if ! command_exists cmake; then
        print_warning "CMake not found"
        print_info "Install with:"
        if [[ "$OS_NAME" == *"Ubuntu"* ]] || [[ "$OS_NAME" == *"Debian"* ]]; then
            echo "  sudo apt-get install cmake build-essential"
        elif [[ "$OS_NAME" == *"Fedora"* ]] || [[ "$OS_NAME" == *"Red Hat"* ]]; then
            echo "  sudo dnf install cmake gcc g++ make"
        else
            echo "  Please install CMake and build tools for your distribution"
        fi
        exit 1
    fi
    print_success "CMake found"

    if ! command_exists gcc && ! command_exists clang; then
        print_error "C compiler not found"
        exit 1
    fi
    print_success "C compiler found"

    echo ""
    setup_from_submodule
}

setup_windows() {
    echo ""
    print_info "Setting up on Windows..."
    echo ""

    print_warning "Windows setup requires manual steps:"
    echo ""
    print_info "1. Install Visual Studio Build Tools with C++ support"
    echo "   Download from: https://visualstudio.microsoft.com/downloads/"
    echo ""
    print_info "2. Install CMake"
    echo "   Download from: https://cmake.org/download/"
    echo ""
    print_info "3. Run this script again, or manually execute:"
    echo "   git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp"
    echo "   git submodule update --init --recursive"
    echo ""

    setup_from_submodule
}

setup_from_submodule() {
    echo ""
    print_info "Setting up from git submodule..."
    echo ""

    if [ -d "vendor/whisper.cpp" ]; then
        print_warning "vendor/whisper.cpp already exists"
        print_info "Updating existing submodule..."
        git submodule update --init --recursive
    else
        print_info "Cloning whisper.cpp repository..."
        git submodule add https://github.com/ggerganov/whisper.cpp vendor/whisper.cpp
        git submodule update --init --recursive
    fi

    if [ -d "vendor/whisper.cpp" ]; then
        print_success "Whisper.cpp repository ready"
    else
        print_error "Failed to set up whisper.cpp submodule"
        exit 1
    fi
}

build_orangenote() {
    echo ""
    print_info "Building OrangeNote CLI with whisper support..."
    echo ""

    if cargo build --features whisper --release; then
        print_success "Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
}

verify_installation() {
    echo ""
    print_info "Verifying installation..."
    echo ""

    if ./target/release/orangenote-cli info 2>/dev/null | grep -q "Whisper support: ✓"; then
        print_success "Whisper support verified!"
        echo ""
        print_info "Next steps:"
        echo ""
        echo "1. Download a model:"
        echo "   ./target/release/orangenote-cli model download base"
        echo ""
        echo "2. Test transcription:"
        echo "   ./target/release/orangenote-cli transcribe test_audio.wav"
        echo ""
        echo "3. See more options:"
        echo "   ./target/release/orangenote-cli transcribe --help"
        echo ""
    else
        print_warning "Whisper support not detected"
        print_info "Try rebuilding or check WHISPER_SETUP_GUIDE.md for troubleshooting"
    fi
}

# Run main
main
