use padauk::prelude::{color_hex, AppBarStyleOptions};
use padauk::{app_bar_medium, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"app_bar_medium("Custom Medium")
    .options(AppBarStyleOptions {
        container_color: Some(color_hex("#E8F0FE")),
        title_color: Some(color_hex("#1A237E")),
        nav_icon_color: Some(color_hex("#1A237E")),
    })"##;

pub struct CustomMediumAppBarScreen;

impl Widget for CustomMediumAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar_medium("Custom Medium").options(AppBarStyleOptions {
                container_color: Some(color_hex("#E8F0FE")),
                title_color: Some(color_hex("#1A237E")),
                nav_icon_color: Some(color_hex("#1A237E")),
            }),
            column(children![
                text("Medium app bar with custom style options"),
                text("Use this for a softer surface and stronger title contrast."),
            ]),
            CODE,
        )
    }
}
