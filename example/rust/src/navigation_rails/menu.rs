use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::navigation_rails::basic::BasicNavigationRailScreen;
use crate::navigation_rails::styled::StyledNavigationRailScreen;
use crate::navigation_rails::toggle::ToggleNavigationRailScreen;

pub struct NavigationRailsMenu;

impl Widget for NavigationRailsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Narrow rail").on_click(|| {
                Navigator::push(Route::new("navigation_rail_basic", || {
                    BasicNavigationRailScreen {}
                }));
            }),
            divider(),
            list_item("Wide rail").on_click(|| {
                Navigator::push(Route::new("navigation_rail_styled", || {
                    StyledNavigationRailScreen {}
                }));
            }),
            divider(),
            list_item("Toggle (narrow/wide)").on_click(|| {
                Navigator::push(Route::new("navigation_rail_toggle", || {
                    ToggleNavigationRailScreen {}
                }));
            }),
        ]))
        .app_bar(app_bar("Navigation Rails"))
        .build()
    }
}
