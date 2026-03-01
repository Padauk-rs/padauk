use crate::{
    impl_modifiers,
    prelude::Navigator,
    ui::{
        modifier::Modifiers,
        widget::{Button, UiNode, Widget},
    },
};
use log::debug;

// ==========================
//      SCAFFOLD WIDGET
// ==========================

pub struct Scaffold {
    pub app_bar: Option<Box<dyn Widget>>,
    pub drawer: Option<Box<dyn Widget>>,
    pub body: Box<dyn Widget>,
    pub bottom_bar: Option<Box<dyn Widget>>,
    pub fab: Option<Box<dyn Widget>>,
    pub modifiers: Modifiers,
}

impl Scaffold {
    pub fn new(body: impl Widget + 'static) -> Self {
        Self {
            body: Box::new(body),
            app_bar: None,
            drawer: None,
            bottom_bar: None,
            fab: None,
            modifiers: Modifiers::default(),
        }
    }

    pub fn app_bar(mut self, bar: impl Widget + 'static) -> Self {
        self.app_bar = Some(Box::new(bar));
        self
    }

    pub fn drawer(mut self, drawer: impl Widget + 'static) -> Self {
        self.drawer = Some(Box::new(drawer));
        self
    }

    pub fn fab(mut self, button: impl Widget + 'static) -> Self {
        self.fab = Some(Box::new(button));
        self
    }

    pub fn bottom_bar(mut self, bar: impl Widget + 'static) -> Self {
        self.bottom_bar = Some(Box::new(bar));
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

            // 2. Logic: Inject Back Button if Global Navigator says we can pop.
            // If a drawer is attached, keep leading empty so the renderer can show a menu icon.
            if Navigator::can_pop() && self.drawer.is_none() {
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
            drawer: to_vec(&self.drawer),
            body: vec![self.body.build()],
            bottom_bar: to_vec(&self.bottom_bar),
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
//      SCROLL VIEW
// ==========================

pub struct Scroll {
    pub child: Box<dyn Widget>,
    pub modifiers: Modifiers,
}

impl Scroll {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            child: Box::new(child),
            modifiers: Modifiers::default(),
        }
    }
}

impl_modifiers!(Scroll);

impl Widget for Scroll {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::ScrollView {
                views: vec![self.child.build()],
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Scroll {
                child: vec![self.child.build()],
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn scroll(child: impl Widget + 'static) -> Scroll {
    Scroll::new(child)
}

// ==========================
//      COLUMN WIDGET
// ==========================

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
