use std::sync::OnceLock;

use padauk::prelude::{form_key, state, FormKey, Navigator, Route, State, TextFieldStyle};
use padauk::{
    app_bar, children, column, dropdown_field, filled_button, menu, outlined_button, text, Widget,
};

use crate::example_layout::example_screen;

const CODE: &str = r##"dropdown_field("Country", country_state().get(), ["Myanmar", "Thailand", "Japan"], |value| {
    country_state().set(value);
})
.placeholder("Select a country")
.autovalidate_on_user_interaction(true)
.validator(form(), |value| {
    if value.trim().is_empty() {
        Some("Please choose a country".to_string())
    } else {
        None
    }
});"##;

static FORM: OnceLock<FormKey> = OnceLock::new();
static COUNTRY: OnceLock<State<String>> = OnceLock::new();
static STATUS: OnceLock<State<String>> = OnceLock::new();

fn form() -> &'static FormKey {
    FORM.get_or_init(form_key)
}

fn country_state() -> &'static State<String> {
    COUNTRY.get_or_init(|| state(String::new()))
}

fn status_state() -> &'static State<String> {
    STATUS.get_or_init(|| state("Tap validate".to_string()))
}

pub struct DropdownFieldScreen;

impl Widget for DropdownFieldScreen {
    fn build(&self) -> padauk::UiNode {
        let form_key = form();
        let country = country_state().get();
        let status = status_state().get();

        example_screen(
            app_bar("Menu & Dropdown"),
            column(children![
                menu("Show actions")
                    .item("Profile", || {
                        status_state().set("Selected: Profile".to_string());
                    })
                    .item("Settings", || {
                        status_state().set("Selected: Settings".to_string());
                    })
                    .disabled_item("Disabled action")
                    .padding(8.0),
                dropdown_field(
                    "Country",
                    country,
                    ["Myanmar", "Thailand", "Japan", "Singapore"],
                    |value| country_state().set(value),
                )
                .placeholder("Select a country")
                .supporting_text("Required")
                .autovalidate_on_user_interaction(true)
                .validator(form_key, |value| {
                    if value.trim().is_empty() {
                        Some("Please choose a country".to_string())
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
                outlined_button("Open outlined dropdown example", || {
                    Navigator::push(Route::new("dropdown_outlined_demo", || {
                        DropdownOutlinedScreen {}
                    }));
                }),
                text(status.as_str()),
            ]),
            CODE,
        )
    }
}

pub struct DropdownOutlinedScreen;

impl Widget for DropdownOutlinedScreen {
    fn build(&self) -> padauk::UiNode {
        let country = country_state().get();

        example_screen(
            app_bar("Outlined Dropdown"),
            column(children![dropdown_field(
                "Country",
                country,
                ["Myanmar", "Thailand", "Japan", "Singapore"],
                |value| country_state().set(value),
            )
            .style(TextFieldStyle::Outlined)
            .placeholder("Outlined style")
            .autovalidate_on_user_interaction(true)
            .required(form(), "Country is required"),]),
            CODE,
        )
    }
}
