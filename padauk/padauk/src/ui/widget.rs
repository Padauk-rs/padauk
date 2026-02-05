#[cfg(not(target_os = "ios"))]
pub use crate::native::android_ui_node::AndroidUiNode;

#[cfg(target_os = "ios")]
pub use crate::native::ios_ui_node::IosUiNode;

use crate::{
    impl_modifiers,
    prelude::Navigator,
    ui::{
        app_bar::AppBarStyle,
        button::{ButtonStyle, FabStyle, IconButtonStyle, IconType},
        card::CardStyle,
        modifier::Modifiers,
    },
};
use log::debug;

// --------------------------------------------------------
// THE SWITCH: Choose the definition based on the OS
// --------------------------------------------------------

#[cfg(target_os = "ios")]
pub use IosUiNode as UiNode;

#[cfg(not(target_os = "ios"))] // Fallback for iOS/Tests
pub use AndroidUiNode as UiNode;
use uuid::Uuid;

// This is equivalent to Flutter's "abstract class Widget"
pub trait Widget {
    // Equivalent to: Widget build(BuildContext context)
    fn build(&self) -> UiNode;
}

pub trait IntoWidget {
    fn into_widget(self) -> Box<dyn Widget>;
}

impl<T: Widget + Sized + 'static> IntoWidget for T {
    fn into_widget(self) -> Box<dyn Widget> {
        Box::new(self)
    }
}

// ==========================
//      SCAFFOLD WIDGET
// ==========================

pub struct Scaffold {
    pub app_bar: Option<Box<dyn Widget>>,
    pub body: Box<dyn Widget>,
    pub fab: Option<Box<dyn Widget>>,
    pub modifiers: Modifiers,
}

impl Scaffold {
    pub fn new(body: impl Widget + 'static) -> Self {
        Self {
            body: Box::new(body),
            app_bar: None,
            fab: None,
            modifiers: Modifiers::default(),
        }
    }

    pub fn app_bar(mut self, bar: impl Widget + 'static) -> Self {
        self.app_bar = Some(Box::new(bar));
        self
    }

    pub fn fab(mut self, button: impl Widget + 'static) -> Self {
        self.fab = Some(Box::new(button));
        self
    }
}

impl_modifiers!(Scaffold);

impl Widget for Scaffold {
    fn build(&self) -> UiNode {
        // 1. Build the AppBar Node first
        let mut app_bar_nodes: Vec<UiNode> = Vec::new();

        if let Some(bar) = &self.app_bar {
            let mut node = bar.build();

            // 2. Logic: Inject Back Button if Global Navigator says we can pop
            if Navigator::can_pop() {
                debug!("Scaffold: injecting back button into AppBar.");
                if let UiNode::AppBar { leading, .. } = &mut node {
                    // Create a Back Button
                    let back_btn = Button::new("<", || {
                        Navigator::pop();
                    });

                    // Inject into the 'leading' slot of the AppBar node
                    leading.push(back_btn.build());
                }
            }
            app_bar_nodes.push(node);
        }

        // Helper to convert Option<Box<Widget>> -> Vec<UiNode>
        let to_vec = |opt: &Option<Box<dyn Widget>>| -> Vec<UiNode> {
            match opt {
                Some(w) => vec![w.build()],
                None => vec![],
            }
        };

        UiNode::Scaffold {
            app_bar: app_bar_nodes,
            body: vec![self.body.build()],
            floating_action_button: to_vec(&self.fab),
            modifiers: self.modifiers.clone(),
        }
    }
}

// DSL Helper
pub fn scaffold(body: impl Widget + 'static) -> Scaffold {
    Scaffold::new(body)
}

// ==========================
//      APP BAR WIDGET
// ==========================

pub struct AppBar {
    pub title: String,
    pub style: AppBarStyle,
    pub modifiers: Modifiers,
}

impl AppBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            style: AppBarStyle::Small,
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: AppBarStyle) -> Self {
        self.style = style;
        self
    }
}

