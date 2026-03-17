use crate::ui::{button::IconType, color::ColorValue};

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationBarOptions {
    pub container_color: Option<ColorValue>,
    pub content_color: Option<ColorValue>,
    pub indicator_color: Option<ColorValue>,
    pub selected_icon_color: Option<ColorValue>,
    pub selected_text_color: Option<ColorValue>,
    pub unselected_icon_color: Option<ColorValue>,
    pub unselected_text_color: Option<ColorValue>,
    pub always_show_label: bool,
}

impl Default for NavigationBarOptions {
    fn default() -> Self {
        Self {
            container_color: None,
            content_color: None,
            indicator_color: None,
            selected_icon_color: None,
            selected_text_color: None,
            unselected_icon_color: None,
            unselected_text_color: None,
            always_show_label: true,
        }
    }
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationDestination {
    pub label: String,
    pub icon: IconType,
    pub selected: bool,
    pub action_id: String,
}
