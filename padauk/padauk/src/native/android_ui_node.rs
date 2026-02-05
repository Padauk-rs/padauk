use crate::ui::{
    app_bar::AppBarStyle,
    button::{
        ButtonShape,
        ButtonStyle,
        ButtonStyleOptions,
        FabOptions,
        FabStyle,
        IconButtonOptions,
        IconButtonStyle,
        IconType,
    },
    card::{CardShape, CardStyle, CardStyleOptions},
    chip::{ChipStyle, ChipStyleOptions},
    image::{BoxFit, ImageSource},
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
