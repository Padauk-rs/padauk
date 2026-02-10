use crate::ui::modifier::Modifiers;

// iOS-specific definitions
#[derive(uniffi::Enum, Clone)]
pub enum IosUiNode {
    // Matches SwiftUI naming
    VStack {
        views: Vec<IosUiNode>,
        attributes: Modifiers,
    },
    Dialog {
        title: Option<String>,
        text: String,
        confirm_label: String,
        confirm_action_id: String,
        dismiss_label: Option<String>,
        dismiss_action_id: Option<String>,
        dismissible: bool,
        attributes: Modifiers,
    },
    ScrollView {
        views: Vec<IosUiNode>,
        attributes: Modifiers,
    },
    Label {
        title: String,
        pt_size: f32, // iOS uses Points
        attributes: Modifiers,
    },
    Button {
        action_id: String,
        label: Vec<IosUiNode>,
        attributes: Modifiers,
    },
}
