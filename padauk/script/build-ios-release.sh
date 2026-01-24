#!/bin/bash
set -e

SDK_ROOT=$(pwd)
CORE_CRATE="$SDK_ROOT/padauk"
IOS_OUT_DIR="$SDK_ROOT/ios"
FRAMEWORK_NAME="Padauk"
XCFRAMEWORK_DIR="$IOS_OUT_DIR/Frameworks"

# 1. Build for targets
echo "🍎 Building Rust for iOS targets..."
# Physical iPhone
rustup target add aarch64-apple-ios
cargo build -p padauk --release --target aarch64-apple-ios

# Simulator (Universal arm64 + x86_64)
rustup target add aarch64-apple-ios-sim x86_64-apple-ios
cargo build -p padauk --release --target aarch64-apple-ios-sim
cargo build -p padauk --release --target x86_64-apple-ios

# 2. Create Universal Simulator Library using lipo
echo "🍎 Creating Universal Simulator Library..."
mkdir -p "$SDK_ROOT/target/universal-sim/release"
lipo -create \
    "$SDK_ROOT/target/aarch64-apple-ios-sim/release/libpadauk.a" \
    "$SDK_ROOT/target/x86_64-apple-ios/release/libpadauk.a" \
    -output "$SDK_ROOT/target/universal-sim/release/libpadauk.a"

# 3. Generate Swift/C Bindings
echo "🍎 Generating Bindings..."
cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library "$SDK_ROOT/target/aarch64-apple-ios/release/libpadauk.a" \
    --language swift \
    --out-dir "$IOS_OUT_DIR/Generated" \
    --no-format

# 4. Package as XCFramework
echo "📦 Creating XCFramework..."
rm -rf "$XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"

xcodebuild -create-xcframework \
    -library "$SDK_ROOT/target/aarch64-apple-ios/release/libpadauk.a" \
    -headers "$IOS_OUT_DIR/Generated" \
    -library "$SDK_ROOT/target/universal-sim/release/libpadauk.a" \
    -headers "$IOS_OUT_DIR/Generated" \
    -output "$XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"

echo "✅ XCFramework created at $XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"

# 5. Zip the XCFramework for embedding
echo "🤐 Zipping XCFramework..."
ASSETS_DIR="$CORE_CRATE/assets/ios"
mkdir -p "$ASSETS_DIR"

# Navigate to the frameworks directory to avoid nesting folders in the zip
cd "$XCFRAMEWORK_DIR"
zip -r -X "$ASSETS_DIR/Padauk.xcframework.zip" "$FRAMEWORK_NAME.xcframework"

echo "✅ iOS Framework zipped and moved to CLI assets."

echo "🔄 Building again to embed generated xcframework file"
cargo build -p padauk --release --features embed-assets
