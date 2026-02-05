use padauk::{app_bar, children, column, outlined_icon_button, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct IconButtonOutlinedScreen;

impl Widget for IconButtonOutlinedScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Outlined icon button"),
            outlined_icon_button(IconType::Menu, || {}),
        ]))
        .app_bar(app_bar("Icon Outlined"))
        .build()
    }
}
