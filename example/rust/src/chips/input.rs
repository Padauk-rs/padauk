use std::sync::atomic::{AtomicBool, Ordering};

use padauk::{app_bar, column, input_chip, scaffold, text, Widget};
use padauk::prelude::{IconType, Navigator, Route};

static INPUT_PRESENT: AtomicBool = AtomicBool::new(true);

fn remove_chip() {
    INPUT_PRESENT.store(false, Ordering::SeqCst);
    Navigator::replace(Route::new("chip_input", || InputChipScreen {}));
}

fn reset_chip() {
    INPUT_PRESENT.store(true, Ordering::SeqCst);
    Navigator::replace(Route::new("chip_input", || InputChipScreen {}));
}

pub struct InputChipScreen;

impl Widget for InputChipScreen {
    fn build(&self) -> padauk::UiNode {
        let present = INPUT_PRESENT.load(Ordering::SeqCst);

        let mut widgets: Vec<Box<dyn padauk::Widget>> = vec![
            Box::new(text("Input chips show a selected entity and can be removed.")),
        ];

        if present {
            let chip = input_chip("Jane Doe", || {})
                .leading_icon(IconType::Person)
                .trailing_icon(IconType::Close)
                .close_action(|| remove_chip());
            widgets.push(Box::new(chip));
        } else {
            widgets.push(Box::new(text("Chip removed")));
        }

        widgets.push(Box::new(
            input_chip("Reset", || reset_chip()).leading_icon(IconType::Add),
        ));

        scaffold(column(widgets))
            .app_bar(app_bar("Input Chip"))
            .build()
    }
}
