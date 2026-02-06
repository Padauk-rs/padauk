use crate::example_app::ExampleApp;

mod example_app;
mod example_layout;
mod app_bars;
mod buttons;
mod cards;
mod checkboxes;
mod chips;
mod generated;
mod home_screen;
mod navigator;

uniffi::setup_scaffolding!();

#[padauk::main]
fn start() {
    ExampleApp {}
}
