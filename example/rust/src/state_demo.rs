use std::sync::OnceLock;

use padauk::{
    app_bar, button, children, column, Text, Widget,
    prelude::{State, state},
};

use crate::example_layout::example_screen;

static COUNTER: OnceLock<State<i32>> = OnceLock::new();

fn counter() -> &'static State<i32> {
    COUNTER.get_or_init(|| state(0))
}

pub struct StateDemoScreen;

impl Widget for StateDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let content = StateDemoContent;
        example_screen(
            app_bar("State"),
            content,
            STATE_DEMO_CODE,
        )
    }
}

struct StateDemoContent;

impl Widget for StateDemoContent {
    fn build(&self) -> padauk::UiNode {
        let value = counter().get();
        column(children![
            Text::new(format!("Count: {}", value)),
            button("Increment", || {
                counter().update(|v| *v += 1);
            }),
        ])
        .build()
    }
}

const STATE_DEMO_CODE: &str = r#"use std::sync::OnceLock;
use padauk::prelude::{State, state};

static COUNTER: OnceLock<State<i32>> = OnceLock::new();

fn counter() -> &'static State<i32> {
    COUNTER.get_or_init(|| state(0))
}

// In your widget:
let value = counter().get();
button("Increment", || {
    counter().update(|v| *v += 1);
});
"#;
