use padauk::{app_bar, column, scaffold, suggestion_chip, text, Widget};
use padauk::prelude::IconType;

pub struct SuggestionChipScreen;

impl Widget for SuggestionChipScreen {
    fn build(&self) -> padauk::UiNode {
        let c = suggestion_chip("Suggestion", || {}).leading_icon(IconType::Search);
        let t = text("Suggestion chips help discover options.");
        scaffold(column(vec![Box::new(c), Box::new(t)]))
            .app_bar(app_bar("Suggestion Chip"))
            .build()
    }
}
