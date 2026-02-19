use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::buttons::elevated::ElevatedButtonScreen;
use crate::buttons::fab_extended::FabExtendedScreen;
use crate::buttons::fab_large::FabLargeScreen;
use crate::buttons::fab_normal::FabNormalScreen;
use crate::buttons::fab_small::FabSmallScreen;
use crate::buttons::filled::FilledButtonScreen;
use crate::buttons::filled_tonal::FilledTonalButtonScreen;
use crate::buttons::icon_filled::IconButtonFilledScreen;
use crate::buttons::icon_filled_tonal::IconButtonFilledTonalScreen;
use crate::buttons::icon_outlined::IconButtonOutlinedScreen;
use crate::buttons::icon_standard::IconButtonStandardScreen;
use crate::buttons::outlined::OutlinedButtonScreen;
use crate::buttons::text_button::TextButtonScreen;

pub struct ButtonsMenu;

impl Widget for ButtonsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Filled").on_click(|| {
                Navigator::push(Route::new("btn_filled", || FilledButtonScreen {}));
            }),
            divider(),
            list_item("Filled tonal").on_click(|| {
                Navigator::push(Route::new(
                    "btn_filled_tonal",
                    || FilledTonalButtonScreen {},
                ));
            }),
            divider(),
            list_item("Elevated").on_click(|| {
                Navigator::push(Route::new("btn_elevated", || ElevatedButtonScreen {}));
            }),
            divider(),
            list_item("Outlined").on_click(|| {
                Navigator::push(Route::new("btn_outlined", || OutlinedButtonScreen {}));
            }),
            divider(),
            list_item("Text").on_click(|| {
                Navigator::push(Route::new("btn_text", || TextButtonScreen {}));
            }),
            divider(),
            list_item("Icon standard").on_click(|| {
                Navigator::push(Route::new("btn_icon_std", || IconButtonStandardScreen {}));
            }),
            divider(),
            list_item("Icon filled").on_click(|| {
                Navigator::push(Route::new("btn_icon_filled", || IconButtonFilledScreen {}));
            }),
            divider(),
            list_item("Icon filled tonal").on_click(|| {
                Navigator::push(Route::new("btn_icon_tonal", || {
                    IconButtonFilledTonalScreen {}
                }));
            }),
            divider(),
            list_item("Icon outlined").on_click(|| {
                Navigator::push(Route::new("btn_icon_outlined", || {
                    IconButtonOutlinedScreen {}
                }));
            }),
            divider(),
            list_item("FAB small").on_click(|| {
                Navigator::push(Route::new("btn_fab_small", || FabSmallScreen {}));
            }),
            divider(),
            list_item("FAB default").on_click(|| {
                Navigator::push(Route::new("btn_fab_normal", || FabNormalScreen {}));
            }),
            divider(),
            list_item("FAB large").on_click(|| {
                Navigator::push(Route::new("btn_fab_large", || FabLargeScreen {}));
            }),
            divider(),
            list_item("FAB extended").on_click(|| {
                Navigator::push(Route::new("btn_fab_extended", || FabExtendedScreen {}));
            }),
        ]))
        .app_bar(app_bar("Buttons"))
        .build()
    }
}
