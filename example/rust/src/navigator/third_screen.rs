use padauk::{app_bar, button, children, column, text, Widget};
use padauk::prelude::Navigator;

use crate::example_layout::example_screen;

const CODE: &str = include_str!("third_screen.rs");

pub struct ThirdScreen;

impl Widget for ThirdScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Third"),
            column(children![
                text("Third Screen"),
                button("Pop To First", || {
                    Navigator::pop_to_first();
                }),
                button("Pop Until Demo Root", || {
                    Navigator::pop_until("nav_demo");
                }),
                button("Pop Til Demo Root", || {
                    Navigator::pop_til("nav_demo");
                }),
            ]),
            CODE,
        )
    }
}
