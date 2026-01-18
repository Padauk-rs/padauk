#[cfg(not(target_os = "ios"))]
pub use crate::native::android_ui_node::AndroidUiNode;

#[cfg(target_os = "ios")]
pub use crate::native::ios_ui_node::IosUiNode;

use crate::{impl_modifiers, ui::modifier::Modifiers};

// --------------------------------------------------------
// THE SWITCH: Choose the definition based on the OS
// --------------------------------------------------------

#[cfg(target_os = "ios")]
pub use nodes::android::IosUiNode as UiNode;

#[cfg(not(target_os = "ios"))] // Fallback for iOS/Tests
pub use AndroidUiNode as UiNode;

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
                modifier: self.modifiers.clone(),
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

pub struct Button {
    pub label: String,
    pub action_id: String, // The UUID from the Registry
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
                action: self.action.clone(),
                // FIX: Wrap in Arc::new
                label: std::sync::Arc::new(child_node),
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
                modifier: Modifiers::default(),
            };

            UiNode::Button {
                action_id: self.action_id.clone(),
                // FIX: Wrap in Arc::new
                content: vec![child_node],
                modifier: self.modifiers.clone(),
            }
        }
    }
}

impl Button {
    // Standard constructor
    pub fn new(label: impl Into<String>, action_id: String) -> Self {
        Self {
            label: label.into(),
            action_id,
            modifiers: Modifiers::default(),
        }
    }
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
                modifier: self.modifiers.clone(),
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
