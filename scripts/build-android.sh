#!/bin/bash

# Build script for JARVIS Android

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

echo "======================================"
echo "JARVIS Android Build Script"
echo "======================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    # Check Android targets
    if ! rustup target list --installed | grep -q "aarch64-linux-android"; then
        print_warn "Android targets not installed. Installing..."
        rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
    fi
    
    # Check Java
    if ! command -v java &> /dev/null; then
        print_error "Java is not installed. Please install JDK 17 or later."
        exit 1
    fi
    
    # Check Android SDK
    if [ -z "$ANDROID_HOME" ] && [ -z "$ANDROID_SDK_ROOT" ]; then
        print_warn "ANDROID_HOME or ANDROID_SDK_ROOT not set"
        print_info "Please set ANDROID_HOME to your Android SDK path"
        print_info "Example: export ANDROID_HOME=$HOME/Android/Sdk"
    fi
    
    print_info "Prerequisites check complete!"
}

# Download Vosk models
download_models() {
    print_info "Checking Vosk models..."
    
    if [ ! -d "$PROJECT_ROOT/resources/vosk" ] || [ -z "$(ls -A "$PROJECT_ROOT/resources/vosk" 2>/dev/null)" ]; then
        print_info "Downloading Vosk models..."
        bash "$SCRIPT_DIR/download-vosk-models.sh"
    else
        print_info "Vosk models already present"
    fi
}

# Build for Android
build_android() {
    print_info "Building JARVIS for Android..."
    
    cd "$PROJECT_ROOT/crates/jarvis-gui"
    
    # Check if Tauri CLI is installed
    if ! command -v cargo-tauri &> /dev/null; then
        print_warn "cargo-tauri not found. Installing..."
        cargo install tauri-cli --version "^2.0"
    fi
    
    # Initialize Android project if not exists
    if [ ! -d "$PROJECT_ROOT/crates/jarvis-gui/gen/android" ]; then
        print_info "Initializing Android project..."
        cargo tauri android init
    fi
    
    # Build debug APK
    print_info "Building debug APK..."
    cargo tauri android build --debug
    
    print_info "Build complete!"
    print_info "APK location: $PROJECT_ROOT/crates/jarvis-gui/gen/android/app/build/outputs/apk/"
}

# Install on device
install_device() {
    print_info "Installing on connected device..."
    
    # Check if adb is available
    if ! command -v adb &> /dev/null; then
        print_error "adb not found. Please install Android Platform Tools."
        return 1
    fi
    
    # Check if device is connected
    if ! adb devices | grep -q "device$"; then
        print_error "No Android device connected. Please connect a device or start an emulator."
        return 1
    fi
    
    APK_PATH="$PROJECT_ROOT/crates/jarvis-gui/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
    
    if [ ! -f "$APK_PATH" ]; then
        print_error "APK not found at $APK_PATH"
        print_info "Please build first with: ./build-android.sh build"
        return 1
    fi
    
    adb install -r "$APK_PATH"
    print_info "App installed successfully!"
    
    # Launch the app
    adb shell am start -n com.jarvis.app/.MainActivity
    print_info "App launched!"
}

# Run development server
run_dev() {
    print_info "Starting development server..."
    
    cd "$PROJECT_ROOT/crates/jarvis-gui"
    cargo tauri android dev
}

# Clean build artifacts
clean() {
    print_info "Cleaning build artifacts..."
    
    cd "$PROJECT_ROOT"
    cargo clean
    
    if [ -d "$PROJECT_ROOT/crates/jarvis-gui/gen/android" ]; then
        rm -rf "$PROJECT_ROOT/crates/jarvis-gui/gen/android/app/build"
    fi
    
    print_info "Clean complete!"
}

# Show help
show_help() {
    echo "Usage: ./build-android.sh [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build       Build the Android APK (default)"
    echo "  install     Install APK on connected device"
    echo "  dev         Run development server with hot reload"
    echo "  clean       Clean build artifacts"
    echo "  models      Download Vosk models"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./build-android.sh build    # Build APK"
    echo "  ./build-android.sh install  # Build and install"
    echo "  ./build-android.sh dev      # Development mode"
}

# Main
main() {
    case "${1:-build}" in
        build)
            check_prerequisites
            download_models
            build_android
            ;;
        install)
            check_prerequisites
            download_models
            build_android
            install_device
            ;;
        dev)
            check_prerequisites
            download_models
            run_dev
            ;;
        clean)
            clean
            ;;
        models)
            download_models
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
