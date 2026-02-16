use std::sync::OnceLock;

use padauk::{
    app_bar, button, column, date_picker_dialog, date_range_picker_dialog, text, Widget,
};
use padauk::prelude::{state, State};

use crate::example_layout::example_screen;

const CODE: &str = r##"let open_date = date_open().get();
let open_range = range_open().get();
let selected = selected_date().get();
let (start, end) = selected_range().get();

let mut items: Vec<Box<dyn Widget>> = vec![
    Box::new(button("Open date picker", || {
        date_open().update(|v| *v = true);
    })),
    Box::new(text(format!("Selected: {}", selected.unwrap_or(0)))),
    Box::new(button("Open range picker", || {
        range_open().update(|v| *v = true);
    })),
    Box::new(text(format!("Range: {} - {}", start.unwrap_or(0), end.unwrap_or(0)))),
];

if open_date {
    items.push(Box::new(
        date_picker_dialog(
            Some("Pick a date"),
            selected,
            "OK",
            |millis| {
                selected_date().set(Some(millis));
                date_open().set(false);
            },
        )
        .dismiss("Cancel", || {
            date_open().set(false);
        }),
    ));
}

if open_range {
    items.push(Box::new(
        date_range_picker_dialog(
            Some("Pick a range"),
            start,
            end,
            "OK",
            |start_millis, end_millis| {
                selected_range().set((start_millis, end_millis));
                range_open().set(false);
            },
        )
        .dismiss("Cancel", || {
            range_open().set(false);
        }),
    ));
}

column(items)"##;

static DATE_OPEN: OnceLock<State<bool>> = OnceLock::new();
static RANGE_OPEN: OnceLock<State<bool>> = OnceLock::new();
static SELECTED_DATE: OnceLock<State<Option<i64>>> = OnceLock::new();
static SELECTED_RANGE: OnceLock<State<(Option<i64>, Option<i64>)>> = OnceLock::new();

fn date_open() -> &'static State<bool> {
    DATE_OPEN.get_or_init(|| state(false))
}

fn range_open() -> &'static State<bool> {
    RANGE_OPEN.get_or_init(|| state(false))
}

fn selected_date() -> &'static State<Option<i64>> {
    SELECTED_DATE.get_or_init(|| state(None))
}

fn selected_range() -> &'static State<(Option<i64>, Option<i64>)> {
    SELECTED_RANGE.get_or_init(|| state((None, None)))
}

pub struct DatePickerDemoScreen;

impl Widget for DatePickerDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let open_date = date_open().get();
        let open_range = range_open().get();
        let selected = selected_date().get();
        let (start, end) = selected_range().get();

        let mut items: Vec<Box<dyn Widget>> = vec![
            Box::new(button("Open date picker", || {
                date_open().update(|v| *v = true);
            })),
            Box::new(text(&format!("Selected: {}", selected.unwrap_or(0)))),
            Box::new(button("Open range picker", || {
                range_open().update(|v| *v = true);
            })),
            Box::new(text(&format!("Range: {} - {}", start.unwrap_or(0), end.unwrap_or(0)))),
        ];

        if open_date {
            items.push(Box::new(
                date_picker_dialog(
                    Some("Pick a date"),
                    selected,
                    "OK",
                    |millis| {
                        selected_date().set(Some(millis));
                        date_open().set(false);
                    },
                )
                .dismiss("Cancel", || {
                    date_open().set(false);
                }),
            ));
        }

        if open_range {
            items.push(Box::new(
                date_range_picker_dialog(
                    Some("Pick a range"),
                    start,
                    end,
                    "OK",
                    |start_millis, end_millis| {
                        selected_range().set((start_millis, end_millis));
                        range_open().set(false);
                    },
                )
                .dismiss("Cancel", || {
                    range_open().set(false);
                }),
            ));
        }

        example_screen(app_bar("Date pickers"), column(items), CODE)
    }
}
