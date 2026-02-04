use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::navigator::first_screen::FirstScreen;
use crate::navigator::second_screen::SecondScreen;
use crate::navigator::third_screen::ThirdScreen;

pub struct NavigationMenu;

impl Widget for NavigationMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Navigation Demo"),
            button("Push First", || {
                Navigator::push(Route::new("first", || FirstScreen {}));
            }),
            button("Push Second", || {
                Navigator::push(Route::new("second", || SecondScreen {}));
            }),
            button("Push Third", || {
                Navigator::push(Route::new("third", || ThirdScreen {}));
            }),
            button("Replace with First", || {
                Navigator::replace(Route::new("first", || FirstScreen {}));
            }),
            button("Replace with Second", || {
                Navigator::replace(Route::new("second", || SecondScreen {}));
            }),
            button("Replace with Third", || {
                Navigator::replace(Route::new("third", || ThirdScreen {}));
            }),
        ]))
        .app_bar(app_bar("Navigation"))
        .build()
    }
}
