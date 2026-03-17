use std::sync::OnceLock;

use padauk::prelude::{color_hex, state, State, TabsOptions, TabsStyle};
use padauk::{app_bar, children, column, tab, tabs, text, Text, Widget};

use crate::example_layout::example_screen;

static SECONDARY_TAB: OnceLock<State<usize>> = OnceLock::new();

fn secondary_tab() -> &'static State<usize> {
    SECONDARY_TAB.get_or_init(|| state(0))
}

const TAB_LABELS: [&str; 6] = ["All", "Mentions", "Unread", "Starred", "Files", "Archived"];

const CODE: &str = r##"tabs(vec![
    tab("All", selected == 0, || secondary_tab().set(0)),
    tab("Mentions", selected == 1, || secondary_tab().set(1)),
    tab("Unread", selected == 2, || secondary_tab().set(2)),
    tab("Starred", selected == 3, || secondary_tab().set(3)),
    tab("Files", selected == 4, || secondary_tab().set(4)),
    tab("Archived", selected == 5, || secondary_tab().set(5)),
])
.options(TabsOptions {
    style: TabsStyle::Secondary,
    scrollable: true,
    indicator_color: Some(color_hex("#2B5F8C")),
    selected_content_color: Some(color_hex("#2B5F8C")),
    unselected_content_color: Some(color_hex("#6C7A88")),
    ..Default::default()
})"##;

pub struct SecondaryTabsScreen;

impl Widget for SecondaryTabsScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = secondary_tab().get();

        example_screen(
            app_bar("Tabs (Secondary)"),
            column(children![
                tabs(vec![
                    tab("All", selected == 0, || secondary_tab().set(0)),
                    tab("Mentions", selected == 1, || secondary_tab().set(1)),
                    tab("Unread", selected == 2, || secondary_tab().set(2)),
                    tab("Starred", selected == 3, || secondary_tab().set(3)),
                    tab("Files", selected == 4, || secondary_tab().set(4)),
                    tab("Archived", selected == 5, || secondary_tab().set(5)),
                ])
                .options(TabsOptions {
                    style: TabsStyle::Secondary,
                    scrollable: true,
                    indicator_color: Some(color_hex("#2B5F8C")),
                    selected_content_color: Some(color_hex("#2B5F8C")),
                    unselected_content_color: Some(color_hex("#6C7A88")),
                    ..Default::default()
                })
                .fill_max_width(),
                Text::new(format!("Selected tab: {}", TAB_LABELS[selected])).padding(12.0),
                text("Secondary tabs work well for related subsections and can be scrollable.")
                    .padding(12.0),
            ])
            .fill_max_width(),
            CODE,
        )
    }
}
