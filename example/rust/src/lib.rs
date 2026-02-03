use crate::example_app::ExampleApp;

mod example_app;
mod generated;
mod home_screen;
mod navigator;

uniffi::setup_scaffolding!();

#[padauk::main]
fn start() {
    ExampleApp {}
}
