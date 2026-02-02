pub use crate::ui::widget::{UiNode, Widget};

uniffi::setup_scaffolding!();

pub mod native;
pub mod ui;

pub mod prelude {
    pub use crate::PadaukApp;
    pub use crate::native::log;
    pub use crate::ui::navigation::{Navigator, Route};
    pub use crate::ui::widget::*;
}

pub use crate::ui::widget::*;
pub use padauk_macros::main;

use std::sync::OnceLock;

// Embed the native source code inside the Rust library
#[cfg(feature = "embed-assets")]
pub const FRAMEWORK_AAR: &[u8] = include_bytes!("../assets/android/padauk-release.aar");

// #[cfg(feature = "embed-assets")]
// pub const FRAMEWORK_XC: &[u8] = include_bytes!("../assets/ios/Padauk.xcframework.zip");

pub trait PadaukApp: Send + Sync + 'static {
    /// Optional: Define the starting screen for the Navigator.
    /// If provided, the framework will initialize the Navigator automatically.
    fn initial_route(&self) -> Option<crate::ui::navigation::Route> {
        None
    }

    /// Render the UI.
    /// If initial_route() is provided, the Navigator takes precedence,
    /// and this might not be called depending on padauk_render_root logic.
    fn render(&self) -> Box<dyn Widget>;
}

static APP_INSTANCE: OnceLock<Box<dyn PadaukApp>> = OnceLock::new();

pub fn start_app<A: PadaukApp>(app: A) {
    if let Some(route) = app.initial_route() {
        crate::ui::navigation::Navigator::init(route);
    }

    let _ = APP_INSTANCE.set(Box::new(app));
}

#[uniffi::export]
pub fn padauk_render_root() -> Option<UiNode> {
    // 1. Try to render via Navigator first
    if let Some(nav_widget) = crate::ui::navigation::Navigator::render_current() {
        Some(nav_widget.build())
    } else {
        // 2. Fallback to the App's manual render() method if Navigator isn't initialized
        APP_INSTANCE.get().map(|app| app.render().build())
    }
}
