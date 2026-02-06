use padauk::{app_bar, children, column, filled_tonal_button, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r#"filled_tonal_button("Tonal action", || {});"#;

pub struct FilledTonalButtonScreen;

impl Widget for FilledTonalButtonScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Filled Tonal"),
            column(children![
                text("Filled tonal button"),
                filled_tonal_button("Tonal action", || {}),
            ]),
            CODE,
        )
    }
}
