use padauk::prelude::{Navigator, Route};
use padauk::{app_bar, children, divider, list, list_item, scaffold, Widget};

use crate::text_fields::filled::FilledTextFieldScreen;
use crate::text_fields::outlined::OutlinedTextFieldScreen;

pub struct TextFieldsMenu;

impl Widget for TextFieldsMenu {
    fn build(&self) -> padauk::UiNode {
        scaffold(list(children![
            list_item("Filled").on_click(|| {
                Navigator::push(Route::new("text_field_filled", || FilledTextFieldScreen {}));
            }),
            divider(),
            list_item("Outlined").on_click(|| {
                Navigator::push(Route::new("text_field_outlined", || {
                    OutlinedTextFieldScreen {}
                }));
            }),
        ]))
        .app_bar(app_bar("Text fields"))
        .build()
    }
}