impl_modifiers!(AppBar);

impl Widget for AppBar {
    fn build(&self) -> UiNode {
        UiNode::AppBar {
            title: self.title.clone(),
            leading: vec![], // Default empty, populated by Scaffold if needed
            style: self.style,
            modifiers: self.modifiers.clone(),
        }
    }
}

pub fn app_bar(title: impl Into<String>) -> AppBar {
    AppBar::new(title)
}

pub fn app_bar_center_aligned(title: impl Into<String>) -> AppBar {
    AppBar::new(title).style(AppBarStyle::CenterAligned)
}

pub fn app_bar_medium(title: impl Into<String>) -> AppBar {
    AppBar::new(title).style(AppBarStyle::Medium)
}

pub fn app_bar_large(title: impl Into<String>) -> AppBar {
    AppBar::new(title).style(AppBarStyle::Large)
}

// --- Primitives ---

pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub modifiers: Modifiers,
}

impl_modifiers!(Text);

impl Widget for Text {
    fn build(&self) -> UiNode {
        // --- IOS BUILD LOGIC ---
        #[cfg(target_os = "ios")]
        {
            UiNode::Label {
                title: self.content.clone(),
                // Logic: Convert abstract "size" to iOS Points (if different)
                pt_size: self.font_size,
                attributes: self.modifiers.clone(),
            }
        }

        // --- ANDROID BUILD LOGIC ---
        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Text {
                text: self.content.clone(),
                // Logic: Convert abstract "size" to Android SP
                sp_size: self.font_size,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: 16.0, // ✅ Your Custom Default Value
            modifiers: Modifiers::default(),
        }
    }
}

// Returns 'Text', NOT 'Box<dyn Widget>'
pub fn text(content: &str) -> Text {
    Text::new(content)
}

pub struct Button {
    pub label: String,
    pub action_id: String,
    pub style: ButtonStyle,
    pub modifiers: Modifiers,
}

impl_modifiers!(Button);

