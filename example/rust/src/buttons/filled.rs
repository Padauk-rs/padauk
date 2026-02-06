use padauk::{app_bar, children, column, filled_button, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("filled.rs");

pub struct FilledButtonScreen;

impl Widget for FilledButtonScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Filled"),
            column(children![
                text("Filled button"),
                filled_button("Primary action", || {}),
            ]),
            CODE,
        )
    }
}
