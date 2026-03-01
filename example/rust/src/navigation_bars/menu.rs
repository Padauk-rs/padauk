use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::navigation_bars::basic::BasicNavigationBarScreen;
use crate::navigation_bars::styled::StyledNavigationBarScreen;

pub struct NavigationBarsMenu;

impl Widget for NavigationBarsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Basic").on_click(|| {
                Navigator::push(Route::new("navigation_bar_basic", || {
                    BasicNavigationBarScreen {}
                }));
            }),
            divider(),
            list_item("Styled").on_click(|| {
                Navigator::push(Route::new("navigation_bar_styled", || {
                    StyledNavigationBarScreen {}
                }));
            }),
        ]))
        .app_bar(app_bar("Navigation Bars"))
        .build()
    }
}
