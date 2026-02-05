use crate::ui::color::ColorValue;

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum CardStyle {
    Filled,
    Elevated,
    Outlined,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum CardShape {
    Default,
    Rounded,
    Pill,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct CardStyleOptions {
    pub enabled: bool,
    pub shape: CardShape,
    pub container_color: Option<ColorValue>,
    pub border_color: Option<ColorValue>,
    pub border_width: Option<f32>,
    pub elevation: Option<f32>,
}

impl Default for CardStyleOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            shape: CardShape::Default,
            container_color: None,
            border_color: None,
            border_width: None,
            elevation: None,
        }
    }
}
