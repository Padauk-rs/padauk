use padauk::{app_bar, children, column, fab_large, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct FabLargeScreen;

impl Widget for FabLargeScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Large FAB"),
        ]))
        .fab(fab_large(IconType::Add, || {}))
        .app_bar(app_bar("FAB Large"))
        .build()
    }
}
