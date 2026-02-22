use padauk::{app_bar, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"// TODO: Replace with real text field API when available.
// Example target:
//
// outlined_text_field("Email")
//     .value(email_state().get())
//     .on_change(|value| email_state().set(value))
//     .placeholder("name@example.com");"##;

pub struct OutlinedTextFieldScreen;

impl Widget for OutlinedTextFieldScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Outlined Text Field"),
            column(children![
                text("Outlined text field sample"),
                text("TextField widget API is not available in this build yet."),
            ]),
            CODE,
        )
    }
}
