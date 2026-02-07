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

ASSETS_DIR="$SDK_ROOT/padauk/assets/android"
ZIP_OUT="$ASSETS_DIR/padauk-android.zip"

echo "🔄 Step 3: Building Android library (verification)..."
cd "$ANDROID_LIB_DIR"
./gradlew assembleRelease

echo "🔄 Step 4: Packaging Android library module..."
mkdir -p "$ASSETS_DIR"
rm -f "$ZIP_OUT"

(
  cd "$ANDROID_LIB_DIR"
  zip -r "$ZIP_OUT" padauk \
    -x "padauk/build/*" \
    -x "padauk/.gradle/*"
)

echo "✅ CLI is now armed with the latest Android module zip:"
echo "$ZIP_OUT"

echo "🔄 Building again to embed generated Android module zip"
cargo build -p padauk --release --features embed-assets
