use padauk::{app_bar, children, column, fab_extended, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct FabExtendedScreen;

impl Widget for FabExtendedScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Extended FAB"),
        ]))
        .fab(fab_extended(IconType::Add, "Create", || {}))
        .app_bar(app_bar("FAB Extended"))
        .build()
    }
}
