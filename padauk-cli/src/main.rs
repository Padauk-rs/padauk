use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};
use serde_json::Value;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};
use uniffi_bindgen::bindings::{GenerateOptions, TargetLanguage};
use zip::ZipArchive;

mod assets;

static PROJECT_TEMPLATE: &[u8] = include_bytes!("../target/template.zip");

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
    Create {
        name: String,
    },
    /// Run the app on a device
    Run {
        /// Build and run a debug app (Android only)
        #[arg(long)]
        debug: bool,
        /// Wait for debugger attach on launch (Android only; implies --debug)
        #[arg(long)]
        wait_on_launch: bool,
        /// LLDB server port for Android debug
        #[arg(long, default_value_t = 5039)]
        debug_port: u16,
    },
    /// Start LLDB server for an already running Android app
    Debug {
        /// LLDB server port for Android debug
        #[arg(long, default_value_t = 5039)]
        debug_port: u16,
    },
    /// Build an APK for Android
    Build {
        platform: String,
        /// Comma-separated ABI list (e.g., arm64-v8a,x86_64). Defaults: release=arm64-v8a, debug=arm64-v8a,x86_64
        #[arg(long)]
        abi: Option<String>,
        #[arg(long)]
        release: bool,
    },
    // Generate Assets Constant
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name } => {
            create_project(name);
        }
        Commands::Run {
            debug,
            wait_on_launch,
            debug_port,
        } => {
            let mut debug = *debug;
            if *wait_on_launch && !debug {
                eprintln!("ℹ️  --wait-on-launch implies --debug; enabling debug build.");
                debug = true;
            }
            run_auto(debug, *wait_on_launch, *debug_port).unwrap();
        }
        Commands::Debug { debug_port } => {
            debug_android(*debug_port).unwrap();
        }
        Commands::Build {
            platform,
            release,
            abi,
        } => {
            if platform == "android" {
                build_android(*release, abi.as_deref()).unwrap();
            } else {
                eprintln!("❌ Unsupported platform: {}", platform);
            }
        }
        Commands::Generate => {
            sync_and_generate_assets().unwrap();
        }
    }
}

