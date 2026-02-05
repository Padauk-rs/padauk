use padauk::{app_bar, children, column, filled_button, scaffold, text, Widget};

pub struct FilledButtonScreen;

impl Widget for FilledButtonScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Filled button"),
            filled_button("Primary action", || {}),
        ]))
        .app_bar(app_bar("Filled"))
        .build()
    }
}
