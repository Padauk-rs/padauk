use padauk::{UiNode, Widget, children, column, text};

#[uniffi::export]
pub fn build_ui() -> UiNode {
    column(children![
        text("Hello from Padauk! 🌳").size(32.0),
        text("This is running from Rust core.").size(18.0)
    ])
    .build()
}
