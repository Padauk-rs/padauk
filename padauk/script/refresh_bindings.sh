#!/bin/bash

# 1. Path setup
SDK_ROOT=$(pwd)
CORE_CRATE="$SDK_ROOT/padauk"
OUTPUT_DIR="$SDK_ROOT/padauk/generated/android"

echo "🔄 Refreshing Core Padauk Bindings..."

# 2. Build the Rust crate to ensure it's valid
cargo build -p padauk

# 3. Run uniffi-bindgen to generate the Kotlin file
# We use the library-mode so it finds the UDL/Macros automatically
cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/debug/libpadauk.dylib \
    --language kotlin \
    --config uniffi.toml \
    --out-dir "$OUTPUT_DIR" \
    --no-format

# 4. Cleanup (UniFFI sometimes creates nested folders like uniffi/padauk/Padauk.kt)
# We want Padauk.kt to sit directly in native/android for the CLI to find it easily.
if [ -f "$OUTPUT_DIR/rs/padauk/core/Padauk.kt" ]; then
    mv "$OUTPUT_DIR/rs/padauk/core/Padauk.kt" "$OUTPUT_DIR/Padauk.kt"
    rm -rf "$OUTPUT_DIR/rs"
fi

echo "✨ Success! Padauk.kt is now updated in $OUTPUT_DIR"