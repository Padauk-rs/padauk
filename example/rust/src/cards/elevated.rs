use padauk::{app_bar, children, column, elevated_card, scaffold, text, Widget};

pub struct ElevatedCardScreen;

impl Widget for ElevatedCardScreen {
    fn build(&self) -> padauk::UiNode {
        let c = elevated_card(children![
            text("Elevated card"),
            text("Use elevation to emphasize content."),
        ])
        .on_click(|| {});

        scaffold(column(vec![Box::new(c)]))
            .app_bar(app_bar("Elevated Card"))
            .build()
    }
}
