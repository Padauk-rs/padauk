use padauk::*;

struct CounterApp {
    count: State<i32>,
}

impl PadaukApp for CounterApp {
    fn render(&self) -> Box<dyn Widget> {
        column(children![
            text(format!("Count: {}", self.count.get())),
            button("Increment", move || {
                let val = self.count.get();
                self.count.set(val + 1);
            })
        ])
        .into_widget()
    }
}

#[padauk::main]
fn start() {
    CounterApp {
        count: State::new(0),
    }
}
