use padauk::{app_bar, children, column, outlined_card, scaffold, text, Widget};

pub struct OutlinedCardScreen;

impl Widget for OutlinedCardScreen {
    fn build(&self) -> padauk::UiNode {
        let c = outlined_card(children![
            text("Outlined card"),
            text("Outlined cards show a border without elevation."),
        ])
        .on_click(|| {});

        scaffold(column(vec![Box::new(c)]))
            .app_bar(app_bar("Outlined Card"))
            .build()
    }
}
