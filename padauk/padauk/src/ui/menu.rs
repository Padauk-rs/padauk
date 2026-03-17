use crate::ui::text_field::TextFieldStyle;

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct MenuItem {
    pub label: String,
    pub enabled: bool,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct DropdownFieldOptions {
    pub placeholder: Option<String>,
    pub supporting_text: Option<String>,
    pub enabled: bool,
    pub style: TextFieldStyle,
}

impl Default for DropdownFieldOptions {
    fn default() -> Self {
        Self {
            placeholder: None,
            supporting_text: None,
            enabled: true,
            style: TextFieldStyle::Outlined,
        }
    }
}
