use padauk::prelude::*;

use crate::generated::assets;

mod generated;

uniffi::setup_scaffolding!();

struct ExampleApp {}

impl PadaukApp for ExampleApp {
    // 1. Define the entry point here
    fn initial_route(&self) -> Option<Route> {
        Some(Route::new("home", || {
            scaffold(text("Hello Navigation!")).app_bar(app_bar("Home"))
        }))
    }

    // 2. This becomes a dummy or fallback implementation
    fn render(&self) -> Box<dyn Widget> {
        // This is only shown if initial_route returns None
        text("Loading...").into_widget()
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

#[padauk::main]
fn start() {
    ExampleApp {}
}
