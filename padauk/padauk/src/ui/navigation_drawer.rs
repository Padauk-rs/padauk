use crate::ui::{button::IconType, color::ColorValue};

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum NavigationDrawerType {
    Modal,
    Dismissible,
    Permanent,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationDrawerOptions {
    pub gestures_enabled: bool,
    pub container_color: Option<ColorValue>,
    pub content_color: Option<ColorValue>,
    pub indicator_color: Option<ColorValue>,
    pub selected_icon_color: Option<ColorValue>,
    pub selected_text_color: Option<ColorValue>,
    pub unselected_icon_color: Option<ColorValue>,
    pub unselected_text_color: Option<ColorValue>,
}

impl Default for NavigationDrawerOptions {
    fn default() -> Self {
        Self {
            gestures_enabled: true,
            container_color: None,
            content_color: None,
            indicator_color: None,
            selected_icon_color: None,
            selected_text_color: None,
            unselected_icon_color: None,
            unselected_text_color: None,
        }
    }
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationDrawerDestination {
    pub label: String,
    pub icon: IconType,
    pub selected: bool,
    pub badge: Option<String>,
    pub action_id: String,
}
