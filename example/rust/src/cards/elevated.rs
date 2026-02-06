use padauk::{app_bar, children, column, elevated_card, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("elevated.rs");

pub struct ElevatedCardScreen;

impl Widget for ElevatedCardScreen {
    fn build(&self) -> padauk::UiNode {
        let c = elevated_card(children![
            text("Elevated card"),
            text("Use elevation to emphasize content."),
        ])
        .on_click(|| {});

        example_screen(app_bar("Elevated Card"), column(vec![Box::new(c)]), CODE)
    }
}
