use padauk::{app_bar, children, column, scroll, text, Widget};

use crate::example_layout::example_screen;

const CODE: &str = r##"scroll(column(children![
    text("Item 1").padding(8.0),
    text("Item 2").padding(8.0),
    text("Item 3").padding(8.0),
    text("Item 4").padding(8.0),
    text("Item 5").padding(8.0),
    text("Item 6").padding(8.0),
    text("Item 7").padding(8.0),
    text("Item 8").padding(8.0),
    text("Item 9").padding(8.0),
    text("Item 10").padding(8.0),
]))
.fill_max_width()
.height(200.0);"##;

pub struct ScrollDemoScreen;

impl Widget for ScrollDemoScreen {
    fn build(&self) -> padauk::UiNode {
        let content = scroll(column(children![
            text("Item 1").padding(8.0),
            text("Item 2").padding(8.0),
            text("Item 3").padding(8.0),
            text("Item 4").padding(8.0),
            text("Item 5").padding(8.0),
            text("Item 6").padding(8.0),
            text("Item 7").padding(8.0),
            text("Item 8").padding(8.0),
            text("Item 9").padding(8.0),
            text("Item 10").padding(8.0),
        ]))
        .fill_max_width()
        .height(200.0);

        example_screen(app_bar("Scroll"), content, CODE)
    }
}
