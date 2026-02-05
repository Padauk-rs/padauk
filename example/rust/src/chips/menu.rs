use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::chips::assist::AssistChipScreen;
use crate::chips::filter::FilterChipScreen;
use crate::chips::input::InputChipScreen;
use crate::chips::suggestion::SuggestionChipScreen;

pub struct ChipsMenu;

impl Widget for ChipsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Chips"),
            button("Assist", || {
                Navigator::push(Route::new("chip_assist", || AssistChipScreen {}));
            }),
            button("Filter", || {
                Navigator::push(Route::new("chip_filter", || FilterChipScreen {}));
            }),
            button("Input", || {
                Navigator::push(Route::new("chip_input", || InputChipScreen {}));
            }),
            button("Suggestion", || {
                Navigator::push(Route::new("chip_suggestion", || SuggestionChipScreen {}));
            }),
        ]))
        .app_bar(app_bar("Chips"))
        .build()
    }
}
