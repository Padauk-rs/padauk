use padauk::{app_bar, children, column, elevated_button, scaffold, text, Widget};

pub struct ElevatedButtonScreen;

impl Widget for ElevatedButtonScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Elevated button"),
            elevated_button("Elevated action", || {}),
        ]))
        .app_bar(app_bar("Elevated"))
        .build()
    }
}
