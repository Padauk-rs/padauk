use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::navigation_drawers::basic::BasicNavigationDrawerScreen;
use crate::navigation_drawers::styled::StyledNavigationDrawerScreen;

pub struct NavigationDrawersMenu;

impl Widget for NavigationDrawersMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Basic drawer").on_click(|| {
                Navigator::push(Route::new("navigation_drawer_basic", || {
                    BasicNavigationDrawerScreen {}
                }));
            }),
            divider(),
            list_item("Styled drawer").on_click(|| {
                Navigator::push(Route::new("navigation_drawer_styled", || {
                    StyledNavigationDrawerScreen {}
                }));
            }),
        ]))
        .app_bar(app_bar("Navigation Drawers"))
        .build()
    }
}
