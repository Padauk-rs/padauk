use std::sync::OnceLock;

use padauk::prelude::{state, IconType, NavigationRailOptions, State};
use padauk::{
    app_bar, children, column, nav_rail_destination, navigation_rail, scaffold, text, Text, Widget,
};

static TOGGLE_RAIL_TAB: OnceLock<State<usize>> = OnceLock::new();

fn toggle_rail_tab() -> &'static State<usize> {
    TOGGLE_RAIL_TAB.get_or_init(|| state(0))
}

pub struct ToggleNavigationRailScreen;

impl Widget for ToggleNavigationRailScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = toggle_rail_tab().get();

        scaffold(column(children![
            Text::new(format!("Active destination index: {}", selected)),
            text("Use the top-left menu icon to toggle narrow/wide rail."),
        ]))
        .app_bar(app_bar("Rail (Toggle)"))
        .rail(
            navigation_rail(vec![
                nav_rail_destination("Home", IconType::Menu, selected == 0, || {
                    toggle_rail_tab().set(0)
                }),
                nav_rail_destination("Search", IconType::Search, selected == 1, || {
                    toggle_rail_tab().set(1)
                }),
                nav_rail_destination("Profile", IconType::Person, selected == 2, || {
                    toggle_rail_tab().set(2)
                }),
            ])
            .options(NavigationRailOptions {
                expanded: false,
                allow_toggle: true,
                always_show_label: true,
                ..Default::default()
            }),
        )
        .build()
    }
}
