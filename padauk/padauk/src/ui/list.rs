use crate::ui::{button::IconType, color::ColorValue};

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct ListItemOptions {
    pub enabled: bool,
    pub container_color: Option<ColorValue>,
    pub headline_color: Option<ColorValue>,
    pub supporting_color: Option<ColorValue>,
    pub overline_color: Option<ColorValue>,
    pub leading_color: Option<ColorValue>,
    pub trailing_color: Option<ColorValue>,
    pub tonal_elevation: Option<f32>,
    pub trailing_supporting_text: bool,
}

impl Default for ListItemOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            container_color: None,
            headline_color: None,
            supporting_color: None,
            overline_color: None,
            leading_color: None,
            trailing_color: None,
            tonal_elevation: None,
            trailing_supporting_text: true,
        }
    }
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug, Default)]
pub struct ListItemTrailing {
    pub text: Option<String>,
    pub icon: Option<IconType>,
}
