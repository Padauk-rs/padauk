use std::sync::OnceLock;

use padauk::prelude::{color_hex, state, IconType, NavigationBarOptions, State};
use padauk::{app_bar, children, column, nav_destination, navigation_bar, text, Text, Widget};

use crate::example_layout::example_screen_with_bottom_bar;

static STYLED_TAB: OnceLock<State<usize>> = OnceLock::new();

fn styled_tab() -> &'static State<usize> {
    STYLED_TAB.get_or_init(|| state(0))
}

const CODE: &str = r##"navigation_bar(vec![
    nav_destination("Home", IconType::Menu, selected == 0, || styled_tab().set(0)),
    nav_destination("Search", IconType::Search, selected == 1, || styled_tab().set(1)),
    nav_destination("Profile", IconType::Person, selected == 2, || styled_tab().set(2)),
])
.options(NavigationBarOptions {
    container_color: Some(color_hex("#F8F5FF")),
    indicator_color: Some(color_hex("#D8CCFF")),
    selected_icon_color: Some(color_hex("#1F1147")),
    selected_text_color: Some(color_hex("#1F1147")),
    unselected_icon_color: Some(color_hex("#5E5873")),
    unselected_text_color: Some(color_hex("#5E5873")),
    always_show_label: false,
    ..Default::default()
})"##;

pub struct StyledNavigationBarScreen;

impl Widget for StyledNavigationBarScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = styled_tab().get();

        example_screen_with_bottom_bar(
            app_bar("Navigation Bar (Styled)"),
            column(children![
                Text::new(format!("Active index: {}", selected)),
                text("This sample customizes colors and hides unselected labels."),
            ]),
            navigation_bar(vec![
                nav_destination("Home", IconType::Menu, selected == 0, || {
                    styled_tab().set(0)
                }),
                nav_destination("Search", IconType::Search, selected == 1, || {
                    styled_tab().set(1)
                }),
                nav_destination("Profile", IconType::Person, selected == 2, || {
                    styled_tab().set(2)
                }),
            ])
            .options(NavigationBarOptions {
                container_color: Some(color_hex("#F8F5FF")),
                indicator_color: Some(color_hex("#D8CCFF")),
                selected_icon_color: Some(color_hex("#1F1147")),
                selected_text_color: Some(color_hex("#1F1147")),
                unselected_icon_color: Some(color_hex("#5E5873")),
                unselected_text_color: Some(color_hex("#5E5873")),
                always_show_label: false,
                ..Default::default()
            }),
            CODE,
        )
    }
}
