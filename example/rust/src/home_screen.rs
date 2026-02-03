use padauk::Widget;
use padauk::{
    app_bar, button, children, column,
    prelude::{Navigator, Route},
    scaffold,
};

use crate::navigator::first_screen::FirstScreen;

pub struct HomeScreen;

impl Widget for HomeScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![button("Navigation", || {
            Navigator::push(Route::new("screen_a", || FirstScreen {}));
        })]))
        .app_bar(app_bar("Home"))
        .build()
    }
}
