use std::sync::OnceLock;

use padauk::{app_bar, column, input_chip, text, Widget};
use padauk::prelude::{IconType, State, state};

use crate::example_layout::example_screen;

const CODE: &str = r#"if input_state().get() {
    input_chip("Jane Doe", || {})
        .leading_icon(IconType::Person)
        .trailing_icon(IconType::Close)
        .close_action(|| input_state().set(false));
}"#;

static INPUT_PRESENT: OnceLock<State<bool>> = OnceLock::new();

fn input_state() -> &'static State<bool> {
    INPUT_PRESENT.get_or_init(|| state(true))
}

pub struct InputChipScreen;

impl Widget for InputChipScreen {
    fn build(&self) -> padauk::UiNode {
        let present = input_state().get();

        let mut widgets: Vec<Box<dyn padauk::Widget>> = vec![
            Box::new(text("Input chips show a selected entity and can be removed.")),
        ];

        if present {
            let chip = input_chip("Jane Doe", || {})
                .leading_icon(IconType::Person)
                .trailing_icon(IconType::Close)
                .close_action(|| input_state().set(false));
            widgets.push(Box::new(chip));
        } else {
            widgets.push(Box::new(text("Chip removed")));
        }

        widgets.push(Box::new(
            input_chip("Reset", || input_state().set(true)).leading_icon(IconType::Add),
        ));

        example_screen(app_bar("Input Chip"), column(widgets), CODE)
    }
}
