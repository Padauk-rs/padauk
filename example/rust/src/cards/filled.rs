use padauk::{app_bar, card, column, text, Widget};
use padauk::prelude::{CardShape, CardStyleOptions, color_hex};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("filled.rs");

pub struct FilledCardScreen;

impl Widget for FilledCardScreen {
    fn build(&self) -> padauk::UiNode {
        let options = CardStyleOptions {
            enabled: true,
            shape: CardShape::Rounded,
            container_color: Some(color_hex("#FFF3E0")),
            border_color: Some(color_hex("#FFB74D")),
            border_width: Some(1.0),
            elevation: Some(2.0),
        };

        let c = card(vec![
            Box::new(text("Filled card")),
            Box::new(text("Cards provide flexible containers for content.")),
        ])
        .options(options)
        .on_click(|| {});

        example_screen(app_bar("Filled Card"), column(vec![Box::new(c)]), CODE)
    }
}
