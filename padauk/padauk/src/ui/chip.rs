use crate::ui::color::ColorValue;

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum ChipStyle {
    Assist,
    Filter,
    Input,
    Suggestion,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum ChipShape {
    Default,
    Pill,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct ChipStyleOptions {
    pub enabled: bool,
    pub shape: ChipShape,
    pub container_color: Option<ColorValue>,
    pub label_color: Option<ColorValue>,
    pub icon_color: Option<ColorValue>,
    pub border_color: Option<ColorValue>,
    pub border_width: Option<f32>,
    pub elevation: Option<f32>,
}

impl Default for ChipStyleOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            shape: ChipShape::Default,
            container_color: None,
            label_color: None,
            icon_color: None,
            border_color: None,
            border_width: None,
            elevation: None,
        }
    }
}
