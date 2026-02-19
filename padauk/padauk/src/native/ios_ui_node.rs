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
    FullscreenDialog {
        title: String,
        content: Vec<IosUiNode>,
        confirm_label: Option<String>,
        confirm_action_id: Option<String>,
        dismiss_label: String,
        dismiss_action_id: String,
        dismissible: bool,
        attributes: Modifiers,
    },
    DatePickerDialog {
        title: Option<String>,
        initial_selected_millis: Option<i64>,
        show_mode_toggle: bool,
        confirm_label: String,
        confirm_action_id: String,
        dismiss_label: Option<String>,
        dismiss_action_id: Option<String>,
        dismissible: bool,
        attributes: Modifiers,
    },
    DateRangePickerDialog {
        title: Option<String>,
        initial_start_millis: Option<i64>,
        initial_end_millis: Option<i64>,
        show_mode_toggle: bool,
        confirm_label: String,
        confirm_action_id: String,
        dismiss_label: Option<String>,
        dismiss_action_id: Option<String>,
        dismissible: bool,
        attributes: Modifiers,
    },
    TimePickerDialog {
        title: Option<String>,
        initial_hour: Option<i32>,
        initial_minute: Option<i32>,
        is_24_hour: bool,
        show_mode_toggle: bool,
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
