use camino::Utf8Path;
use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};
use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

static PROJECT_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

#[derive(Parser)]
#[command(name = "padauk")]
#[command(about = "The Padauk SDK CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Padauk project
    Create { name: String },
    /// Run the app on a device
    Run { platform: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name } => {
            create_project(name);
        }
        Commands::Run { platform } => {
            if platform == "android" {
                run_android();
            }
        }
    }
}

fn create_project(name: &str) {
    println!("🌳 Planting a new project: {}...", name);

    let project_path = std::env::current_dir().unwrap().join(name);

    // 1. Unpack the embedded template
    PROJECT_TEMPLATE.extract(&project_path).unwrap();

    // 2. Personalize the Cargo.toml
    let cargo_path = project_path.join("rust/Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_path)
        .unwrap()
        .replace("{{PROJECT_NAME}}", name);
    fs::write(cargo_path, cargo_content).unwrap();

    println!("🌳 Padauk project '{}' is ready!", name);
}

fn run_android() {
    // 1. Pick the device first
    let device_serial = pick_android_device();

    // 2. Detect the ABI and Map to Rust Target
    let abi = get_device_abi(&device_serial);
    let rust_target =
        map_abi_to_target(&abi).expect(&format!("Unsupported Android architecture: {}", abi));

    println!("🎯 Target detected: {} (Device: {})", rust_target, abi);

    // 3. Compile Rust for the SPECIFIC target
    println!("🏗️  Building app for {}...", rust_target);
    let status = Command::new("cargo")
        .args(["build", "--target", rust_target, "--release"])
        .current_dir("./rust")
        .status()
        .expect("Failed to build Rust library");

    if status.success() {
        // 4. Sync assets (we pass the detected abi so we know which jniLibs folder to use)
        sync_assets(rust_target, &abi);

        // 5. Run on the specific device
        println!("📲 Installing on {}...", device_serial);

        // We can pass the serial to Gradle so it targets the right device
        Command::new("./gradlew")
            .args([
                "installDebug",
                &format!(
                    "-Pandroid.testInstrumentationRunnerArguments.serial={}",
                    device_serial
                ),
            ])
            // Or more simply, use adb directly to install the generated APK
            .current_dir("./android")
            .status()
            .expect("Failed to run Android app");

        // 4. Start the Activity via ADB
        let adb = get_adb_path(); // Resolve the path
        Command::new(adb)
            .args([
                "-s",
                &device_serial,
                "shell",
                "am",
                "start",
                "-n",
                "com.example.padauk/com.example.padauk.MainActivity",
            ])
            .status()
            .unwrap();
    }
}

fn sync_assets(rust_target: &str, abi: &str) {
    let project_root = std::env::current_dir().unwrap();
    let so_name = "libpadauk.so";

    // Rust target folder (e.g., target/aarch64-linux-android/release)
    let so_path = project_root
        .join("app/target")
        .join(rust_target)
        .join("release")
        .join(&so_name);

    // Correct Android JNI folder (e.g., jniLibs/arm64-v8a)
    let dst_dir = project_root.join("android/app/src/main/jniLibs").join(abi);

    fs::create_dir_all(&dst_dir).unwrap();
    fs::copy(&so_path, dst_dir.join(&so_name)).expect("Failed to sync .so binary");

    // 2. Path where Kotlin should go
    let kotlin_out = project_root.join("android/app/src/main/kotlin");
    println!("  ⚙️ Generating Kotlin bindings from binary...");

    // Generate bindings using the embedded logic
    run_internal_bindgen(&so_path, &kotlin_out);
}

#[derive(Debug)]
struct AndroidDevice {
    serial: String,
    model: String,
}

impl std::fmt::Display for AndroidDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.model, self.serial)
    }
}

fn get_android_devices() -> Vec<AndroidDevice> {
    let adb = get_adb_path(); // Resolve the path

    let output = Command::new(adb).args(["devices", "-l"]).output().expect(
        "❌ Error: Could not find 'adb'. Please set your ANDROID_HOME environment variable.",
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    // Skip the first line ("List of devices attached")
    for line in stdout.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            let serial = parts[0].to_string();
            // Find the part that starts with "model:"
            let model = parts
                .iter()
                .find(|p| p.starts_with("model:"))
                .map(|p| p.replace("model:", ""))
                .unwrap_or_else(|| "Unknown Device".to_string());

            devices.push(AndroidDevice { serial, model });
        }
    }
    devices
}

fn pick_android_device() -> String {
    let devices = get_android_devices();

    if devices.is_empty() {
        println!("❌ No devices found. Please connect a phone or start an emulator.");
        std::process::exit(1);
    }

    if devices.len() == 1 {
        println!("📱 Using only available device: {}", devices[0]);
        return devices[0].serial.clone();
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a device to run on")
        .items(&devices)
        .default(0)
        .interact()
        .unwrap();

    devices[selection].serial.clone()
}

fn map_abi_to_target(abi: &str) -> Option<&'static str> {
    match abi.trim() {
        "arm64-v8a" => Some("aarch64-linux-android"),
        "x86_64" => Some("x86_64-linux-android"),
        "armeabi-v7a" => Some("armv7-linux-androideabi"),
        "x86" => Some("i686-linux-android"),
        _ => None,
    }
}

fn get_device_abi(serial: &str) -> String {
    let adb = get_adb_path(); // Resolve the path

    let output = Command::new(adb)
        .args(["-s", serial, "shell", "getprop", "ro.product.cpu.abi"])
        .output()
        .expect("Failed to query device ABI");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

use uniffi_bindgen::bindings::TargetLanguage;
use uniffi_bindgen::library_mode::generate_bindings;

fn run_internal_bindgen(library_path: &Path, out_dir: &Path) {
    println!("  ⚙️  Generating Kotlin bindings (Internal)...");

    // 1. We specify Kotlin as the target
    let target_lang = TargetLanguage::Kotlin;

    let lib_utf8 = Utf8Path::new(
        library_path
            .to_str()
            .expect("Invalid UTF-8 path for library"),
    );
    let out_utf8 = Utf8Path::new(out_dir.to_str().expect("Invalid UTF-8 path for output"));

    // 2. We call the generation logic directly
    // This looks inside the .so file for the 'uniffi_metadata' section
    match generate_bindings(
        lib_utf8,           // Path to libpadauk_app.so
        None,               // Optional crate name override
        &vec![target_lang], // Languages to generate
        out_utf8,           // Where to save the .kt files
        // None,              // config_file_path
        false, // try_format_code
    ) {
        Ok(_) => println!("  ✅ Bindings generated successfully."),
        Err(e) => {
            eprintln!("  ❌ Failed to generate bindings: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_adb_path() -> PathBuf {
    // 1. Check ANDROID_HOME environment variable
    if let Ok(android_home) = env::var("ANDROID_HOME") {
        let adb_path = PathBuf::from(android_home)
            .join("platform-tools")
            .join("adb");
        if adb_path.exists() {
            return adb_path;
        }
    }

    // 2. Fallback: Check ANDROID_SDK_ROOT (older naming convention)
    if let Ok(sdk_root) = env::var("ANDROID_SDK_ROOT") {
        let adb_path = PathBuf::from(sdk_root).join("platform-tools").join("adb");
        if adb_path.exists() {
            return adb_path;
        }
    }

    // 3. Last Resort: Just return "adb" and hope it's in the system PATH
    PathBuf::from("adb")
}
