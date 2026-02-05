#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Filled,
    FilledTonal,
    Elevated,
    Outlined,
    Text,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum IconButtonStyle {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum FabStyle {
    Small,
    Normal,
    Large,
    Extended,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum IconType {
    Add,
    Close,
    Menu,
    Favorite,
    Search,
}
