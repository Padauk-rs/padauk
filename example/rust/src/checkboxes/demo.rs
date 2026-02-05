use std::sync::atomic::{AtomicBool, Ordering};

use padauk::{app_bar, checkbox, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

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

        scaffold(column(children![
            text("Checkbox"),
            checkbox(checked, || toggle()),
            text(if checked { "Checked" } else { "Unchecked" }),
        ]))
        .app_bar(app_bar("Checkbox"))
        .build()
    }
}
