use padauk::{app_bar, children, column, fab_small, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct FabSmallScreen;

impl Widget for FabSmallScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Small FAB"),
        ]))
        .fab(fab_small(IconType::Add, || {}))
        .app_bar(app_bar("FAB Small"))
        .build()
    }
}
