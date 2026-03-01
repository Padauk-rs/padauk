use std::sync::OnceLock;

use padauk::prelude::{state, IconType, NavigationRailOptions, State};
use padauk::{
    app_bar, children, column, nav_rail_destination, navigation_rail, scaffold, text, Text, Widget,
};

static BASIC_RAIL_TAB: OnceLock<State<usize>> = OnceLock::new();

fn basic_rail_tab() -> &'static State<usize> {
    BASIC_RAIL_TAB.get_or_init(|| state(0))
}

pub struct BasicNavigationRailScreen;

impl Widget for BasicNavigationRailScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = basic_rail_tab().get();

        scaffold(column(children![
            Text::new(format!("Selected rail item index: {}", selected)),
            text("Navigation rail is useful for wider layouts."),
        ]))
        .app_bar(app_bar("Rail (Narrow)"))
        .rail(
            navigation_rail(vec![
                nav_rail_destination("Home", IconType::Menu, selected == 0, || {
                    basic_rail_tab().set(0)
                }),
                nav_rail_destination("Search", IconType::Search, selected == 1, || {
                    basic_rail_tab().set(1)
                }),
                nav_rail_destination("Profile", IconType::Person, selected == 2, || {
                    basic_rail_tab().set(2)
                }),
            ])
            .options(NavigationRailOptions {
                expanded: false,
                allow_toggle: false,
                always_show_label: true,
                ..Default::default()
            }),
        )
        .build()
    }
}
