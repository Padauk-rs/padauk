use padauk::{app_bar, children, column, fab, scaffold, text, Widget};
use padauk::prelude::IconType;

pub struct FabNormalScreen;

impl Widget for FabNormalScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Default FAB"),
        ]))
        .fab(fab(IconType::Add, || {}))
        .app_bar(app_bar("FAB"))
        .build()
    }
}
