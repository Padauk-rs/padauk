use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::tabs::primary::PrimaryTabsScreen;
use crate::tabs::secondary::SecondaryTabsScreen;

pub struct TabsMenu;

impl Widget for TabsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Primary tabs").on_click(|| {
                Navigator::push(Route::new("tabs_primary", || PrimaryTabsScreen {}));
            }),
            divider(),
            list_item("Secondary tabs (scrollable)").on_click(|| {
                Navigator::push(Route::new("tabs_secondary", || SecondaryTabsScreen {}));
            }),
        ]))
        .app_bar(app_bar("Tabs"))
        .build()
    }
}
