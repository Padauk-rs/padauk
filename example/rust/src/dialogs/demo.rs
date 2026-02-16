use std::sync::OnceLock;

use padauk::{app_bar, button, children, column, dialog, dialog_fullscreen, scroll, text, Widget};
use padauk::prelude::{state, State};

use crate::example_layout::example_screen;

const CODE: &str = r##"// Basic dialog
let open = dialog_open().get();
let full = fullscreen_open().get();
let mut items: Vec<Box<dyn Widget>> = vec![
    Box::new(button("Open dialog", || {
        dialog_open().update(|v| *v = true);
    })),
    Box::new(button("Open full screen dialog", || {
        fullscreen_open().update(|v| *v = true);
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

// Full screen dialog
if full {
    items.push(Box::new(
        dialog_fullscreen(
            "Edit profile",
            scroll(column(children![
                text("Section 1").padding(8.0),
                text("Section 2").padding(8.0),
                text("Section 3").padding(8.0),
                text("Section 4").padding(8.0),
                text("Section 5").padding(8.0),
                text("Section 6").padding(8.0),
            ]))
            .fill_max_width(),
            "Close",
            || {
                fullscreen_open().update(|v| *v = false);
            },
        )
        .confirm("Save", || {
            fullscreen_open().update(|v| *v = false);
        }),
    ));
}

column(items)"##;

static OPEN: OnceLock<State<bool>> = OnceLock::new();
static FULL: OnceLock<State<bool>> = OnceLock::new();

fn dialog_open() -> &'static State<bool> {
    OPEN.get_or_init(|| state(false))
}

fn fullscreen_open() -> &'static State<bool> {
    FULL.get_or_init(|| state(false))
}

pub struct DialogDemoScreen;

impl Widget for DialogDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let open = dialog_open().get();
        let full = fullscreen_open().get();
        let mut items: Vec<Box<dyn Widget>> = vec![
            Box::new(button("Open dialog", || {
                dialog_open().update(|v| *v = true);
            })),
            Box::new(button("Open full screen dialog", || {
                fullscreen_open().update(|v| *v = true);
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

        if full {
            items.push(Box::new(
                dialog_fullscreen(
                    "Edit profile",
                    scroll(column(children![
                        text("Section 1").padding(8.0),
                        text("Section 2").padding(8.0),
                        text("Section 3").padding(8.0),
                        text("Section 4").padding(8.0),
                        text("Section 5").padding(8.0),
                        text("Section 6").padding(8.0),
                    ]))
                    .fill_max_width(),
                    "Close",
                    || {
                        fullscreen_open().update(|v| *v = false);
                    },
                )
                .confirm("Save", || {
                    fullscreen_open().update(|v| *v = false);
                }),
            ));
        }

        example_screen(app_bar("Dialog"), column(items), CODE)
    }
}
