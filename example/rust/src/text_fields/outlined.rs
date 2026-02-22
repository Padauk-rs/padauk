use std::sync::OnceLock;

use padauk::prelude::{form_key, state, FormKey, State};
use padauk::{app_bar, children, column, filled_button, outlined_text_field, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"outlined_text_field("Password", password_state().get(), |v| {
    password_state().set(v);
})
    .password(true)
    .autovalidate_on_user_interaction(true)
    .validator(form(), |value| {
        if value.len() < 8 {
            Some("Minimum 8 characters".to_string())
        } else {
            None
        }
    });"##;

static FORM: OnceLock<FormKey> = OnceLock::new();
static PASSWORD: OnceLock<State<String>> = OnceLock::new();
static STATUS: OnceLock<State<String>> = OnceLock::new();

fn form() -> &'static FormKey {
    FORM.get_or_init(form_key)
}

fn password_state() -> &'static State<String> {
    PASSWORD.get_or_init(|| state(String::new()))
}

fn status_state() -> &'static State<String> {
    STATUS.get_or_init(|| state("Tap validate".to_string()))
}

pub struct OutlinedTextFieldScreen;

impl Widget for OutlinedTextFieldScreen {
    fn build(&self) -> padauk::UiNode {
        let password = password_state().get();
        let status = status_state().get();

        example_screen(
            app_bar("Outlined Text Field"),
            column(children![
                outlined_text_field("Password", password, |value| password_state().set(value))
                    .password(true)
                    .single_line(true)
                    .supporting_text("At least 8 characters")
                    .autovalidate_on_user_interaction(true)
                    .validator(form(), |value| {
                        if value.len() < 8 {
                            Some("Minimum 8 characters".to_string())
                        } else {
                            None
                        }
                    }),
                filled_button("Validate form", || {
                    let is_valid = form().validate();
                    status_state().set(if is_valid {
                        "Password looks good".to_string()
                    } else {
                        "Password is invalid".to_string()
                    });
                }),
                text(status.as_str()),
            ]),
            CODE,
        )
    }
}
