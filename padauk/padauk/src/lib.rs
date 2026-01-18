pub use crate::ui::widget::{UiNode, Widget};

uniffi::setup_scaffolding!();

pub mod native;
pub mod ui;

// pub use crate::nodes::UiNode;
// pub use crate::ui::widget::Widget;
pub use crate::ui::macros::*;
pub use crate::ui::widget::*;
pub use padauk_macros::main;

use std::sync::OnceLock;

// Embed the native source code inside the Rust library
pub const KOTLIN_RENDERER_SOURCE: &str = include_str!("../generated/android/PadaukRenderer.kt");
pub const KOTLIN_UI_TYPES: &str = include_str!("../generated/android/Padauk.kt");

pub trait PadaukApp: Send + Sync + 'static {
    fn render(&self) -> Box<dyn Widget>;
}

static APP_INSTANCE: OnceLock<Box<dyn PadaukApp>> = OnceLock::new();

pub fn start_app<A: PadaukApp>(app: A) {
    let _ = APP_INSTANCE.set(Box::new(app));
}

#[uniffi::export]
pub fn padauk_render_root() -> Option<UiNode> {
    ui::event_registry::clear_actions(); // Prevent action IDs from leaking memory

    let instance = APP_INSTANCE.get();
    println!(
        "🔍 padauk_render_root called. Instance exists: {}",
        instance.is_some()
    );
    instance.map(|app| app.render().build())
}
