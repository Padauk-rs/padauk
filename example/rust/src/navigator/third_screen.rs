use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::Navigator;

pub struct ThirdScreen;

impl Widget for ThirdScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
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
        ]))
        .app_bar(app_bar("Third"))
        .build()
    }
}
