use padauk::{app_bar, children, column, icon_button, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen;

const CODE: &str = include_str!("icon_standard.rs");

pub struct IconButtonStandardScreen;

impl Widget for IconButtonStandardScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Icon Standard"),
            column(children![
                text("Standard icon button"),
                icon_button(IconType::Search, || {}),
            ]),
            CODE,
        )
    }
}
