use padauk::{app_bar, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"// TODO: Replace with real text field API when available.
// Example target:
//
// filled_text_field("Name")
//     .value(name_state().get())
//     .on_change(|value| name_state().set(value))
//     .placeholder("Enter your name");"##;

pub struct FilledTextFieldScreen;

impl Widget for FilledTextFieldScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Filled Text Field"),
            column(children![
                text("Filled text field sample"),
                text("TextField widget API is not available in this build yet."),
            ]),
            CODE,
        )
    }
}
