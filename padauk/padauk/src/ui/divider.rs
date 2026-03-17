use crate::ui::color::ColorValue;

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct DividerOptions {
    pub color: Option<ColorValue>,
    pub thickness: Option<f32>,
    pub inset_start: Option<f32>,
    pub inset_end: Option<f32>,
    pub vertical: bool,
}

impl Default for DividerOptions {
    fn default() -> Self {
        Self {
            color: None,
            thickness: None,
            inset_start: None,
            inset_end: None,
            vertical: false,
        }
    }
}
