use crate::ui::modifier::Modifiers;

// Android-specific definitions
#[derive(uniffi::Enum, Clone)]
pub enum AndroidUiNode {
    // Matches Jetpack Compose naming
    Column {
        children: Vec<AndroidUiNode>,
        modifier: Modifiers,
    },
    Text {
        text: String,
        sp_size: f32, // Android uses SP for fonts
        modifier: Modifiers,
    },
    Button {
        action_id: String,
        content: Vec<AndroidUiNode>, // Using Vec as workaround to avoid uniffi error in Box/Arc
        modifier: Modifiers,
    },
}
