use crate::ui::button::IconType;

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum TextFieldStyle {
    Filled,
    Outlined,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone)]
pub struct TextFieldOptions {
    pub placeholder: Option<String>,
    pub supporting_text: Option<String>,
    pub enabled: bool,
    pub read_only: bool,
    pub single_line: bool,
    pub max_lines: i32,
    pub is_password: bool,
    pub leading_icon: Option<IconType>,
    pub trailing_icon: Option<IconType>,
}

impl Default for TextFieldOptions {
    fn default() -> Self {
        Self {
            placeholder: None,
            supporting_text: None,
            enabled: true,
            read_only: false,
            single_line: false,
            max_lines: 4,
            is_password: false,
            leading_icon: None,
            trailing_icon: None,
        }
    }
}
