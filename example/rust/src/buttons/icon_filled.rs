use padauk::{app_bar, children, column, filled_icon_button, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen;

const CODE: &str = r#"filled_icon_button(IconType::Favorite, || {});"#;

pub struct IconButtonFilledScreen;

impl Widget for IconButtonFilledScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Icon Filled"),
            column(children![
                text("Filled icon button"),
                filled_icon_button(IconType::Favorite, || {}),
            ]),
            CODE,
        )
    }
}
