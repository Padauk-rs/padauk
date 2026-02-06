use padauk::{app_bar, children, column, outlined_icon_button, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen;

const CODE: &str = include_str!("icon_outlined.rs");

pub struct IconButtonOutlinedScreen;

impl Widget for IconButtonOutlinedScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Icon Outlined"),
            column(children![
                text("Outlined icon button"),
                outlined_icon_button(IconType::Menu, || {}),
            ]),
            CODE,
        )
    }
}
