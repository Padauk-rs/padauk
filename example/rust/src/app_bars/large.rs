use padauk::{app_bar_large, children, column, scaffold, text, Widget};

pub struct LargeAppBarScreen;

impl Widget for LargeAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Large app bar"),
            text("Use for prominent pages with bold titles."),
        ]))
        .app_bar(app_bar_large("Large"))
        .build()
    }
}
