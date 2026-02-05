use padauk::{app_bar, children, column, filled_tonal_button, scaffold, text, Widget};

pub struct FilledTonalButtonScreen;

impl Widget for FilledTonalButtonScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Filled tonal button"),
            filled_tonal_button("Tonal action", || {}),
        ]))
        .app_bar(app_bar("Filled Tonal"))
        .build()
    }
}
