use padauk::{app_bar, children, column, fab_large, text, Widget};
use padauk::prelude::IconType;

use crate::example_layout::example_screen_with_fab;

const CODE: &str = include_str!("fab_large.rs");

pub struct FabLargeScreen;

impl Widget for FabLargeScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen_with_fab(
            app_bar("FAB Large"),
            column(children![text("Large FAB"),]),
            fab_large(IconType::Add, || {}),
            CODE,
        )
    }
}
