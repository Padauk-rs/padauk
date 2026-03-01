use padauk::prelude::{color_hex, AppBarStyle, AppBarStyleOptions};
use padauk::{app_bar, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"app_bar("Branded Small")
    .style(AppBarStyle::Small)
    .options(AppBarStyleOptions {
        container_color: Some(color_hex("#0B3D91")),
        title_color: Some(color_hex("#FFFFFF")),
        nav_icon_color: Some(color_hex("#FFFFFF")),
    })"##;

pub struct CustomSmallAppBarScreen;

impl Widget for CustomSmallAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Branded Small")
                .style(AppBarStyle::Small)
                .options(AppBarStyleOptions {
                    container_color: Some(color_hex("#0B3D91")),
                    title_color: Some(color_hex("#FFFFFF")),
                    nav_icon_color: Some(color_hex("#FFFFFF")),
                }),
            column(children![
                text("Small app bar with custom colors"),
                text("Use AppBarStyleOptions to apply brand colors."),
            ]),
            CODE,
        )
    }
}
