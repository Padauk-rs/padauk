use padauk::*;

uniffi::setup_scaffolding!();

struct CounterApp {}
struct HomeScreen;

impl PadaukApp for CounterApp {
    fn initial_route(&self) -> prelude::Route {
        prelude::Route::new("home", || HomeScreen)
    }
}

impl Widget for HomeScreen {
    fn build(&self) -> UiNode {
        scaffold(column(children![text("Hello World!"),]))
            .app_bar(app_bar("My App"))
            .build()
    }
}

#[padauk::main]
fn start() {
    CounterApp {}
}
