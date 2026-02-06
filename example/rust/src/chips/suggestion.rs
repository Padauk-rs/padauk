use padauk::{app_bar, column, suggestion_chip, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen;

const CODE: &str = include_str!("suggestion.rs");

pub struct SuggestionChipScreen;

impl Widget for SuggestionChipScreen {
    fn build(&self) -> padauk::UiNode {
        let c = suggestion_chip("Suggestion", || {}).leading_icon(IconType::Search);
        let t = text("Suggestion chips help discover options.");
        example_screen(
            app_bar("Suggestion Chip"),
            column(vec![Box::new(c), Box::new(t)]),
            CODE,
        )
    }
}
