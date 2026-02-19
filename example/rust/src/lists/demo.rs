use std::sync::OnceLock;

use padauk::prelude::{state, IconType, State};
use padauk::{app_bar, children, column, divider, list, list_item, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"let content = list(children![
    // Single-line list item
    list_item("Inbox")
        .leading_icon(IconType::Menu)
        .trailing_text("12")
        .on_click(|| selected_item().set("Inbox".to_string())),
    // Inset horizontal divider
    divider().inset_start(56.0),
    // Two-line list item
    list_item("Updates")
        .supporting_text("2 new messages")
        .leading_icon(IconType::Person)
        .trailing_text("Now")
        .on_click(|| selected_item().set("Updates".to_string())),
    // Standard divider
    divider(),
    // Three-line list item
    list_item("Activity")
        .overline_text("Yesterday")
        .supporting_text("Build completed successfully")
        .leading_icon(IconType::Favorite)
        .trailing_icon(IconType::Search)
        .on_click(|| selected_item().set("Activity".to_string())),
]);"##;

static SELECTED_ITEM: OnceLock<State<String>> = OnceLock::new();

fn selected_item() -> &'static State<String> {
    SELECTED_ITEM.get_or_init(|| state("None".to_string()))
}

pub struct ListsDemoScreen;

impl Widget for ListsDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let selected = selected_item().get();
        let content = list(children![
            text(&format!("Selected: {}", selected)).padding(8.0),
            list_item("Inbox")
                .leading_icon(IconType::Menu)
                .trailing_text("12")
                .on_click(|| selected_item().set("Inbox".to_string())),
            divider().inset_start(56.0),
            list_item("Updates")
                .supporting_text("2 new messages")
                .leading_icon(IconType::Person)
                .trailing_text("Now")
                .on_click(|| selected_item().set("Updates".to_string())),
            divider(),
            list_item("Activity")
                .overline_text("Yesterday")
                .supporting_text("Build completed successfully")
                .leading_icon(IconType::Favorite)
                .trailing_icon(IconType::Search)
                .on_click(|| selected_item().set("Activity".to_string())),
        ])
        .fill_max_width();

        example_screen(app_bar("Lists"), column(children![content]), CODE)
    }
}