fn create_project(name: &str) {
    println!("🌳 Planting a new project: {}...", name);

    let project_path = std::env::current_dir().unwrap().join(name);

    // IMPORTANT: include_dir's extract() will fail if this path doesn't exist
    fs::create_dir_all(&project_path).unwrap();

    // 1. Unpack the embedded template
    extract_template(&project_path).unwrap();

    // 2. Personalize the Cargo.toml
    let cargo_path = project_path.join("rust/Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_path)
        .unwrap()
        .replace("{{PROJECT_NAME}}", name);
    fs::write(cargo_path, cargo_content).unwrap();

    println!("🌳 Padauk project '{}' is ready!", name);
}

pub fn extract_template(target_dir: &PathBuf) -> anyhow::Result<()> {
    // 1. Create the root project directory
    std::fs::create_dir_all(target_dir)?;

    // 2. Wrap our embedded bytes in a Cursor for the ZipArchive
    let reader = Cursor::new(PROJECT_TEMPLATE);
    let mut archive = ZipArchive::new(reader)?;

    // 3. Iterate through every file in the zip and extract it
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // On Unix, restore file permissions (very important for scripts!)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn run_android(
    device_serial: Option<String>,
    debug: bool,
    wait_on_launch: bool,
    debug_port: u16,
) {
    prepare_gradle().expect("Failed setting necessary permission to android ./gradlew");

    // 1. Pick the device first
    let device_serial = device_serial.unwrap_or_else(pick_android_device);
    let project_root = std::env::current_dir().unwrap();
    let app_id = get_android_application_id(&project_root);

    // 2. Detect the ABI and Map to Rust Target
    let abi = get_device_abi(&device_serial);
    let rust_target =
        map_abi_to_target(&abi).expect(&format!("Unsupported Android architecture: {}", abi));

    println!("🎯 Target detected: {} (Device: {})", rust_target, abi);

    let profile = if debug { "debug" } else { "release" };
    let gradle_task = if debug { "installDebug" } else { "installRelease" };

    // 3. Compile Rust for the SPECIFIC target
    println!("🏗️  Building app for {}...", rust_target);
    let mut cargo_args = vec!["build", "--target", rust_target];
    if !debug {
        cargo_args.push("--release");
    }
    let status = Command::new("cargo")
        .args(cargo_args)
        .current_dir("./rust")
        .status()
        .expect("Failed to build Rust library");

    if status.success() {
        // 4. Sync assets (we pass the detected abi so we know which jniLibs folder to use)
        sync_from_crate_source();
        sync_assets(rust_target, &abi, profile);

        // 5. Run on the specific device
        println!("📲 Installing on {}...", device_serial);

        // We can pass the serial to Gradle so it targets the right device
        Command::new("./gradlew")
            .args([
                gradle_task,
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
                &format!("{}/{}.MainActivity", app_id, app_id),
            ])
            .status()
            .unwrap();

        if debug && wait_on_launch {
            start_android_debugging(
                &project_root,
                &device_serial,
                &abi,
                &app_id,
                debug_port,
                wait_on_launch,
            );
        }
    }
}

fn run_auto(debug: bool, wait_on_launch: bool, debug_port: u16) -> anyhow::Result<()> {
    let project_root = std::env::current_dir().unwrap();
    let mut devices = Vec::new();

    let mut android_devices = get_android_devices();
    let mut ios_devices = get_available_simulators().unwrap_or_default();

    devices.append(&mut android_devices);
    devices.append(&mut ios_devices);

    if devices.is_empty() {
        anyhow::bail!(
            "No running devices found. Start an Android emulator or boot an iOS simulator, then retry."
        );
    }

    let selected_device: &Device;

    if devices.len() == 1 {
        selected_device = &devices[0];
    } else {
        let labels: Vec<String> = devices
            .iter()
            .map(|d| {
                let platform = if d.ios { "iOS" } else { "Android" };
                format!("{}: {} [{}]", platform, d.name, d.serial)
            })
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a device to run on")
            .items(&labels)
            .default(0)
            .interact()?;

        selected_device = &devices[selection];
    }

    if selected_device.ios {
        run_ios(&project_root, selected_device)?;
    } else {
        run_android(
            Some(selected_device.serial.clone()),
            debug,
            wait_on_launch,
            debug_port,
        );
    }

    Ok(())
}

fn debug_android(debug_port: u16) -> anyhow::Result<()> {
    let project_root = std::env::current_dir().unwrap();
    let devices = get_android_devices();

    if devices.is_empty() {
        anyhow::bail!(
            "No running Android emulator found. Start an emulator and try again."
        );
    }

    let selected_device: &Device;
    if devices.len() == 1 {
        selected_device = &devices[0];
    } else {
        let labels: Vec<String> = devices
            .iter()
            .map(|d| format!("Android: {} [{}]", d.name, d.serial))
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a device to attach to")
            .items(&labels)
            .default(0)
            .interact()?;

        selected_device = &devices[selection];
    }

    let app_id = get_android_application_id(&project_root);
    let abi = get_device_abi(&selected_device.serial);
    println!(
        "📱 Attaching to device: {} [{}] (ABI: {})",
        selected_device.name, selected_device.serial, abi
    );

    start_android_debugging(
        &project_root,
        &selected_device.serial,
        &abi,
        &app_id,
        debug_port,
        true,
    );

    Ok(())
}

fn start_android_debugging(
    project_root: &PathBuf,
    device_serial: &str,
    abi: &str,
    app_id: &str,
    debug_port: u16,
    wait_on_launch: bool,
) {
    let adb = get_adb_path();

    let ndk_dir = match find_ndk_dir() {
        Some(dir) => dir,
        None => {
            eprintln!(
                "❌ Android NDK not found. Set ANDROID_NDK_HOME (or ANDROID_HOME/ANDROID_SDK_ROOT)."
            );
            return;
        }
    };

    // If the device is 32-bit only, force 32-bit lldb-server.
    let abilist64 = get_prop(&adb, device_serial, "ro.product.cpu.abilist64");
    let mut lldb_abi = abi.to_string();
    if abi == "arm64-v8a" && abilist64.as_deref().unwrap_or("").is_empty() {
        lldb_abi = "armeabi-v7a".to_string();
    }

    let remote_path = "/data/local/tmp/lldb-server";
    let mut lldb_server = find_lldb_server(&ndk_dir, &lldb_abi);
    if lldb_server.is_none() && lldb_abi != "armeabi-v7a" {
        lldb_server = find_lldb_server(&ndk_dir, "armeabi-v7a");
    }
    let lldb_server = match lldb_server {
        Some(path) => path,
        None => {
            eprintln!(
                "❌ lldb-server not found in NDK. Ensure LLDB is installed with the NDK."
            );
            return;
        }
    };

    let primary = push_and_check_lldb_server(&adb, device_serial, &lldb_server, remote_path);
    if primary.is_err() && lldb_abi != "armeabi-v7a" {
        if let Some(fallback) = find_lldb_server(&ndk_dir, "armeabi-v7a") {
            let fallback_res =
                push_and_check_lldb_server(&adb, device_serial, &fallback, remote_path);
            if fallback_res.is_ok() {
                eprintln!(
                    "⚠️  64-bit lldb-server failed; using 32-bit fallback for debugging."
                );
            } else {
                eprintln!("❌ lldb-server failed to run on device.");
                if let Err(err) = primary {
                    eprintln!("{}", err);
                }
                if let Err(err) = fallback_res {
                    eprintln!("{}", err);
                }
                return;
            }
        } else {
            eprintln!("❌ lldb-server failed to run on device.");
            if let Err(err) = primary {
                eprintln!("{}", err);
            }
            return;
        }
    } else if let Err(err) = primary {
        eprintln!("❌ lldb-server failed to run on device.");
        eprintln!("{}", err);
        return;
    }

    // Forward LLDB port
    let port = debug_port.to_string();
    let _ = Command::new(&adb)
        .args([
            "-s",
            device_serial,
            "forward",
            &format!("tcp:{}", port),
            &format!("tcp:{}", port),
        ])
        .status();

    // Try to fetch PID for attach
    let pid = wait_for_android_pid(&adb, device_serial, app_id);

    if let Some(pid) = pid {
        println!("🔎 App PID: {}", pid);
        // Start lldb-server in gdbserver mode, attached to the app PID.
        // Prefer running via run-as to avoid exec restrictions on /data/local/tmp.
        let run_as_ok = Command::new(&adb)
            .args(["-s", device_serial, "shell", "run-as", app_id, "true"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if run_as_ok {
            let app_lldb_server = format!("/data/user/0/{}/lldb-server", app_id);
            let copy_ok = Command::new(&adb)
                .args([
                    "-s",
                    device_serial,
                    "shell",
                    "run-as",
                    app_id,
                    "cp",
                    remote_path,
                    &app_lldb_server,
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            let chmod_ok = Command::new(&adb)
                .args([
                    "-s",
                    device_serial,
                    "shell",
                    "run-as",
                    app_id,
                    "chmod",
                    "755",
                    &app_lldb_server,
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if !copy_ok || !chmod_ok {
                eprintln!("❌ Failed to prepare lldb-server in app sandbox.");
                return;
            }

            let _ = Command::new(&adb)
                .args([
                    "-s",
                    device_serial,
                    "shell",
                    "run-as",
                    app_id,
                    &app_lldb_server,
                    "gdbserver",
                    "--attach",
                    &pid,
                    &format!("127.0.0.1:{}", port),
                ])
                .spawn();
        } else {
            let _ = Command::new(&adb)
                .args([
                    "-s",
                    device_serial,
                    "shell",
                    remote_path,
                    "gdbserver",
                    "--attach",
                    &pid,
                    &format!("127.0.0.1:{}", port),
                ])
                .spawn();
        }
    } else {
        println!("❌ App PID not found. Launch the app and re-run this command.");
        return;
    }

    println!("🪲 LLDB gdbserver started on device (port {} forwarded).", port);

    if wait_on_launch {
        println!("🧩 LLDB server ready. Attach with your debugger.");
        return;
    }
    // Give the server a moment to start before connecting.
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Check if lldb-server is alive before attaching
    let server_pid = Command::new(&adb)
        .args(["-s", device_serial, "shell", "pidof", "-s", "lldb-server"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty());
    if server_pid.is_none() {
        let ps_out = Command::new(&adb)
            .args(["-s", device_serial, "shell", "ps", "-A"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_default();
        if ps_out.contains("lldb-server") {
            // Continue; some devices don't support pidof.
        } else {
            eprintln!("❌ lldb-server is not running. Try again or check device logs.");
            return;
        }
    }

    println!("🧩 Attaching LLDB...");
    let so_path = find_android_debug_so(project_root, abi);
    let so_dir = so_path
        .as_ref()
        .and_then(|p| p.parent())
        .map(|p| p.to_string_lossy().to_string());
    let mut attached = false;
    for _ in 0..5 {
        let mut cmd = Command::new("lldb");
        if let Some(path) = so_path.as_ref() {
            cmd.arg("-o").arg(format!("target create {}", path.to_string_lossy()));
        }
        if let Some(dir) = so_dir.as_ref() {
            cmd.arg("-o")
                .arg(format!("settings set target.exec-search-paths {}", dir));
        }
        cmd.arg("-o")
            .arg(format!("gdb-remote localhost:{}", port))
            .arg("-o")
            .arg("process handle SIGSTOP -n true -p true -s false")
            .arg("-o")
            .arg("process handle SIGSEGV -n true -p true -s false")
            .arg("-o")
            .arg("process handle SIGBUS -n true -p true -s false")
            .arg("-o")
            .arg("process continue");

        let status = cmd.status();
        if let Ok(status) = status {
            if status.success() {
                attached = true;
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !attached {
        eprintln!("❌ Failed to attach LLDB. Ensure the app is debuggable and try again.");
    }
}

fn get_android_application_id(project_root: &PathBuf) -> String {
    let build_file = project_root.join("android/app/build.gradle.kts");
    if let Ok(content) = fs::read_to_string(&build_file) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("applicationId") {
                if let Some(start) = trimmed.find('"') {
                    if let Some(end) = trimmed[start + 1..].find('"') {
                        return trimmed[start + 1..start + 1 + end].to_string();
                    }
                }
            }
        }
    }
    "com.example.padauk".to_string()
}

fn find_android_debug_so(project_root: &PathBuf, abi: &str) -> Option<PathBuf> {
    let merged_libs = project_root
        .join("android/app/build/intermediates/merged_native_libs/debug/mergeDebugNativeLibs/out/lib")
        .join(abi);
    let app_jni = project_root
        .join("android/app/src/main/jniLibs")
        .join(abi);

    let preferred = vec![
        merged_libs.join("libpadauk.so"),
        app_jni.join("libpadauk.so"),
    ];
    for path in preferred {
        if path.exists() {
            return Some(path);
        }
    }

    for dir in [merged_libs, app_jni] {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "so" {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

fn wait_for_android_pid(adb: &PathBuf, device_serial: &str, app_id: &str) -> Option<String> {
    for _ in 0..20 {
        let pid = Command::new(adb)
            .args(["-s", device_serial, "shell", "pidof", "-s", app_id])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty());

        if pid.is_some() {
            return pid;
        }

        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    None
}

fn get_prop(adb: &PathBuf, device_serial: &str, key: &str) -> Option<String> {
    Command::new(adb)
        .args(["-s", device_serial, "shell", "getprop", key])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn push_and_check_lldb_server(
    adb: &PathBuf,
    device_serial: &str,
    lldb_server: &PathBuf,
    remote_path: &str,
) -> Result<(), String> {
    let _ = Command::new(adb)
        .args(["-s", device_serial, "push"])
        .arg(lldb_server)
        .arg(remote_path)
        .status();
    let _ = Command::new(adb)
        .args(["-s", device_serial, "shell", "chmod", "755", remote_path])
        .status();

    // Sanity check: ensure lldb-server can execute on device
    let check = Command::new(adb)
        .args(["-s", device_serial, "shell", remote_path, "version"])
        .output();
    match check {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            let out_str = String::from_utf8_lossy(&out.stdout);
            Err(format!(
                "lldb-server failed to run: {}{}",
                out_str.trim(),
                if err.is_empty() { "" } else { "\n" }
            ) + err.trim())
        }
        Err(e) => Err(format!("lldb-server check failed: {}", e)),
    }
}

fn find_ndk_dir() -> Option<PathBuf> {
    for key in ["ANDROID_NDK_HOME", "ANDROID_NDK_ROOT", "ANDROID_NDK"] {
        if let Ok(val) = env::var(key) {
            let p = PathBuf::from(val);
            if p.exists() {
                return Some(p);
            }
        }
    }

    let sdk_root = env::var("ANDROID_HOME")
        .or_else(|_| env::var("ANDROID_SDK_ROOT"))
        .ok()
        .map(PathBuf::from)?;

    let ndk_root = sdk_root.join("ndk");
    if !ndk_root.exists() {
        return None;
    }

    let mut candidates: Vec<PathBuf> = fs::read_dir(ndk_root)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();

    candidates.sort();
    candidates.pop()
}

fn find_lldb_server(ndk_dir: &PathBuf, abi: &str) -> Option<PathBuf> {
    let linux_abi = match abi {
        "arm64-v8a" => "aarch64",
        "armeabi-v7a" => "arm",
        "x86_64" => "x86_64",
        "x86" => "i386",
        _ => "aarch64",
    };

    let prebuilt_dir = ndk_dir.join("toolchains/llvm/prebuilt");
    let host_dir = fs::read_dir(&prebuilt_dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .find(|p| p.is_dir())?;

    // Look for the highest clang version directory
    let clang_root = host_dir.join("lib/clang");
    let mut versions: Vec<PathBuf> = fs::read_dir(&clang_root)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    versions.sort();
    let clang_dir = versions.pop()?;

    let candidate = clang_dir
        .join("lib/linux")
        .join(linux_abi)
        .join("lldb-server");

    if candidate.exists() {
        return Some(candidate);
    }

    None
}

pub fn run_ios(project_root: &PathBuf, device: &Device) -> anyhow::Result<()> {
    // 1. Build Rust Static Library
    println!("🦀 Building Rust library for iOS...");
    Command::new("cargo")
        .args(["build", "--target", "aarch64-apple-ios-sim", "--release"])
        .current_dir(project_root.join("rust"))
        .status()?;

    // 2. Build and Install via xcodebuild
    println!("🍎 Building Xcode project for {}...", device.name);
    let project_name = "Template"; // TODO: Rename to Runner

    // 1. Build the .app bundle
    // We use -derivedDataPath to know exactly where the output goes
    let build_dir = project_root.join("ios/build");

    Command::new("xcodebuild")
        .args([
            "-project",
            &format!("ios/{}.xcodeproj", project_name),
            "-scheme",
            project_name,
            "-configuration",
            "Debug",
            "-destination",
            &format!("id={}", device.serial),
            "-derivedDataPath",
            build_dir.to_str().unwrap(),
            "build",
        ])
        .current_dir(project_root)
        .status()?;

    // 2. Locate the built .app file
    // The path usually looks like build/Build/Products/Debug-iphonesimulator/MyApp.app
    let app_path = build_dir.join(format!(
        "Build/Products/Debug-iphonesimulator/{}.app",
        project_name
    ));

    // 3. Manually install to the simulator
    println!("📲 Installing to simulator...");
    Command::new("xcrun")
        .args([
            "simctl",
            "install",
            &device.serial,
            app_path.to_str().unwrap(),
        ])
        .status()?;

    // TODO:  Replace 'com.example.Template' with the user's Bundle ID
    Command::new("xcrun")
        .args(["simctl", "launch", &device.serial, "rs.padauk.app"])
        .status()?;

    Ok(())
}

fn sync_assets(rust_target: &str, abi: &str, profile: &str) {
    let project_root = std::env::current_dir().unwrap();
    let so_name = "librust.so";

    // Rust target folder (e.g., target/aarch64-linux-android/debug)
    let target_dir = cargo_target_dir(&project_root).unwrap_or_else(|| project_root.join("rust/target"));
    let so_path = target_dir.join(rust_target).join(profile).join(&so_name);
    let debug_dir = so_path
        .parent()
        .expect("Rust target directory should exist")
        .to_path_buf();
    let debug_so = debug_dir.join("libpadauk.so");

    // Correct Android JNI folder (e.g., jniLibs/arm64-v8a)
    let dst_dir = project_root.join("android/app/src/main/jniLibs").join(abi);
    let dst_so = dst_dir.join("libpadauk.so");

    fs::create_dir_all(&dst_dir).unwrap();
    fs::copy(&so_path, &dst_so).expect("Failed to sync .so binary");
    // Also place a local libpadauk.so next to librust.so for debugger symbol resolution.
    let _ = fs::copy(&so_path, &debug_so);

    // 2. Path where Kotlin should go
    let kotlin_out = project_root.join("android/app/src/main/kotlin");
    println!("  ⚙️ Generating Kotlin bindings from binary...");

    // Generate bindings using the embedded logic
    run_internal_bindgen(dst_so.to_path_buf(), kotlin_out.to_path_buf());
}

fn cargo_target_dir(project_root: &Path) -> Option<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(project_root)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let metadata: Value = serde_json::from_str(&stdout).ok()?;
    metadata
        .get("target_directory")
        .and_then(|value| value.as_str())
        .map(PathBuf::from)
}

fn build_android(release: bool, abi_list: Option<&str>) -> anyhow::Result<()> {
    prepare_gradle().expect("Failed setting necessary permission to android ./gradlew");

    let profile = if release { "release" } else { "debug" };
    let gradle_task = if release {
        "assembleRelease"
    } else {
        "assembleDebug"
    };

    sync_from_crate_source();

    let abi_targets = resolve_abi_targets(release, abi_list)?;
    let abi_targets = filter_installed_targets(abi_targets, abi_list.is_some())?;

    for (abi, rust_target) in abi_targets {
        println!("🏗️  Building Rust for {} ({})...", rust_target, abi);
        let mut args = vec!["build", "--target", rust_target];
        if release {
            args.push("--release");
        }

        let status = Command::new("cargo")
            .args(args)
            .current_dir("./rust")
            .status()
            .expect("Failed to build Rust library");

        if !status.success() {
            anyhow::bail!("Rust build failed for target {}", rust_target);
        }

        sync_assets(rust_target, abi.as_str(), profile);
    }

    println!("📦 Building Android APK...");
    Command::new("./gradlew")
        .args([gradle_task])
        .current_dir("./android")
        .status()
        .expect("Failed to build Android APK");

    let project_root = std::env::current_dir().unwrap();
    let apk_path = project_root
        .join("android/app/build/outputs/apk")
        .join(profile)
        .join(format!("app-{}.apk", profile));

    println!("✅ APK generated at: {}", apk_path.display());

    Ok(())
}

fn resolve_abi_targets(
    release: bool,
    abi_list: Option<&str>,
) -> anyhow::Result<Vec<(String, &'static str)>> {
    let default_abis = if release {
        "arm64-v8a"
    } else {
        "arm64-v8a,x86_64"
    };

    let abis = abi_list.unwrap_or(default_abis);
    let mut targets = Vec::new();

    for abi in abis.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let target = match abi {
            "arm64-v8a" => "aarch64-linux-android",
            "x86_64" => "x86_64-linux-android",
            "armeabi-v7a" => "armv7-linux-androideabi",
            "x86" => "i686-linux-android",
            _ => anyhow::bail!(
                "Unsupported ABI '{}'. Supported: arm64-v8a, x86_64, armeabi-v7a, x86",
                abi
            ),
        };
        targets.push((abi.to_string(), target));
    }

    if targets.is_empty() {
        anyhow::bail!("No ABIs selected. Provide --abi or use defaults.");
    }

    Ok(targets)
}

fn filter_installed_targets(
    targets: Vec<(String, &'static str)>,
    strict: bool,
) -> anyhow::Result<Vec<(String, &'static str)>> {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();

    let installed: Vec<String> = match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        _ => Vec::new(),
    };

    if installed.is_empty() {
        // rustup not available; fall back to attempting all targets.
        return Ok(targets);
    }

    let mut filtered = Vec::new();
    let mut skipped = Vec::new();

    for (abi, target) in targets {
        if installed.iter().any(|t| t == target) {
            filtered.push((abi, target));
        } else {
            skipped.push((abi, target));
        }
    }

    if strict && !skipped.is_empty() {
        let mut msg = String::from("Missing Rust targets for selected ABIs:\n");
        for (abi, target) in &skipped {
            msg.push_str(&format!(
                "- {} (install: rustup target add {})\n",
                abi, target
            ));
        }
        anyhow::bail!("{}", msg.trim_end());
    }

    for (abi, target) in skipped {
        eprintln!(
            "⚠️  Skipping ABI {} (missing Rust target {}). Install with: rustup target add {}",
            abi, target, target
        );
    }

    if filtered.is_empty() {
        anyhow::bail!(
            "No installed Rust targets for the selected ABIs. Install targets via rustup."
        );
    }

    Ok(filtered)
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

    // 3. Extract the embedded Android library project into the app's android/ folder
    let native_src = crate_root.join("assets").join("android");
    let zip_path = native_src.join("padauk-android.zip");
    let android_dest = PathBuf::from("android");
    let android_build_dir = android_dest.join("build").join("padauk");

    if !zip_path.exists() {
        eprintln!(
            "❌ Missing embedded Android project zip: {}",
            zip_path.display()
        );
        std::process::exit(1);
    }

    // Always refresh the module to avoid version mismatches when the crate updates.
    if android_build_dir.exists() {
        let _ = fs::remove_dir_all(&android_build_dir);
    }

    extract_zip_file(&zip_path, &android_build_dir)
        .expect("Failed to extract Android project module.");

    // Ensure Gradle settings include the padauk module
    let settings_path = project_root.join("android/settings.gradle.kts");
    if let Ok(settings) = fs::read_to_string(&settings_path) {
        if !settings.contains("include(\":padauk\")") {
            let mut updated = settings;
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push_str(
                "include(\":padauk\")\nproject(\":padauk\").projectDir = file(\"build/padauk/padauk\")\n",
            );
            let _ = fs::write(&settings_path, updated);
        }
    }

    // Ensure app module depends on padauk (and remove AAR fileTree dependency)
    let app_build = project_root.join("android/app/build.gradle.kts");
    if let Ok(content) = fs::read_to_string(&app_build) {
        let lines: Vec<String> = content
            .lines()
            .filter(|l| !l.contains("fileTree(mapOf(\"dir\" to \"libs\""))
            .map(|l| l.to_string())
            .collect();
        let mut updated = lines.join("\n");
        if !updated.contains("implementation(project(\":padauk\"))") {
            updated = updated.replace(
                "dependencies {\n",
                "dependencies {\n    implementation(project(\":padauk\"))\n",
            );
        }
        let _ = fs::write(&app_build, updated);
    }

    println!(
        "✅ Android module synced from padauk v{}",
        padauk_pkg["version"]
    );
}

fn extract_zip_file(zip_path: &PathBuf, target_dir: &PathBuf) -> anyhow::Result<()> {
    let reader = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // On Unix, restore file permissions (useful for scripts)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct Device {
    serial: String,
    name: String,
    ios: bool,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.serial)
    }
}

fn get_android_devices() -> Vec<Device> {
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
            if !serial.starts_with("emulator-") {
                continue;
            }
            // Find the part that starts with "model:"
            let name = parts
                .iter()
                .find(|p| p.starts_with("model:"))
                .map(|p| p.replace("model:", ""))
                .unwrap_or_else(|| "Unknown Device".to_string());

            devices.push(Device {
                serial,
                name,
                ios: false,
            });
        }
    }
    devices
}

pub fn get_available_simulators() -> anyhow::Result<Vec<Device>> {
    // xcrun simctl list devices --json
    let output = Command::new("xcrun")
        .args(["simctl", "list", "devices", "available", "--json"])
        .output()?;

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let mut devices = Vec::new();

    if let Some(runtimes) = json["devices"].as_object() {
        for (_, dev_list) in runtimes {
            if let Some(list) = dev_list.as_array() {
                for d in list {
                    if d["state"] == "Booted" {
                        devices.push(Device {
                            serial: d["udid"].as_str().unwrap().to_string(),
                            name: d["name"].as_str().unwrap().to_string(),
                            ios: true,
                        });
                    }
                }
            }
        }
    }
    Ok(devices)
}

fn pick_android_device() -> String {
    let devices = get_android_devices();

    if devices.is_empty() {
        println!("❌ No Android emulators found. Please start an emulator and try again.");
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

use crate::assets::sync_and_generate_assets;

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
