#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum AppBarStyle {
    Small,
    CenterAligned,
    Medium,
    Large,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct AppBarStyleOptions {
    pub container_color: Option<crate::ui::color::ColorValue>,
    pub title_color: Option<crate::ui::color::ColorValue>,
    pub nav_icon_color: Option<crate::ui::color::ColorValue>,
}

impl Default for AppBarStyleOptions {
    fn default() -> Self {
        Self {
            container_color: None,
            title_color: None,
            nav_icon_color: None,
        }
    }
}
