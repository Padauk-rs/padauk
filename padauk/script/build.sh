#!/usr/bin/env bash
set -euo pipefail

SDK_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CORE_CRATE="$SDK_ROOT/padauk"
ANDROID_LIB_DIR="$SDK_ROOT/android"
KOTLIN_OUT_DIR="$ANDROID_LIB_DIR/padauk/src/main/java"
IOS_OUT_DIR="$SDK_ROOT/ios"
FRAMEWORK_NAME="Padauk"
XCFRAMEWORK_DIR="$IOS_OUT_DIR/Frameworks"
WEB_OUT_DIR="$CORE_CRATE/assets/web"

BUILD_MODE="release"
PLATFORM="all"
PLATFORM_SET=0

usage() {
  cat <<USAGE
Usage: ./script/build.sh [--debug] [platform]

Build Padauk framework assets.

Options:
  --debug            Build using debug profile. Default is release.
  -h, --help         Show this help message.

Platforms:
  all                Build android, ios, and web (default)
  android            Build Android assets only
  ios                Build iOS assets only
  web                Build web assets only
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug)
      BUILD_MODE="debug"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    android|ios|web|all)
      if [[ "$PLATFORM_SET" -eq 1 ]]; then
        echo "❌ Platform already specified as '$PLATFORM'."
        usage
        exit 1
      fi
      PLATFORM="$1"
      PLATFORM_SET=1
      shift
      ;;
    *)
      echo "❌ Unknown argument: $1"
      usage
      exit 1
      ;;
  esac
done

TARGET_DIR_NAME="release"
if [[ "$BUILD_MODE" == "debug" ]]; then
  TARGET_DIR_NAME="debug"
fi

run_cargo_build() {
  local args=(-p padauk)
  if [[ "$BUILD_MODE" == "release" ]]; then
    args+=(--release)
  fi
  cargo build "${args[@]}" "$@"
}

run_uniffi_generate() {
  cargo run --features=uniffi/cli --bin uniffi-bindgen generate "$@"
}

is_macos() {
  [[ "$(uname -s)" == "Darwin" ]]
}

build_android() {
  echo "🔄 [Android] Building Rust library metadata ($BUILD_MODE)..."
  run_cargo_build

  local lib_path="$SDK_ROOT/target/$TARGET_DIR_NAME/libpadauk.dylib"
  if [[ ! -f "$lib_path" ]]; then
    lib_path="$SDK_ROOT/target/$TARGET_DIR_NAME/libpadauk.so"
  fi

  if [[ ! -f "$lib_path" ]]; then
    echo "❌ [Android] Could not find libpadauk dynamic library for UniFFI in target/$TARGET_DIR_NAME."
    exit 1
  fi

  echo "🔄 [Android] Generating Kotlin bindings..."
  run_uniffi_generate \
    --library "$lib_path" \
    --language kotlin \
    --out-dir "$KOTLIN_OUT_DIR" \
    --no-format

  local assets_dir="$CORE_CRATE/assets/android"
  local zip_out="$assets_dir/padauk-android.zip"

  echo "🔄 [Android] Building Android library module ($BUILD_MODE)..."
  (
    cd "$ANDROID_LIB_DIR"
    if [[ "$BUILD_MODE" == "debug" ]]; then
      ./gradlew assembleDebug
    else
      ./gradlew assembleRelease
    fi
  )

  echo "🔄 [Android] Packaging Android library module..."
  mkdir -p "$assets_dir"
  rm -f "$zip_out"

  (
    cd "$ANDROID_LIB_DIR"
    zip -r "$zip_out" padauk \
      -x "padauk/build/*" \
      -x "padauk/.gradle/*"
  )

  echo "✅ [Android] Packaged asset: $zip_out"
}

build_ios() {
  if ! is_macos; then
    echo "ℹ️  [iOS] Skipping iOS build because host OS is not macOS."
    return
  fi

  echo "🍎 [iOS] Building Rust targets ($BUILD_MODE)..."
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
  run_cargo_build --target aarch64-apple-ios
  run_cargo_build --target aarch64-apple-ios-sim
  run_cargo_build --target x86_64-apple-ios

  echo "🍎 [iOS] Creating universal simulator library..."
  mkdir -p "$SDK_ROOT/target/universal-sim/$TARGET_DIR_NAME"
  lipo -create \
    "$SDK_ROOT/target/aarch64-apple-ios-sim/$TARGET_DIR_NAME/libpadauk.a" \
    "$SDK_ROOT/target/x86_64-apple-ios/$TARGET_DIR_NAME/libpadauk.a" \
    -output "$SDK_ROOT/target/universal-sim/$TARGET_DIR_NAME/libpadauk.a"

  echo "🍎 [iOS] Generating Swift bindings..."
  run_uniffi_generate \
    --library "$SDK_ROOT/target/aarch64-apple-ios/$TARGET_DIR_NAME/libpadauk.a" \
    --language swift \
    --out-dir "$IOS_OUT_DIR/Generated" \
    --no-format

  echo "📦 [iOS] Creating XCFramework..."
  rm -rf "$XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"
  xcodebuild -create-xcframework \
    -library "$SDK_ROOT/target/aarch64-apple-ios/$TARGET_DIR_NAME/libpadauk.a" \
    -headers "$IOS_OUT_DIR/Generated" \
    -library "$SDK_ROOT/target/universal-sim/$TARGET_DIR_NAME/libpadauk.a" \
    -headers "$IOS_OUT_DIR/Generated" \
    -output "$XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"

  echo "🤐 [iOS] Zipping XCFramework..."
  mkdir -p "$CORE_CRATE/assets/ios"
  (
    cd "$XCFRAMEWORK_DIR"
    zip -r -X "$CORE_CRATE/assets/ios/Padauk.xcframework.zip" "$FRAMEWORK_NAME.xcframework"
  )

  echo "✅ [iOS] Packaged asset: $CORE_CRATE/assets/ios/Padauk.xcframework.zip"
}

build_web() {
  echo "🌐 [Web] Building wasm target ($BUILD_MODE)..."
  rustup target add wasm32-unknown-unknown
  run_cargo_build --target wasm32-unknown-unknown

  mkdir -p "$WEB_OUT_DIR"
  cp "$SDK_ROOT/target/wasm32-unknown-unknown/$TARGET_DIR_NAME/libpadauk.rlib" "$WEB_OUT_DIR/libpadauk.rlib"

  echo "✅ [Web] Exported wasm build artifact: $WEB_OUT_DIR/libpadauk.rlib"
}

if [[ "$PLATFORM" == "all" || "$PLATFORM" == "android" ]]; then
  build_android
fi

if [[ "$PLATFORM" == "all" || "$PLATFORM" == "ios" ]]; then
  build_ios
fi

if [[ "$PLATFORM" == "all" || "$PLATFORM" == "web" ]]; then
  build_web
fi

echo "🔄 Building again to embed generated assets ($BUILD_MODE)..."
run_cargo_build --features embed-assets

echo "✅ Done."
