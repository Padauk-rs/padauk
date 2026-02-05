use padauk::{app_bar, children, column, outlined_button, scaffold, text, Widget};

pub struct OutlinedButtonScreen;

impl Widget for OutlinedButtonScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Outlined button"),
            outlined_button("Outlined action", || {}),
        ]))
        .app_bar(app_bar("Outlined"))
        .build()
    }
}
