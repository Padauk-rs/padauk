use std::sync::OnceLock;

use padauk::prelude::{
    color_hex, state, IconType, NavigationDrawerOptions, NavigationDrawerType, State,
};
use padauk::{
    app_bar, children, column, nav_drawer_destination, navigation_drawer, scaffold, text, Text,
    Widget,
};

static STYLED_DRAWER_TAB: OnceLock<State<usize>> = OnceLock::new();

fn styled_drawer_tab() -> &'static State<usize> {
    STYLED_DRAWER_TAB.get_or_init(|| state(0))
}

pub struct StyledNavigationDrawerScreen;

impl Widget for StyledNavigationDrawerScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = styled_drawer_tab().get();

        scaffold(column(children![
            Text::new(format!("Active destination index: {}", selected)),
            text("This sample uses a dismissible drawer and custom drawer colors."),
        ]))
        .app_bar(app_bar("Drawer (Styled)"))
        .drawer(
            navigation_drawer(vec![
                nav_drawer_destination("Overview", IconType::Menu, selected == 0, None, || {
                    styled_drawer_tab().set(0)
                }),
                nav_drawer_destination(
                    "Favorites",
                    IconType::Favorite,
                    selected == 1,
                    Some("9".into()),
                    || styled_drawer_tab().set(1),
                ),
                nav_drawer_destination("Profile", IconType::Person, selected == 2, None, || {
                    styled_drawer_tab().set(2)
                }),
            ])
            .title("Workspace")
            .drawer_type(NavigationDrawerType::Dismissible)
            .options(NavigationDrawerOptions {
                container_color: Some(color_hex("#F7F2FA")),
                indicator_color: Some(color_hex("#DCCCF8")),
                selected_icon_color: Some(color_hex("#2A1144")),
                selected_text_color: Some(color_hex("#2A1144")),
                unselected_icon_color: Some(color_hex("#655A72")),
                unselected_text_color: Some(color_hex("#655A72")),
                ..Default::default()
            }),
        )
        .build()
    }
}
