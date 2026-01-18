use crate::ui::widget::{UiNode, Widget};

uniffi::setup_scaffolding!();

mod native;
mod ui;

use std::sync::OnceLock;

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

    APP_INSTANCE.get().map(|app| app.render().build())
}
