use std::sync::OnceLock;

use padauk::prelude::{color_hex, state, IconType, NavigationRailOptions, State};
use padauk::{
    app_bar, children, column, nav_rail_destination, navigation_rail, scaffold, text, Text, Widget,
};

static STYLED_RAIL_TAB: OnceLock<State<usize>> = OnceLock::new();

fn styled_rail_tab() -> &'static State<usize> {
    STYLED_RAIL_TAB.get_or_init(|| state(0))
}

pub struct StyledNavigationRailScreen;

impl Widget for StyledNavigationRailScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = styled_rail_tab().get();

        scaffold(column(children![
            Text::new(format!("Active destination index: {}", selected)),
            text("This sample customizes indicator and label visibility."),
        ]))
        .app_bar(app_bar("Rail (Wide)"))
        .rail(
            navigation_rail(vec![
                nav_rail_destination("Overview", IconType::Menu, selected == 0, || {
                    styled_rail_tab().set(0)
                }),
                nav_rail_destination("Favorites", IconType::Favorite, selected == 1, || {
                    styled_rail_tab().set(1)
                }),
                nav_rail_destination("Profile", IconType::Person, selected == 2, || {
                    styled_rail_tab().set(2)
                }),
            ])
            .options(NavigationRailOptions {
                container_color: Some(color_hex("#F7F2FA")),
                indicator_color: Some(color_hex("#DCCCF8")),
                selected_icon_color: Some(color_hex("#2A1144")),
                selected_text_color: Some(color_hex("#2A1144")),
                unselected_icon_color: Some(color_hex("#655A72")),
                unselected_text_color: Some(color_hex("#655A72")),
                always_show_label: true,
                expanded: true,
                allow_toggle: false,
                ..Default::default()
            }),
        )
        .build()
    }
}
