use log::{LevelFilter, info};

#[uniffi::export]
pub fn init_logging() {
    // --- ANDROID ---
    #[cfg(not(target_os = "ios"))]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(LevelFilter::Debug) // Set your desired log level
                .with_tag("RustNative"), // This tag appears in Logcat
        );
        info!("Android Logger initialized successfully!");
    }

    // --- iOS / Desktop (Optional fallback) ---
    #[cfg(not(target_os = "ios"))]
    {
        // For simple debugging on other platforms, standard print is often enough
        // or you can use 'oslog' crate for iOS
        // let _ = simple_logger::init_with_level(LevelFilter::Debug);
    }
}
