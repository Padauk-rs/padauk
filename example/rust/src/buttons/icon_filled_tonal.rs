use padauk::{app_bar, children, column, filled_tonal_icon_button, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct IconButtonFilledTonalScreen;

impl Widget for IconButtonFilledTonalScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Filled tonal icon button"),
            filled_tonal_icon_button(IconType::Add, || {}),
        ]))
        .app_bar(app_bar("Icon Tonal"))
        .build()
    }
}
