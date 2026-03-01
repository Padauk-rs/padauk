use crate::ui::{button::IconType, color::ColorValue};

#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationRailOptions {
    pub container_color: Option<ColorValue>,
    pub content_color: Option<ColorValue>,
    pub indicator_color: Option<ColorValue>,
    pub selected_icon_color: Option<ColorValue>,
    pub selected_text_color: Option<ColorValue>,
    pub unselected_icon_color: Option<ColorValue>,
    pub unselected_text_color: Option<ColorValue>,
    pub always_show_label: bool,
    pub expanded: bool,
    pub allow_toggle: bool,
}

impl Default for NavigationRailOptions {
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
            expanded: false,
            allow_toggle: false,
        }
    }
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct NavigationRailDestination {
    pub label: String,
    pub icon: IconType,
    pub selected: bool,
    pub action_id: String,
}
