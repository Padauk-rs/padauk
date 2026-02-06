use padauk::{app_bar_center_aligned, children, column, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = include_str!("center_aligned.rs");

pub struct CenterAlignedAppBarScreen;

impl Widget for CenterAlignedAppBarScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar_center_aligned("Center Aligned"),
            column(children![
                text("Center-aligned app bar"),
                text("Titles are centered for emphasis."),
            ]),
            CODE,
        )
    }
}
