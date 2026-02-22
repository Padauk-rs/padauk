use crate::example_app::ExampleApp;

mod app_bars;
mod buttons;
mod cards;
mod checkboxes;
mod chips;
mod date_pickers;
mod dialogs;
mod example_app;
mod example_layout;
mod generated;
mod home_screen;
mod lists;
mod navigator;
mod scroll;
mod state_demo;
mod text_fields;

uniffi::setup_scaffolding!();

#[padauk::main]
fn start() {
    ExampleApp {}
}
