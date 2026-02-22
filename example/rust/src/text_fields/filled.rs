use std::sync::OnceLock;

use padauk::prelude::{form_key, state, FormKey, State};
use padauk::{app_bar, children, column, filled_button, filled_text_field, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"let form = form();
let field = filled_text_field("Name", name_state().get(), |v| name_state().set(v))
    .placeholder("Enter your name")
    .autovalidate_on_user_interaction(true)
    .validator(form, |value| {
        if value.trim().is_empty() {
            Some("Name is required".to_string())
        } else {
            None
        }
    });

filled_button("Validate form", || {
    let ok = form().validate();
    status_state().set(if ok { "Valid" } else { "Fix errors" }.to_string());
});"##;

static FORM: OnceLock<FormKey> = OnceLock::new();
static NAME: OnceLock<State<String>> = OnceLock::new();
static EMAIL: OnceLock<State<String>> = OnceLock::new();
static STATUS: OnceLock<State<String>> = OnceLock::new();

fn form() -> &'static FormKey {
    FORM.get_or_init(form_key)
}

fn name_state() -> &'static State<String> {
    NAME.get_or_init(|| state(String::new()))
}

fn email_state() -> &'static State<String> {
    EMAIL.get_or_init(|| state(String::new()))
}

fn status_state() -> &'static State<String> {
    STATUS.get_or_init(|| state("Tap validate".to_string()))
}

pub struct FilledTextFieldScreen;

impl Widget for FilledTextFieldScreen {
    fn build(&self) -> padauk::UiNode {
        let form_key = form();
        let name = name_state().get();
        let email = email_state().get();
        let status = status_state().get();

        example_screen(
            app_bar("Filled Text Field"),
            column(children![
                filled_text_field("Name", name, |value| name_state().set(value))
                    .placeholder("Enter your name")
                    .supporting_text("Required")
                    .autovalidate_on_user_interaction(true)
                    .leading_icon(padauk::prelude::IconType::Person)
                    .validator(form_key, |value| {
                        if value.trim().is_empty() {
                            Some("Name is required".to_string())
                        } else {
                            None
                        }
                    }),
                filled_text_field("Email", email, |value| email_state().set(value))
                    .placeholder("name@example.com")
                    .single_line(true)
                    .autovalidate_on_user_interaction(true)
                    .validator(form_key, |value| {
                        let trimmed = value.trim();
                        if trimmed.is_empty() {
                            Some("Email is required".to_string())
                        } else if !trimmed.contains('@') {
                            Some("Email must contain @".to_string())
                        } else {
                            None
                        }
                    }),
                filled_button("Validate form", || {
                    let is_valid = form().validate();
                    status_state().set(if is_valid {
                        "Form is valid".to_string()
                    } else {
                        "Please fix errors".to_string()
                    });
                }),
                text(status.as_str()),
            ]),
            CODE,
        )
    }
}
