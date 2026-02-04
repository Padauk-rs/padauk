use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::app_bars::small::SmallAppBarScreen;
use crate::app_bars::center_aligned::CenterAlignedAppBarScreen;
use crate::app_bars::medium::MediumAppBarScreen;
use crate::app_bars::large::LargeAppBarScreen;

pub struct AppBarsMenu;

impl Widget for AppBarsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("App Bars"),
            button("Small", || {
                Navigator::push(Route::new("app_bar_small", || SmallAppBarScreen {}));
            }),
            button("Center Aligned", || {
                Navigator::push(Route::new("app_bar_center", || CenterAlignedAppBarScreen {}));
            }),
            button("Medium", || {
                Navigator::push(Route::new("app_bar_medium", || MediumAppBarScreen {}));
            }),
            button("Large", || {
                Navigator::push(Route::new("app_bar_large", || LargeAppBarScreen {}));
            }),
        ]))
        .app_bar(app_bar("App Bars"))
        .build()
    }
}
