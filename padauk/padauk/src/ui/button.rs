#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Filled,
    FilledTonal,
    Elevated,
    Outlined,
    Text,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum ButtonShape {
    Default,
    Rounded,
    Pill,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct ButtonStyleOptions {
    pub enabled: bool,
    pub shape: ButtonShape,
    pub container_color: Option<crate::ui::color::ColorValue>,
    pub content_color: Option<crate::ui::color::ColorValue>,
    pub border_color: Option<crate::ui::color::ColorValue>,
    pub border_width: Option<f32>,
    pub elevation: Option<f32>,
    pub content_padding: Option<f32>,
}

impl Default for ButtonStyleOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            shape: ButtonShape::Default,
            container_color: None,
            content_color: None,
            border_color: None,
            border_width: None,
            elevation: None,
            content_padding: None,
        }
    }
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum IconButtonStyle {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct IconButtonOptions {
    pub enabled: bool,
    pub shape: ButtonShape,
    pub container_color: Option<crate::ui::color::ColorValue>,
    pub content_color: Option<crate::ui::color::ColorValue>,
}

impl Default for IconButtonOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            shape: ButtonShape::Default,
            container_color: None,
            content_color: None,
        }
    }
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum FabStyle {
    Small,
    Normal,
    Large,
    Extended,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct FabOptions {
    pub shape: ButtonShape,
    pub container_color: Option<crate::ui::color::ColorValue>,
    pub content_color: Option<crate::ui::color::ColorValue>,
    pub elevation: Option<f32>,
}

impl Default for FabOptions {
    fn default() -> Self {
        Self {
            shape: ButtonShape::Default,
            container_color: None,
            content_color: None,
            elevation: None,
        }
    }
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum IconType {
    Add,
    Close,
    Menu,
    Favorite,
    Search,
    Person,
}
