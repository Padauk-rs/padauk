use crate::ui::color::ColorValue;

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum CardStyle {
    Filled,
    Elevated,
    Outlined,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum CardShape {
    Default,
    Rounded,
    Pill,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
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
