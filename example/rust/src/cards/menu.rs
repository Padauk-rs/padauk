use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::cards::filled::FilledCardScreen;
use crate::cards::elevated::ElevatedCardScreen;
use crate::cards::outlined::OutlinedCardScreen;

pub struct CardsMenu;

impl Widget for CardsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Cards"),
            button("Filled", || {
                Navigator::push(Route::new("card_filled", || FilledCardScreen {}));
            }),
            button("Elevated", || {
                Navigator::push(Route::new("card_elevated", || ElevatedCardScreen {}));
            }),
            button("Outlined", || {
                Navigator::push(Route::new("card_outlined", || OutlinedCardScreen {}));
            }),
        ]))
        .app_bar(app_bar("Cards"))
        .build()
    }
}
