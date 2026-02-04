use padauk::{app_bar_center_aligned, children, column, scaffold, text, Widget};

pub struct CenterAlignedAppBarScreen;

impl Widget for CenterAlignedAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Center-aligned app bar"),
            text("Titles are centered for emphasis."),
        ]))
        .app_bar(app_bar_center_aligned("Center Aligned"))
        .build()
    }
}
