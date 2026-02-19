use crate::ui::{
    app_bar::{AppBarStyle, AppBarStyleOptions},
    button::{
        ButtonShape, ButtonStyle, ButtonStyleOptions, FabOptions, FabStyle, IconButtonOptions,
        IconButtonStyle, IconType,
    },
    card::{CardShape, CardStyle, CardStyleOptions},
    chip::{ChipStyle, ChipStyleOptions},
    divider::DividerOptions,
    image::{BoxFit, ImageSource},
    list::{ListItemOptions, ListItemTrailing},
    modifier::Modifiers,
};

// Android-specific definitions
#[derive(uniffi::Enum, Clone)]
pub enum AndroidUiNode {
    // --- Layouts ---
    Column {
        children: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    Row {
        children: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    Stack {
        children: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    Dialog {
        title: Option<String>,
        text: String,
        confirm_label: String,
        confirm_action_id: String,
        dismiss_label: Option<String>,
        dismiss_action_id: Option<String>,
        dismissible: bool,
        modifiers: Modifiers,
    },
    FullscreenDialog {
        title: String,
        content: Vec<AndroidUiNode>,
        confirm_label: Option<String>,
        confirm_action_id: Option<String>,
        dismiss_label: String,
        dismiss_action_id: String,
        dismissible: bool,
        modifiers: Modifiers,
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
        modifiers: Modifiers,
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
        modifiers: Modifiers,
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
        modifiers: Modifiers,
    },
    Scroll {
        child: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    ListView {
        items: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    ListItem {
        headline: String,
        supporting_text: Option<String>,
        overline_text: Option<String>,
        leading_icon: Option<IconType>,
        trailing: ListItemTrailing,
        action_id: Option<String>,
        options: ListItemOptions,
        modifiers: Modifiers,
    },
    Divider {
        options: DividerOptions,
        modifiers: Modifiers,
    },

    // --- Phase 1: Structural Components ---
    Scaffold {
        // We use Vec as a workaround for Option<Box<UiNode>> in UniFFI Enums.
        // Empty Vec = None, Vec with 1 item = Some.
        app_bar: Vec<AndroidUiNode>,
        body: Vec<AndroidUiNode>,
        floating_action_button: Vec<AndroidUiNode>,
        modifiers: Modifiers,
    },
    AppBar {
        title: String,
        leading: Vec<AndroidUiNode>,
        style: AppBarStyle,
        options: AppBarStyleOptions,
        // Future: actions: Vec<UiNode>,
        modifiers: Modifiers,
    },

    // --- Primitives ---
    Text {
        text: String,
        sp_size: f32, // Android uses SP for fonts
        modifiers: Modifiers,
    },
    Button {
        action_id: String,
        content: Vec<AndroidUiNode>, // Using Vec as workaround to avoid uniffi error in Box/Arc
        style: ButtonStyle,
        options: ButtonStyleOptions,
        modifiers: Modifiers,
    },
    IconButton {
        action_id: String,
        icon: IconType,
        style: IconButtonStyle,
        options: IconButtonOptions,
        modifiers: Modifiers,
    },
    Card {
        children: Vec<AndroidUiNode>,
        style: CardStyle,
        action_id: Option<String>,
        options: CardStyleOptions,
        modifiers: Modifiers,
    },
    Checkbox {
        checked: bool,
        action_id: String,
        enabled: bool,
        color_checked: Option<crate::ui::color::ColorValue>,
        color_unchecked: Option<crate::ui::color::ColorValue>,
        color_checkmark: Option<crate::ui::color::ColorValue>,
        modifiers: Modifiers,
    },
    Chip {
        label: String,
        style: ChipStyle,
        selected: bool,
        action_id: String,
        leading_icon: Option<IconType>,
        trailing_icon: Option<IconType>,
        close_action_id: Option<String>,
        options: ChipStyleOptions,
        modifiers: Modifiers,
    },
    Fab {
        action_id: String,
        icon: IconType,
        style: FabStyle,
        label: Option<String>,
        options: FabOptions,
        modifiers: Modifiers,
    },
    Image {
        source: ImageSource,
        fit: BoxFit,
        modifiers: Modifiers,
    },
}
