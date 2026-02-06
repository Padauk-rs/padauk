use std::sync::OnceLock;

use padauk::{app_bar, column, filter_chip, text, Widget};
use padauk::prelude::{IconType, State, state};

use crate::example_layout::example_screen;

const CODE: &str = r#"let selected = filter_state().get();
filter_chip("Selected", selected, || filter_state().update(|v| *v = !*v))
    .leading_icon(IconType::Favorite);"#;

static FILTER_SELECTED: OnceLock<State<bool>> = OnceLock::new();

fn filter_state() -> &'static State<bool> {
    FILTER_SELECTED.get_or_init(|| state(false))
}

pub struct FilterChipScreen;

impl Widget for FilterChipScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = filter_state().get();
        let chip = filter_chip("Selected", selected, || {
            filter_state().update(|v| *v = !*v);
        })
            .leading_icon(IconType::Favorite);

        let t = text(if selected { "Selected" } else { "Not selected" });
        example_screen(
            app_bar("Filter Chip"),
            column(vec![Box::new(chip), Box::new(t)]),
            CODE,
        )
    }
}
