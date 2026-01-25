use std::sync::OnceLock;
use uniffi::deps::anyhow;

// 1. Define a specific Error type for UniFFI
// UniFFI requires a named enum for Result error types.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum PlatformError {
    #[error("Resource not found: {name}")]
    NotFound { name: String },
    #[error("Generic platform error: {details}")]
    Generic { details: String },
}

// 2. Update the trait to return Result<..., PlatformError>
#[uniffi::export(callback_interface)]
pub trait ResourceLoader: Send + Sync {
    /// Returns the bytes of the file, or a PlatformError
    fn load_raw_resource(&self, name: String) -> Result<Vec<u8>, PlatformError>;
}

// Global storage
static RESOURCE_LOADER: OnceLock<Box<dyn ResourceLoader>> = OnceLock::new();

#[uniffi::export]
pub fn register_resource_loader(loader: Box<dyn ResourceLoader>) {
    if RESOURCE_LOADER.set(loader).is_err() {
        println!("⚠️ Resource loader already registered");
    }
}

// 3. Public API for User (Maps PlatformError to anyhow for ease of use)
pub fn load_raw(name: impl Into<String>) -> anyhow::Result<Vec<u8>> {
    let name = name.into();
    if let Some(loader) = RESOURCE_LOADER.get() {
        loader.load_raw_resource(name).map_err(|e| match e {
            PlatformError::NotFound { name } => anyhow::anyhow!("Asset not found: {}", name),
            PlatformError::Generic { details } => anyhow::anyhow!("Platform error: {}", details),
        })
    } else {
        Err(anyhow::anyhow!("Platform resource loader not initialized"))
    }
}
