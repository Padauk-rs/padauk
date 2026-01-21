#!/bin/bash
set -e # Exit on error

SDK_ROOT=$(pwd)
CORE_CRATE="$SDK_ROOT/padauk"
ANDROID_LIB_DIR="$SDK_ROOT/android"
KOTLIN_OUT_DIR="$ANDROID_LIB_DIR/padauk/src/main/java"

echo "🔄 Step 1: Building Rust Library Metadata..."
cargo build -p padauk --release

# Find the rlib/a file
LIB_PATH="$SDK_ROOT/target/release/libpadauk.dylib"

echo "🔄 Step 2: Generating Kotlin Bindings..."
# Use internal padauk-cli or uniffi-bindgen CLI
cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library "$LIB_PATH" \
    --language kotlin \
    --out-dir "$KOTLIN_OUT_DIR" \
    --no-format

# Fix package naming if necessary (move from uniffi/padauk to com/padauk/core)
# Assuming uniffi.toml handles the package name 'com.padauk.core'

echo "🔄 Step 3: Compiling Android Archive (.aar)..."
cd "$ANDROID_LIB_DIR"
./gradlew assembleRelease

echo "✨ Success! AAR generated at:"
echo "$ANDROID_LIB_DIR/padauk/build/outputs/aar/padauk-release.aar"

AAR_SOURCE="$ANDROID_LIB_DIR/padauk/build/outputs/aar/padauk-release.aar"
ASSETS_DIR="$SDK_ROOT/padauk/assets/android"

echo "🚚 Moving AAR to CLI assets..."
cp "$AAR_SOURCE" "$ASSETS_DIR/padauk-release.aar"

echo "✅ CLI is now armed with the latest framework AAR."

echo "🔄 Building again to embed generated aar file"
cargo build -p padauk --release
