use padauk::{app_bar, button, children, column, scaffold, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::buttons::filled::FilledButtonScreen;
use crate::buttons::filled_tonal::FilledTonalButtonScreen;
use crate::buttons::elevated::ElevatedButtonScreen;
use crate::buttons::outlined::OutlinedButtonScreen;
use crate::buttons::text_button::TextButtonScreen;
use crate::buttons::icon_standard::IconButtonStandardScreen;
use crate::buttons::icon_filled::IconButtonFilledScreen;
use crate::buttons::icon_filled_tonal::IconButtonFilledTonalScreen;
use crate::buttons::icon_outlined::IconButtonOutlinedScreen;
use crate::buttons::fab_small::FabSmallScreen;
use crate::buttons::fab_normal::FabNormalScreen;
use crate::buttons::fab_large::FabLargeScreen;
use crate::buttons::fab_extended::FabExtendedScreen;

pub struct ButtonsMenu;

impl Widget for ButtonsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(column(children![
            text("Buttons"),
            button("Filled", || {
                Navigator::push(Route::new("btn_filled", || FilledButtonScreen {}));
            }),
            button("Filled tonal", || {
                Navigator::push(Route::new("btn_filled_tonal", || FilledTonalButtonScreen {}));
            }),
            button("Elevated", || {
                Navigator::push(Route::new("btn_elevated", || ElevatedButtonScreen {}));
            }),
            button("Outlined", || {
                Navigator::push(Route::new("btn_outlined", || OutlinedButtonScreen {}));
            }),
            button("Text", || {
                Navigator::push(Route::new("btn_text", || TextButtonScreen {}));
            }),
            button("Icon standard", || {
                Navigator::push(Route::new("btn_icon_std", || IconButtonStandardScreen {}));
            }),
            button("Icon filled", || {
                Navigator::push(Route::new("btn_icon_filled", || IconButtonFilledScreen {}));
            }),
            button("Icon filled tonal", || {
                Navigator::push(Route::new("btn_icon_tonal", || IconButtonFilledTonalScreen {}));
            }),
            button("Icon outlined", || {
                Navigator::push(Route::new("btn_icon_outlined", || IconButtonOutlinedScreen {}));
            }),
            button("FAB small", || {
                Navigator::push(Route::new("btn_fab_small", || FabSmallScreen {}));
            }),
            button("FAB default", || {
                Navigator::push(Route::new("btn_fab_normal", || FabNormalScreen {}));
            }),
            button("FAB large", || {
                Navigator::push(Route::new("btn_fab_large", || FabLargeScreen {}));
            }),
            button("FAB extended", || {
                Navigator::push(Route::new("btn_fab_extended", || FabExtendedScreen {}));
            }),
        ]))
        .app_bar(app_bar("Buttons"))
        .build()
    }
}
