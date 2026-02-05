use padauk::{app_bar, children, column, filled_icon_button, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct IconButtonFilledScreen;

impl Widget for IconButtonFilledScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Filled icon button"),
            filled_icon_button(IconType::Favorite, || {}),
        ]))
        .app_bar(app_bar("Icon Filled"))
        .build()
    }
}
