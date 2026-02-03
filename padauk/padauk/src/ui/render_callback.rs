use std::sync::OnceLock;

// --- RENDERING CALLBACK (NEW) ---
#[uniffi::export(callback_interface)]
pub trait RenderCallback: Send + Sync {
    fn on_update(&self);
}

static RENDER_CALLBACK: OnceLock<Box<dyn RenderCallback>> = OnceLock::new();

#[uniffi::export]
pub fn register_render_callback(callback: Box<dyn RenderCallback>) {
    // Only one callback (the main activity) is needed
    let _ = RENDER_CALLBACK.set(callback);
}

/// Call this internally whenever state changes to force a native UI update
pub fn request_redraw() {
    if let Some(callback) = RENDER_CALLBACK.get() {
        callback.on_update();
    }
}
