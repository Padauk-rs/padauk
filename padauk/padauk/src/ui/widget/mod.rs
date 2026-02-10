#[cfg(not(target_os = "ios"))]
pub use crate::native::android_ui_node::AndroidUiNode;

#[cfg(target_os = "ios")]
pub use crate::native::ios_ui_node::IosUiNode;

#[cfg(target_os = "ios")]
pub use IosUiNode as UiNode;

#[cfg(not(target_os = "ios"))]
pub use AndroidUiNode as UiNode;

mod controls;
mod dialogs;
mod layout;

pub use controls::{
    AppBar, Button, Card, Checkbox, Chip, Fab, IconButton, Text, app_bar, app_bar_center_aligned,
    app_bar_large, app_bar_medium, assist_chip, button, card, checkbox, elevated_button,
    elevated_card, fab, fab_extended, fab_large, fab_small, filled_button, filled_icon_button,
    filled_tonal_button, filled_tonal_icon_button, filter_chip, icon_button, input_chip,
    outlined_button, outlined_card, outlined_icon_button, suggestion_chip, text, text_button,
};
pub use dialogs::{Dialog, FullscreenDialog, dialog, dialog_fullscreen};
pub use layout::{Column, Scaffold, Scroll, column, scaffold, scroll};

// This is equivalent to Flutter's "abstract class Widget"
pub trait Widget {
    // Equivalent to: Widget build(BuildContext context)
    fn build(&self) -> UiNode;
}

pub trait IntoWidget {
    fn into_widget(self) -> Box<dyn Widget>;
}

impl<T: Widget + Sized + 'static> IntoWidget for T {
    fn into_widget(self) -> Box<dyn Widget> {
        Box::new(self)
    }
}