impl Widget for Button {
    fn build(&self) -> UiNode {
        // --- iOS ---
        #[cfg(target_os = "ios")]
        {
            let child_node = UiNode::Label {
                title: self.label.clone(),
                pt_size: 16.0,
                attributes: Modifiers::default(),
            };

            UiNode::Button {
                action_id: self.action_id.clone(),
                // FIX: Wrap in Arc::new
                label: vec![child_node],
                attributes: self.modifiers.clone(),
            }
        }

        // --- Android ---
        #[cfg(not(target_os = "ios"))]
        {
            // Create the child node (e.g. Text)
            let child_node = UiNode::Text {
                text: self.label.clone(),
                sp_size: 16.0,
                modifiers: Modifiers::default(),
            };

            UiNode::Button {
                action_id: self.action_id.clone(),
                // FIX: Wrap in Arc::new
                content: vec![child_node],
                style: self.style,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Button {
    // Standard constructor
    pub fn new(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        debug!("Button created with action id: {}", action_id);

        // Register the closure in our static map
        crate::ui::event_registry::register_action(action_id.clone(), on_click);

        Self {
            label: label.into(),
            action_id: action_id,
            style: ButtonStyle::Filled,
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
}

pub fn filled_button(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Button {
    Button::new(label, on_click).style(ButtonStyle::Filled)
}

pub fn filled_tonal_button(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Button {
    Button::new(label, on_click).style(ButtonStyle::FilledTonal)
}

pub fn elevated_button(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Button {
    Button::new(label, on_click).style(ButtonStyle::Elevated)
}

pub fn outlined_button(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Button {
    Button::new(label, on_click).style(ButtonStyle::Outlined)
}

pub fn text_button(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Button {
    Button::new(label, on_click).style(ButtonStyle::Text)
}

pub struct IconButton {
    pub icon: IconType,
    pub style: IconButtonStyle,
    pub action_id: String,
    pub modifiers: Modifiers,
}

impl_modifiers!(IconButton);

impl Widget for IconButton {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            // TODO: iOS icon buttons
            UiNode::Label {
                title: "Icon".to_string(),
                pt_size: 16.0,
                attributes: Modifiers::default(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::IconButton {
                action_id: self.action_id.clone(),
                icon: self.icon,
                style: self.style,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl IconButton {
    pub fn new(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);
        Self {
            icon,
            style: IconButtonStyle::Standard,
            action_id,
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }
}

pub fn icon_button(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> IconButton {
    IconButton::new(icon, on_click)
}

pub fn filled_icon_button(
    icon: IconType,
    on_click: impl Fn() + Send + Sync + 'static,
) -> IconButton {
    IconButton::new(icon, on_click).style(IconButtonStyle::Filled)
}

pub fn filled_tonal_icon_button(
    icon: IconType,
    on_click: impl Fn() + Send + Sync + 'static,
) -> IconButton {
    IconButton::new(icon, on_click).style(IconButtonStyle::FilledTonal)
}

pub fn outlined_icon_button(
    icon: IconType,
    on_click: impl Fn() + Send + Sync + 'static,
) -> IconButton {
    IconButton::new(icon, on_click).style(IconButtonStyle::Outlined)
}

pub struct Card {
    pub children: Vec<Box<dyn Widget>>,
    pub style: CardStyle,
    pub action_id: Option<String>,
    pub modifiers: Modifiers,
}

impl_modifiers!(Card);

impl Widget for Card {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::Label {
                title: "Card".to_string(),
                pt_size: 16.0,
                attributes: Modifiers::default(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Card {
                children: self.children.iter().map(|c| c.build()).collect(),
                style: self.style,
                action_id: self.action_id.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Card {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            children,
            style: CardStyle::Filled,
            action_id: None,
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);
        self.action_id = Some(action_id);
        self
    }
}

pub fn card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children)
}

pub fn elevated_card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children).style(CardStyle::Elevated)
}

pub fn outlined_card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children).style(CardStyle::Outlined)
}

pub struct Fab {
    pub icon: IconType,
    pub style: FabStyle,
    pub label: Option<String>,
    pub action_id: String,
    pub modifiers: Modifiers,
}

impl_modifiers!(Fab);

impl Widget for Fab {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::Label {
                title: "FAB".to_string(),
                pt_size: 16.0,
                attributes: Modifiers::default(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Fab {
                action_id: self.action_id.clone(),
                icon: self.icon,
                style: self.style,
                label: self.label.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Fab {
    pub fn new(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);
        Self {
            icon,
            style: FabStyle::Normal,
            label: None,
            action_id,
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: FabStyle) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

pub fn fab(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Fab {
    Fab::new(icon, on_click).style(FabStyle::Normal)
}

pub fn fab_small(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Fab {
    Fab::new(icon, on_click).style(FabStyle::Small)
}

pub fn fab_large(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Fab {
    Fab::new(icon, on_click).style(FabStyle::Large)
}

pub fn fab_extended(
    icon: IconType,
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Fab {
    Fab::new(icon, on_click)
        .style(FabStyle::Extended)
        .label(label)
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Button {
    Button::new(label, on_click)
}

pub struct Column {
    pub children: Vec<Box<dyn Widget>>,
    pub modifiers: Modifiers,
}

impl_modifiers!(Column);

impl Widget for Column {
    fn build(&self) -> UiNode {
        // Recursively build children
        let node_children = self.children.iter().map(|child| child.build()).collect();

        // 2. Return the Platform-Specific Node
        #[cfg(target_os = "ios")]
        {
            UiNode::VStack {
                views: node_children, // Matches Vec<IosUiNode>
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Column {
                children: node_children, // Matches Vec<AndroidUiNode>
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Column {
    // Constructor
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            children,
            modifiers: Modifiers::default(),
        }
    }
}

pub fn column(children: Vec<Box<dyn Widget>>) -> Column {
    Column::new(children)
}
