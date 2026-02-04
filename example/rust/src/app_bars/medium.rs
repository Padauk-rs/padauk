use padauk::{app_bar_medium, children, column, scaffold, text, Widget};

pub struct MediumAppBarScreen;

impl Widget for MediumAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Medium app bar"),
            text("Use when scrolling content needs a taller bar."),
        ]))
        .app_bar(app_bar_medium("Medium"))
        .build()
    }
}
