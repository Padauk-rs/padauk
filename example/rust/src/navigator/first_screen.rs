use padauk::{app_bar, button, children, column, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::example_layout::example_screen;
use crate::navigator::second_screen::SecondScreen;
use crate::navigator::third_screen::ThirdScreen;

const CODE: &str = r#"// Push
button("Push Second", || {
    Navigator::push(Route::new("second", || SecondScreen {}));
});

// Push another
button("Push Third", || {
    Navigator::push(Route::new("third", || ThirdScreen {}));
});

// Replace
button("Replace with Third", || {
    Navigator::replace(Route::new("third", || ThirdScreen {}));
});"#;

pub struct FirstScreen;

impl Widget for FirstScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("First"),
            column(children![
                text("This is the first screen!"),
                button("Push Second", || {
                    Navigator::push(Route::new("second", || SecondScreen {}));
                }),
                button("Push Third", || {
                    Navigator::push(Route::new("third", || ThirdScreen {}));
                }),
                button("Replace with Third", || {
                    Navigator::replace(Route::new("third", || ThirdScreen {}));
                }),
                button("Pop To First", || {
                    Navigator::pop_to_first();
                }),
                button("Pop Until Demo Root", || {
                    Navigator::pop_until("nav_demo");
                }),
                button("Pop Til Demo Root", || {
                    Navigator::pop_til("nav_demo");
                }),
            ]),
            CODE,
        )
    }
}
