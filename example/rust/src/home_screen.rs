use padauk::Widget;
use padauk::{
    app_bar, button, children, column, scaffold, text,
    prelude::{Navigator, Route},
};

use crate::navigator::navigation_menu::NavigationMenu;

pub struct HomeScreen;

impl Widget for HomeScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Examples"),
            button("Navigation", || {
                Navigator::push(Route::new("nav_demo", || NavigationMenu {}));
            }),
        ]))
        .app_bar(app_bar("Home"))
        .build()
    }
}
