use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::cards::elevated::ElevatedCardScreen;
use crate::cards::filled::FilledCardScreen;
use crate::cards::outlined::OutlinedCardScreen;

pub struct CardsMenu;

impl Widget for CardsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Filled").on_click(|| {
                Navigator::push(Route::new("card_filled", || FilledCardScreen {}));
            }),
            divider(),
            list_item("Elevated").on_click(|| {
                Navigator::push(Route::new("card_elevated", || ElevatedCardScreen {}));
            }),
            divider(),
            list_item("Outlined").on_click(|| {
                Navigator::push(Route::new("card_outlined", || OutlinedCardScreen {}));
            }),
        ]))
        .app_bar(app_bar("Cards"))
        .build()
    }
}
