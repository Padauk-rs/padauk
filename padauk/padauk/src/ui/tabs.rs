use crate::ui::{button::IconType, color::ColorValue};

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum TabsStyle {
    Primary,
    Secondary,
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct TabsOptions {
    pub style: TabsStyle,
    pub scrollable: bool,
    pub container_color: Option<ColorValue>,
    pub content_color: Option<ColorValue>,
    pub indicator_color: Option<ColorValue>,
    pub selected_content_color: Option<ColorValue>,
    pub unselected_content_color: Option<ColorValue>,
    pub divider_color: Option<ColorValue>,
}

impl Default for TabsOptions {
    fn default() -> Self {
        Self {
            style: TabsStyle::Primary,
            scrollable: false,
            container_color: None,
            content_color: None,
            indicator_color: None,
            selected_content_color: None,
            unselected_content_color: None,
            divider_color: None,
        }
    }
}

#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
#[derive(uniffi::Record, Clone, Debug)]
pub struct TabDestination {
    pub label: String,
    pub icon: Option<IconType>,
    pub selected: bool,
    pub action_id: String,
}
