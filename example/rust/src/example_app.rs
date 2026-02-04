use padauk::{prelude::Route, PadaukApp};

use crate::home_screen::HomeScreen;
pub struct ExampleApp {}

impl PadaukApp for ExampleApp {
    // 1. Define the entry point here
    fn initial_route(&self) -> Route {
        Route::new("home", || HomeScreen {})
    }

    // fn render(&self) -> Box<dyn Widget> {
    //     let image_data =
    //         padauk::native::platform::load_raw(assets::raw::MEMORY).unwrap_or_else(|_| vec![]); // Handle error gracefully

    //     scaffold(column(children![
    //         image_asset(assets::image::ASSET)
    //             .size(Some(300.0), Some(300.0))
    //             .fit(ui::image::BoxFit::Fill),
    //         image_memory(image_data)
    //             .size(Some(300.0), Some(300.0))
    //             .fit(ui::image::BoxFit::Fill),
    //     ]))
    //     .app_bar(app_bar("Images"))
    //     .into_widget()
    // }
}
