use padauk::{app_bar, children, column, scaffold, text, Widget};
use padauk::prelude::AppBarStyle;

pub struct SmallAppBarScreen;

impl Widget for SmallAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Small app bar"),
            text("Use for compact screens and short content."),
        ]))
        .app_bar(app_bar("Small").style(AppBarStyle::Small))
        .build()
    }
}
