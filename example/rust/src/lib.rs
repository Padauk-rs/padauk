use crate::example_app::ExampleApp;

mod example_app;
mod app_bars;
mod buttons;
mod generated;
mod home_screen;
mod navigator;

uniffi::setup_scaffolding!();

#[padauk::main]
fn start() {
    ExampleApp {}
}
