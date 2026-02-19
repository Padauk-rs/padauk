use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::app_bars::center_aligned::CenterAlignedAppBarScreen;
use crate::app_bars::large::LargeAppBarScreen;
use crate::app_bars::medium::MediumAppBarScreen;
use crate::app_bars::small::SmallAppBarScreen;

pub struct AppBarsMenu;

impl Widget for AppBarsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Small").on_click(|| {
                Navigator::push(Route::new("app_bar_small", || SmallAppBarScreen {}));
            }),
            divider(),
            list_item("Center Aligned").on_click(|| {
                Navigator::push(Route::new(
                    "app_bar_center",
                    || CenterAlignedAppBarScreen {},
                ));
            }),
            divider(),
            list_item("Medium").on_click(|| {
                Navigator::push(Route::new("app_bar_medium", || MediumAppBarScreen {}));
            }),
            divider(),
            list_item("Large").on_click(|| {
                Navigator::push(Route::new("app_bar_large", || LargeAppBarScreen {}));
            }),
        ]))
        .app_bar(app_bar("App Bars"))
        .build()
    }
}
