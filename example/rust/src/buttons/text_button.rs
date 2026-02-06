use padauk::{app_bar, children, column, text, text_button, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r#"text_button("Text action", || {});"#;

pub struct TextButtonScreen;

impl Widget for TextButtonScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Text"),
            column(children![
                text("Text button"),
                text_button("Text action", || {}),
            ]),
            CODE,
        )
    }
}
