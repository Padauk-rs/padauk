use std::sync::atomic::{AtomicBool, Ordering};

use padauk::{app_bar, column, filter_chip, text, Widget};
use padauk::prelude::{IconType, Navigator, Route};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("filter.rs");

static FILTER_SELECTED: AtomicBool = AtomicBool::new(false);

fn toggle_filter() {
    let next = !FILTER_SELECTED.load(Ordering::SeqCst);
    FILTER_SELECTED.store(next, Ordering::SeqCst);
    Navigator::replace(Route::new("chip_filter", || FilterChipScreen {}));
}

pub struct FilterChipScreen;

impl Widget for FilterChipScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = FILTER_SELECTED.load(Ordering::SeqCst);
        let chip = filter_chip("Selected", selected, || toggle_filter())
            .leading_icon(IconType::Favorite);

        let t = text(if selected { "Selected" } else { "Not selected" });
        example_screen(
            app_bar("Filter Chip"),
            column(vec![Box::new(chip), Box::new(t)]),
            CODE,
        )
    }
}
