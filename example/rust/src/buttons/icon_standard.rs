use padauk::{app_bar, children, column, icon_button, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct IconButtonStandardScreen;

impl Widget for IconButtonStandardScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Standard icon button"),
            icon_button(IconType::Search, || {}),
        ]))
        .app_bar(app_bar("Icon Standard"))
        .build()
    }
}
