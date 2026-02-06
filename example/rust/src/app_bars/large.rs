use padauk::{app_bar_large, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("large.rs");

pub struct LargeAppBarScreen;

impl Widget for LargeAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar_large("Large"),
            column(children![
                text("Large app bar"),
                text("Use for prominent pages with bold titles."),
            ]),
            CODE,
        )
    }
}
