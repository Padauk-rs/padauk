use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};
use include_dir::{Dir, include_dir};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};
use uniffi_bindgen::bindings::{GenerateOptions, TargetLanguage};

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
    prepare_gradle().expect("Failed setting necessary permission to android ./gradlew");

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
        .args(["build", "--target", rust_target])
        .current_dir("./rust")
        .status()
        .expect("Failed to build Rust library");

    if status.success() {
        // 4. Sync assets (we pass the detected abi so we know which jniLibs folder to use)
        sync_from_crate_source();
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
    let so_name = "librust.so";

    // Rust target folder (e.g., target/aarch64-linux-android/debug)
    let so_path = project_root
        .join("rust/target")
        .join(rust_target)
        .join("debug")
        .join(&so_name);

    // Correct Android JNI folder (e.g., jniLibs/arm64-v8a)
    let dst_dir = project_root.join("android/app/src/main/jniLibs").join(abi);
    let dst_so = dst_dir.join("libpadauk.so");

    fs::create_dir_all(&dst_dir).unwrap();
    fs::copy(&so_path, &dst_so).expect("Failed to sync .so binary");

    // 2. Path where Kotlin should go
    let kotlin_out = project_root.join("android/app/src/main/kotlin");
    println!("  ⚙️ Generating Kotlin bindings from binary...");

    // Generate bindings using the embedded logic
    run_internal_bindgen(dst_so.to_path_buf(), kotlin_out.to_path_buf());
}

fn sync_from_crate_source() {
    // 1. Run 'cargo metadata' to find the path of the 'padauk' dependency
    let project_root = std::env::current_dir().unwrap();
    let manifest_path = project_root.join("rust/Cargo.toml");
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--manifest-path",
            manifest_path
                .to_str()
                .expect("Failed to get Cargo.toml path."),
        ]) // --no-deps makes it much faster
        .output()
        .expect("Failed to execute cargo command");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Cargo Metadata Error: {}", error_message);
        std::process::exit(1);
    }

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata JSON");

    // 2. Find the 'padauk' package in the dependency graph
    let padauk_pkg = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "padauk")
        .ok_or_else(|| {
            // Log the actual packages found to help debug
            let found = metadata["packages"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p["name"].as_str().unwrap())
                .collect::<Vec<_>>();
            format!("Crate 'padauk' not found. Available: {:?}", found)
        })
        .expect("Failed to read native resources from Padauk framework.");

    let crate_root: PathBuf = PathBuf::from(padauk_pkg["manifest_path"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_path_buf();

    // 3. Copy the pre-baked native files to the Android project
    let native_src = crate_root.join("generated/android");
    let android_dest = PathBuf::from("android/app/src/main/kotlin/rs/padauk/core");

    fs::create_dir_all(&android_dest).unwrap();

    for entry in fs::read_dir(native_src).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), android_dest.join(entry.file_name())).unwrap();
    }

    println!(
        "✅ Native Renderers synced from padauk v{}",
        padauk_pkg["version"]
    );
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

fn run_internal_bindgen(library_path: PathBuf, out_dir: PathBuf) {
    println!("  ⚙️  Generating Kotlin bindings (Internal)...");

    let original_dir = env::current_dir().expect("Current dir");
    let rust_dir = original_dir.join("rust");

    // 1. We specify Kotlin as the target
    let target_lang = TargetLanguage::Kotlin;

    let lib_utf8 = Utf8PathBuf::from_path_buf(library_path).expect("Invalid UTF-8 path for input");
    let out_utf8 =
        Utf8PathBuf::from_path_buf(out_dir.clone()).expect("Invalid UTF-8 path for output");

    env::set_current_dir(&rust_dir).expect("Changed to rust dir.");

    // 2. We call the generation logic directly
    // This looks inside the .so file for the 'uniffi_metadata' section
    match uniffi_bindgen::bindings::generate(GenerateOptions {
        languages: vec![target_lang],
        source: lib_utf8,
        out_dir: out_utf8,
        config_override: Some(Utf8PathBuf::from_str("uniffi.toml").expect("Config file.")),
        format: false,
        crate_filter: None,
        metadata_no_deps: true,
    }) {
        Ok(_) => println!("  ✅ Bindings generated successfully."),
        Err(e) => {
            eprintln!("  ❌ Failed to generate bindings: {}", e);
            std::process::exit(1);
        }
    }

    env::set_current_dir(original_dir).expect("Change back to project folder.");

    // 4. Cleanup Padauk.kt if it was generated
    let unwanted = out_dir
        .join("rs")
        .join("padauk")
        .join("app")
        .join("padauk.kt");
    if unwanted.exists() {
        let _ = std::fs::remove_file(unwanted);
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

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn prepare_gradle() -> anyhow::Result<()> {
    let project_root = std::env::current_dir().unwrap();
    let gradlew_path = project_root.join("android/gradlew");

    // Only applies to macOS/Linux
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&gradlew_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&gradlew_path, perms)?;
        println!("🔐 Set executable permissions for gradlew");
    }

    Ok(())
}
