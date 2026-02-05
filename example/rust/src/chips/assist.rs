use padauk::{app_bar, assist_chip, column, scaffold, text, Widget};
use padauk::prelude::{ChipShape, ChipStyleOptions, color_hex};
use padauk::prelude::IconType;

pub struct AssistChipScreen;

impl Widget for AssistChipScreen {
    fn build(&self) -> padauk::UiNode {
        let options = ChipStyleOptions {
            enabled: true,
            shape: ChipShape::Pill,
            container_color: Some(color_hex("#E8F0FE")),
            label_color: None,
            icon_color: None,
            border_color: None,
            border_width: None,
            elevation: None,
        };
        let c = assist_chip("Assist", || {})
            .leading_icon(IconType::Search)
            .options(options);

        let t = text("Use assist chips for contextual actions.");
        scaffold(column(vec![Box::new(c), Box::new(t)]))
            .app_bar(app_bar("Assist Chip"))
            .build()
    }
}
