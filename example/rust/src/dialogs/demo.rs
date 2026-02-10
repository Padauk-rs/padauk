use std::sync::OnceLock;

use padauk::{app_bar, button, column, dialog, text, Widget};
use padauk::prelude::{state, State};

use crate::example_layout::example_screen;

const CODE: &str = r##"let open = dialog_open().get();
let mut items: Vec<Box<dyn Widget>> = vec![
    Box::new(button("Open dialog", || {
        dialog_open().update(|v| *v = true);
    })),
    Box::new(text(if open { "Dialog is open" } else { "Dialog is closed" })),
];

if open {
    items.push(Box::new(
        dialog(
            Some("Discard draft?"),
            "You have unsaved changes. Discard draft?",
            "Discard",
            || {
                dialog_open().update(|v| *v = false);
            },
        )
        .dismiss("Cancel", || {
            dialog_open().update(|v| *v = false);
        }),
    ));
}

column(items)"##;

static OPEN: OnceLock<State<bool>> = OnceLock::new();

fn dialog_open() -> &'static State<bool> {
    OPEN.get_or_init(|| state(false))
}

pub struct DialogDemoScreen;

impl Widget for DialogDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let open = dialog_open().get();
        let mut items: Vec<Box<dyn Widget>> = vec![
            Box::new(button("Open dialog", || {
                dialog_open().update(|v| *v = true);
            })),
            Box::new(text(if open { "Dialog is open" } else { "Dialog is closed" })),
        ];

        if open {
            items.push(Box::new(
                dialog(
                    Some("Discard draft?"),
                    "You have unsaved changes. Discard draft?",
                    "Discard",
                    || {
                        dialog_open().update(|v| *v = false);
                    },
                )
                .dismiss("Cancel", || {
                    dialog_open().update(|v| *v = false);
                }),
            ));
        }

        example_screen(app_bar("Dialog"), column(items), CODE)
    }
}
