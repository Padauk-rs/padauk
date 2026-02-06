use padauk::{app_bar, children, column, text, Widget};
use padauk::prelude::AppBarStyle;

use crate::example_layout::example_screen;

const CODE: &str = r#"app_bar("Small").style(AppBarStyle::Small)"#;

pub struct SmallAppBarScreen;

impl Widget for SmallAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Small").style(AppBarStyle::Small),
            column(children![
                text("Small app bar"),
                text("Use for compact screens and short content."),
            ]),
            CODE,
        )
    }
}
