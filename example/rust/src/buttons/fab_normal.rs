use padauk::{app_bar, children, column, fab, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen_with_fab;

const CODE: &str = r#"scaffold(content).fab(fab(IconType::Add, || {}));"#;

pub struct FabNormalScreen;

impl Widget for FabNormalScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen_with_fab(
            app_bar("FAB"),
            column(children![text("Default FAB"),]),
            fab(IconType::Add, || {}),
            CODE,
        )
    }
}
