use padauk::Widget;
use padauk::{
    app_bar, button, children, column, scaffold, text,
    prelude::{Navigator, Route},
};

use crate::app_bars::menu::AppBarsMenu;
use crate::buttons::menu::ButtonsMenu;
use crate::cards::menu::CardsMenu;
use crate::checkboxes::demo::CheckboxDemo;
use crate::chips::menu::ChipsMenu;
use crate::navigator::navigation_menu::NavigationMenu;

pub struct HomeScreen;

impl Widget for HomeScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Examples"),
            button("Navigation", || {
                Navigator::push(Route::new("nav_demo", || NavigationMenu {}));
            }),
            button("App bars", || {
                Navigator::push(Route::new("app_bars", || AppBarsMenu {}));
            }),
            button("Buttons", || {
                Navigator::push(Route::new("buttons", || ButtonsMenu {}));
            }),
            button("Cards", || {
                Navigator::push(Route::new("cards", || CardsMenu {}));
            }),
            button("Checkbox", || {
                Navigator::push(Route::new("checkbox_demo", || CheckboxDemo {}));
            }),
            button("Chips", || {
                Navigator::push(Route::new("chips", || ChipsMenu {}));
            }),
        ]))
        .app_bar(app_bar("Home"))
        .build()
    }
}
