use padauk::{app_bar, button, children, column, text, Widget};
use padauk::prelude::{Navigator, Route};

use crate::example_layout::example_screen;
use crate::navigator::third_screen::ThirdScreen;

const CODE: &str = r#"// Push
button("Push Third", || {
    Navigator::push(Route::new("third", || ThirdScreen {}));
});

// Replace
button("Replace with Third", || {
    Navigator::replace(Route::new("third", || ThirdScreen {}));
});

// Pop variants
button("Pop Until Demo Root", || {
    Navigator::pop_until("nav_demo");
});
button("Pop Til Demo Root", || {
    Navigator::pop_til("nav_demo");
});
button("Pop To First", || {
    Navigator::pop_to_first();
});"#;

pub struct SecondScreen;

impl Widget for SecondScreen {
    fn build(&self) -> padauk::UiNode {
        example_screen(
            app_bar("Second"),
            column(children![
                text("Second Screen"),
                button("Push Third", || {
                    Navigator::push(Route::new("third", || ThirdScreen {}));
                }),
                button("Replace with Third", || {
                    Navigator::replace(Route::new("third", || ThirdScreen {}));
                }),
                button("Pop Until Demo Root", || {
                    Navigator::pop_until("nav_demo");
                }),
                button("Pop Til Demo Root", || {
                    Navigator::pop_til("nav_demo");
                }),
                button("Pop To First", || {
                    Navigator::pop_to_first();
                }),
            ]),
            CODE,
        )
    }
}
