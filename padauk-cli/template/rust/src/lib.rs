use padauk::*;

uniffi::setup_scaffolding!();

struct CounterApp {}

impl PadaukApp for CounterApp {
    fn render(&self) -> Box<dyn Widget> {
        column(children![text("Hello World!"),]).into_widget()
    }
}

#[padauk::main]
fn start() {
    CounterApp {}
}
