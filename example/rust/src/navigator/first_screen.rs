use padauk::{app_bar, scaffold, text, Widget};

pub struct FirstScreen;

impl Widget for FirstScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(text("This is the first screen!"))
            .app_bar(app_bar("Screen A"))
            .build()
    }
}
