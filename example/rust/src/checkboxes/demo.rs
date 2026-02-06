use std::sync::OnceLock;

use padauk::{app_bar, checkbox, column, text, Widget};
use padauk::prelude::{color_hex, color_rgb, state, State};

use crate::example_layout::example_screen;

const CODE: &str = r##"let checked = checked_state().get();
checkbox(checked, || checked_state().update(|v| *v = !*v))
    .colors(
        Some(color_hex("#1E88E5")),
        Some(color_rgb(180, 180, 180)),
        Some(color_hex("#FFFFFF")),
    )
    .enabled(true);"##;

static CHECKED: OnceLock<State<bool>> = OnceLock::new();

fn checked_state() -> &'static State<bool> {
    CHECKED.get_or_init(|| state(false))
}

pub struct CheckboxDemo;

impl Widget for CheckboxDemo {
    fn build(&self) -> padauk::UiNode {
        let checked = checked_state().get();

        let cb = checkbox(checked, || {
            checked_state().update(|v| *v = !*v);
        })
            .colors(
                Some(color_hex("#1E88E5")),
                Some(color_rgb(180, 180, 180)),
                Some(color_hex("#FFFFFF")),
            )
            .enabled(true);

        example_screen(
            app_bar("Checkbox"),
            column(vec![
                Box::new(text("Checkbox")),
                Box::new(cb),
                Box::new(text(if checked { "Checked" } else { "Unchecked" })),
            ]),
            CODE,
        )
    }
}
