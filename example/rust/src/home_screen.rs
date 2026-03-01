use padauk::Widget;
use padauk::{
    app_bar, children, divider, list, list_item,
    prelude::{Navigator, Route},
    scaffold,
};

use crate::app_bars::menu::AppBarsMenu;
use crate::buttons::menu::ButtonsMenu;
use crate::cards::menu::CardsMenu;
use crate::checkboxes::demo::CheckboxDemo;
use crate::chips::menu::ChipsMenu;
use crate::date_pickers::demo::DatePickerDemoScreen;
use crate::dialogs::demo::DialogDemoScreen;
use crate::lists::demo::ListsDemoScreen;
use crate::navigation_bars::menu::NavigationBarsMenu;
use crate::navigator::navigation_menu::NavigationMenu;
use crate::scroll::demo::ScrollDemoScreen;
use crate::state_demo::StateDemoScreen;
use crate::text_fields::menu::TextFieldsMenu;

pub struct HomeScreen;

impl Widget for HomeScreen {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Navigation")
                .supporting_text("Navigator push/pop/replace demos")
                .on_click(|| {
                    Navigator::push(Route::new("nav_demo", || NavigationMenu {}));
                }),
            divider(),
            list_item("App bars")
                .supporting_text("Material 3 app bar styles")
                .on_click(|| {
                    Navigator::push(Route::new("app_bars", || AppBarsMenu {}));
                }),
            divider(),
            list_item("Navigation bars")
                .supporting_text("Material 3 bottom navigation bar")
                .on_click(|| {
                    Navigator::push(Route::new("navigation_bars", || NavigationBarsMenu {}));
                }),
            divider(),
            list_item("Buttons")
                .supporting_text("All button variants")
                .on_click(|| {
                    Navigator::push(Route::new("buttons", || ButtonsMenu {}));
                }),
            divider(),
            list_item("Cards")
                .supporting_text("Filled, elevated, outlined cards")
                .on_click(|| {
                    Navigator::push(Route::new("cards", || CardsMenu {}));
                }),
            divider(),
            list_item("Checkbox")
                .supporting_text("Checkbox customization")
                .on_click(|| {
                    Navigator::push(Route::new("checkbox_demo", || CheckboxDemo {}));
                }),
            divider(),
            list_item("Chips")
                .supporting_text("Assist/filter/input/suggestion chips")
                .on_click(|| {
                    Navigator::push(Route::new("chips", || ChipsMenu {}));
                }),
            divider(),
            list_item("Date pickers")
                .supporting_text("Date, range, and time pickers")
                .on_click(|| {
                    Navigator::push(Route::new("date_pickers", || DatePickerDemoScreen {}));
                }),
            divider(),
            list_item("Dialog")
                .supporting_text("Alert and fullscreen dialogs")
                .on_click(|| {
                    Navigator::push(Route::new("dialog_demo", || DialogDemoScreen {}));
                }),
            divider(),
            list_item("Lists")
                .supporting_text("List and divider examples")
                .on_click(|| {
                    Navigator::push(Route::new("list_demo", || ListsDemoScreen {}));
                }),
            divider(),
            list_item("Scroll")
                .supporting_text("Scrollable content usage")
                .on_click(|| {
                    Navigator::push(Route::new("scroll_demo", || ScrollDemoScreen {}));
                }),
            divider(),
            list_item("State")
                .supporting_text("State management sample")
                .on_click(|| {
                    Navigator::push(Route::new("state_demo", || StateDemoScreen {}));
                }),
            divider(),
            list_item("Text fields")
                .supporting_text("Filled and outlined text field samples")
                .on_click(|| {
                    Navigator::push(Route::new("text_fields", || TextFieldsMenu {}));
                }),
        ]))
        .app_bar(app_bar("Examples"))
        .build()
    }
}
