use std::sync::OnceLock;
use log::{debug, info, warn};

// --- RENDERING CALLBACK (NEW) ---
#[uniffi::export(callback_interface)]
pub trait RenderCallback: Send + Sync {
    fn on_update(&self);
}

static RENDER_CALLBACK: OnceLock<Box<dyn RenderCallback>> = OnceLock::new();

#[uniffi::export]
pub fn register_render_callback(callback: Box<dyn RenderCallback>) {
    // Only one callback (the main activity) is needed
    match RENDER_CALLBACK.set(callback) {
        Ok(()) => info!("Render callback registered."),
        Err(_) => warn!("Render callback already registered."),
    }
}

/// Call this internally whenever state changes to force a native UI update
pub fn request_redraw() {
    if let Some(callback) = RENDER_CALLBACK.get() {
        debug!("Request redraw.");
        callback.on_update();
    } else {
        warn!("Request redraw ignored: no render callback registered.");
    }
}
