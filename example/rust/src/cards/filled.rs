use padauk::{app_bar, card, children, column, scaffold, text, Widget};

pub struct FilledCardScreen;

impl Widget for FilledCardScreen {
    fn build(&self) -> padauk::UiNode {
        let c = card(children![
            text("Filled card"),
            text("Cards provide flexible containers for content."),
        ])
        .on_click(|| {});

        scaffold(column(vec![Box::new(c)]))
            .app_bar(app_bar("Filled Card"))
            .build()
    }
}
