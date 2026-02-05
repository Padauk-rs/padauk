use padauk::{app_bar, children, column, scaffold, text, text_button, Widget};

pub struct TextButtonScreen;

impl Widget for TextButtonScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Text button"),
            text_button("Text action", || {}),
        ]))
        .app_bar(app_bar("Text"))
        .build()
    }
}
