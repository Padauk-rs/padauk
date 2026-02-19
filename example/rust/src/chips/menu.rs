use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::chips::assist::AssistChipScreen;
use crate::chips::filter::FilterChipScreen;
use crate::chips::input::InputChipScreen;
use crate::chips::suggestion::SuggestionChipScreen;

pub struct ChipsMenu;

impl Widget for ChipsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Assist").on_click(|| {
                Navigator::push(Route::new("chip_assist", || AssistChipScreen {}));
            }),
            divider(),
            list_item("Filter").on_click(|| {
                Navigator::push(Route::new("chip_filter", || FilterChipScreen {}));
            }),
            divider(),
            list_item("Input").on_click(|| {
                Navigator::push(Route::new("chip_input", || InputChipScreen {}));
            }),
            divider(),
            list_item("Suggestion").on_click(|| {
                Navigator::push(Route::new("chip_suggestion", || SuggestionChipScreen {}));
            }),
        ]))
        .app_bar(app_bar("Chips"))
        .build()
    }
}
