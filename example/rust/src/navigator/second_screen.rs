use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::navigator::third_screen::ThirdScreen;

pub struct SecondScreen;

impl Widget for SecondScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Second Screen"),
            button("Push Third", || {
                Navigator::push(Route::new("third", || ThirdScreen {}));
            }),
            button("Replace with Third", || {
                Navigator::replace(Route::new("third", || ThirdScreen {}));
            }),
            button("Pop Until Demo Root", || {
                Navigator::pop_until("nav_demo");
            }),
            button("Pop Til Demo Root", || {
                Navigator::pop_til("nav_demo");
            }),
            button("Pop To First", || {
                Navigator::pop_to_first();
            }),
        ]))
        .app_bar(app_bar("Second"))
        .build()
    }
}
