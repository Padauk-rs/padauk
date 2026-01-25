use padauk::{
    ui::image::{image_asset, image_memory},
    *,
};

use crate::generated::assets;

mod generated;

uniffi::setup_scaffolding!();

struct ExampleApp {}

impl PadaukApp for ExampleApp {
    fn render(&self) -> Box<dyn Widget> {
        let image_data =
            padauk::native::platform::load_raw(assets::raw::MEMORY).unwrap_or_else(|_| vec![]); // Handle error gracefully

        scaffold(column(children![
            image_asset(assets::image::ASSET)
                .size(Some(300.0), Some(300.0))
                .fit(ui::image::BoxFit::Fill),
            image_memory(image_data)
                .size(Some(300.0), Some(300.0))
                .fit(ui::image::BoxFit::Fill),
        ]))
        .app_bar(app_bar("Images"))
        .into_widget()
    }
}

#[padauk::main]
fn start() {
    ExampleApp {}
}
