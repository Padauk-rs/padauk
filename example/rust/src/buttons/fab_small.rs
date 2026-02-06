use padauk::{app_bar, children, column, fab_small, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen_with_fab;

const CODE: &str = r#"scaffold(content).fab(fab_small(IconType::Add, || {}));"#;

pub struct FabSmallScreen;

impl Widget for FabSmallScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen_with_fab(
            app_bar("FAB Small"),
            column(children![text("Small FAB"),]),
            fab_small(IconType::Add, || {}),
            CODE,
        )
    }
}
