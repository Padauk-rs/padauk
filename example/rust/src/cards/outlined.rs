use padauk::{app_bar, children, column, outlined_card, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("outlined.rs");

pub struct OutlinedCardScreen;

impl Widget for OutlinedCardScreen {
    fn build(&self) -> padauk::UiNode {
        let c = outlined_card(children![
            text("Outlined card"),
            text("Outlined cards show a border without elevation."),
        ])
        .on_click(|| {});

        example_screen(app_bar("Outlined Card"), column(vec![Box::new(c)]), CODE)
    }
}
