use padauk::{app_bar, children, column, elevated_button, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("elevated.rs");

pub struct ElevatedButtonScreen;

impl Widget for ElevatedButtonScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Elevated"),
            column(children![
                text("Elevated button"),
                elevated_button("Elevated action", || {}),
            ]),
            CODE,
        )
    }
}
