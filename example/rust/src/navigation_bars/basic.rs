use std::sync::OnceLock;

use padauk::prelude::{state, IconType, State};
use padauk::{app_bar, children, column, nav_destination, navigation_bar, text, Text, Widget};

use crate::example_layout::example_screen_with_bottom_bar;

static BASIC_TAB: OnceLock<State<usize>> = OnceLock::new();

fn basic_tab() -> &'static State<usize> {
    BASIC_TAB.get_or_init(|| state(0))
}

fn tab_label(index: usize) -> &'static str {
    match index {
        0 => "Home",
        1 => "Search",
        2 => "Profile",
        _ => "Unknown",
    }
}

const CODE: &str = r##"navigation_bar(vec![
    nav_destination("Home", IconType::Menu, selected == 0, || basic_tab().set(0)),
    nav_destination("Search", IconType::Search, selected == 1, || basic_tab().set(1)),
    nav_destination("Profile", IconType::Person, selected == 2, || basic_tab().set(2)),
])"##;

pub struct BasicNavigationBarScreen;

impl Widget for BasicNavigationBarScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = basic_tab().get();

        example_screen_with_bottom_bar(
            app_bar("Navigation Bar (Basic)"),
            column(children![
                Text::new(format!("Selected tab: {}", tab_label(selected))),
                text("Tap items in the bottom bar to switch selection."),
            ]),
            navigation_bar(vec![
                nav_destination("Home", IconType::Menu, selected == 0, || basic_tab().set(0)),
                nav_destination("Search", IconType::Search, selected == 1, || {
                    basic_tab().set(1)
                }),
                nav_destination("Profile", IconType::Person, selected == 2, || {
                    basic_tab().set(2)
                }),
            ]),
            CODE,
        )
    }
}
