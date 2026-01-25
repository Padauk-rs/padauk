#[derive(uniffi::Record, Default, Clone)]
pub struct Modifiers {
    pub padding: f32,
    pub background_color: Option<String>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
