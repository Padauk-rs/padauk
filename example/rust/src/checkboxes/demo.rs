use std::sync::atomic::{AtomicBool, Ordering};

use padauk::{app_bar, checkbox, column, text, Widget};
use padauk::prelude::{color_hex, color_rgb};
use padauk::prelude::{Navigator, Route};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("demo.rs");

static CHECKED: AtomicBool = AtomicBool::new(false);

fn toggle() {
    let next = !CHECKED.load(Ordering::SeqCst);
    CHECKED.store(next, Ordering::SeqCst);
    Navigator::replace(Route::new("checkbox_demo", || CheckboxDemo {}));
}

pub struct CheckboxDemo;

impl Widget for CheckboxDemo {
    fn build(&self) -> padauk::UiNode {
        let checked = CHECKED.load(Ordering::SeqCst);

        let cb = checkbox(checked, || toggle())
            .colors(
                Some(color_hex("#1E88E5")),
                Some(color_rgb(180, 180, 180)),
                Some(color_hex("#FFFFFF")),
            )
            .enabled(true);

        example_screen(
            app_bar("Checkbox"),
            column(vec![
                Box::new(text("Checkbox")),
                Box::new(cb),
                Box::new(text(if checked { "Checked" } else { "Unchecked" })),
            ]),
            CODE,
        )
    }
}
