use std::sync::OnceLock;

use padauk::prelude::{state, IconType, State};
use padauk::{app_bar, children, column, tab_with_icon, tabs, text, Text, Widget};

use crate::example_layout::example_screen;

static PRIMARY_TAB: OnceLock<State<usize>> = OnceLock::new();

fn primary_tab() -> &'static State<usize> {
    PRIMARY_TAB.get_or_init(|| state(0))
}

fn tab_label(index: usize) -> &'static str {
    match index {
        0 => "Overview",
        1 => "Activity",
        2 => "Profile",
        _ => "Unknown",
    }
}

const CODE: &str = r##"tabs(vec![
    tab_with_icon("Overview", IconType::Menu, selected == 0, || primary_tab().set(0)),
    tab_with_icon("Activity", IconType::Search, selected == 1, || primary_tab().set(1)),
    tab_with_icon("Profile", IconType::Person, selected == 2, || primary_tab().set(2)),
])"##;

pub struct PrimaryTabsScreen;

impl Widget for PrimaryTabsScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = primary_tab().get();

        example_screen(
            app_bar("Tabs (Primary)"),
            column(children![
                tabs(vec![
                    tab_with_icon("Overview", IconType::Menu, selected == 0, || {
                        primary_tab().set(0)
                    }),
                    tab_with_icon("Activity", IconType::Search, selected == 1, || {
                        primary_tab().set(1)
                    }),
                    tab_with_icon("Profile", IconType::Person, selected == 2, || {
                        primary_tab().set(2)
                    }),
                ])
                .fill_max_width(),
                Text::new(format!("Selected tab: {}", tab_label(selected))).padding(12.0),
                text("Primary tabs are suited for top-level sections.").padding(12.0),
            ])
            .fill_max_width(),
            CODE,
        )
    }
}
