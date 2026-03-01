use std::sync::OnceLock;

use padauk::prelude::{state, IconType, State};
use padauk::{
    app_bar, children, column, nav_drawer_destination, navigation_drawer, scaffold, text, Text,
    Widget,
};

static BASIC_DRAWER_TAB: OnceLock<State<usize>> = OnceLock::new();

fn basic_drawer_tab() -> &'static State<usize> {
    BASIC_DRAWER_TAB.get_or_init(|| state(0))
}

fn tab_name(index: usize) -> &'static str {
    match index {
        0 => "Inbox",
        1 => "Starred",
        2 => "Sent",
        _ => "Unknown",
    }
}

pub struct BasicNavigationDrawerScreen;

impl Widget for BasicNavigationDrawerScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = basic_drawer_tab().get();

        scaffold(column(children![
            Text::new(format!("Selected drawer item: {}", tab_name(selected))),
            text("Open drawer using the app bar menu icon or edge swipe."),
        ]))
        .app_bar(app_bar("Drawer (Basic)"))
        .drawer(
            navigation_drawer(vec![
                nav_drawer_destination("Inbox", IconType::Menu, selected == 0, None, || {
                    basic_drawer_tab().set(0)
                }),
                nav_drawer_destination(
                    "Starred",
                    IconType::Favorite,
                    selected == 1,
                    Some("12".into()),
                    || basic_drawer_tab().set(1),
                ),
                nav_drawer_destination("Sent", IconType::Search, selected == 2, None, || {
                    basic_drawer_tab().set(2)
                }),
            ])
            .title("Mailboxes"),
        )
        .build()
    }
}
