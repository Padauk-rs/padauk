use padauk::{app_bar_medium, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r#"app_bar_medium("Medium")"#;

pub struct MediumAppBarScreen;

impl Widget for MediumAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar_medium("Medium"),
            column(children![
                text("Medium app bar"),
                text("Use when scrolling content needs a taller bar."),
            ]),
            CODE,
        )
    }
}
