use padauk::{app_bar, children, column, outlined_button, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r#"outlined_button("Outlined action", || {});"#;

pub struct OutlinedButtonScreen;

impl Widget for OutlinedButtonScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Outlined"),
            column(children![
                text("Outlined button"),
                outlined_button("Outlined action", || {}),
            ]),
            CODE,
        )
    }
}
