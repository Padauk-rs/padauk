use padauk::{app_bar, children, column, filled_tonal_icon_button, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen;

const CODE: &str = r#"filled_tonal_icon_button(IconType::Add, || {});"#;

pub struct IconButtonFilledTonalScreen;

impl Widget for IconButtonFilledTonalScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Icon Tonal"),
            column(children![
                text("Filled tonal icon button"),
                filled_tonal_icon_button(IconType::Add, || {}),
            ]),
            CODE,
        )
    }
}
