use padauk::{app_bar, children, column, fab_extended, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen_with_fab;

const CODE: &str = include_str!("fab_extended.rs");

pub struct FabExtendedScreen;

impl Widget for FabExtendedScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen_with_fab(
            app_bar("FAB Extended"),
            column(children![text("Extended FAB"),]),
            fab_extended(IconType::Add, "Create", || {}),
            CODE,
        )
    }
}
