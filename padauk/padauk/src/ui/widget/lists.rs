use crate::{
    impl_modifiers,
    ui::{
        button::IconType,
        divider::DividerOptions,
        list::{ListItemOptions, ListItemTrailing},
        modifier::Modifiers,
        widget::{UiNode, Widget},
    },
};
use uuid::Uuid;

pub struct List {
    pub items: Vec<Box<dyn Widget>>,
    pub modifiers: Modifiers,
}

impl_modifiers!(List);

impl List {
    pub fn new(items: Vec<Box<dyn Widget>>) -> Self {
        Self {
            items,
            modifiers: Modifiers::default(),
        }
    }
}

impl Widget for List {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::VStack {
                views: self.items.iter().map(|item| item.build()).collect(),
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::ListView {
                items: self.items.iter().map(|item| item.build()).collect(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn list(items: Vec<Box<dyn Widget>>) -> List {
    List::new(items)
}

pub struct ListItem {
    pub headline: String,
    pub supporting_text: Option<String>,
    pub overline_text: Option<String>,
    pub leading_icon: Option<IconType>,
    pub trailing: ListItemTrailing,
    pub action_id: Option<String>,
    pub options: ListItemOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(ListItem);

impl ListItem {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            supporting_text: None,
            overline_text: None,
            leading_icon: None,
            trailing: ListItemTrailing::default(),
            action_id: None,
            options: ListItemOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn supporting_text(mut self, value: impl Into<String>) -> Self {
        self.supporting_text = Some(value.into());
        self
    }

    pub fn overline_text(mut self, value: impl Into<String>) -> Self {
        self.overline_text = Some(value.into());
        self
    }

    pub fn leading_icon(mut self, icon: IconType) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_text(mut self, value: impl Into<String>) -> Self {
        self.trailing.text = Some(value.into());
        self
    }

    pub fn trailing_icon(mut self, icon: IconType) -> Self {
        self.trailing.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), handler);
        self.action_id = Some(action_id);
        self
    }

    pub fn options(mut self, options: ListItemOptions) -> Self {
        self.options = options;
        self
    }
}

impl Widget for ListItem {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::Label {
                title: self.headline.clone(),
                pt_size: 16.0,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::ListItem {
                headline: self.headline.clone(),
                supporting_text: self.supporting_text.clone(),
                overline_text: self.overline_text.clone(),
                leading_icon: self.leading_icon,
                trailing: self.trailing.clone(),
                action_id: self.action_id.clone(),
                options: self.options.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn list_item(headline: impl Into<String>) -> ListItem {
    ListItem::new(headline)
}

pub struct Divider {
    pub options: DividerOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(Divider);

impl Divider {
    pub fn new() -> Self {
        Self {
            options: DividerOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn options(mut self, options: DividerOptions) -> Self {
        self.options = options;
        self
    }

    pub fn color(mut self, color: crate::ui::color::ColorValue) -> Self {
        self.options.color = Some(color);
        self
    }

    pub fn thickness(mut self, value: f32) -> Self {
        self.options.thickness = Some(value);
        self
    }

    pub fn inset_start(mut self, value: f32) -> Self {
        self.options.inset_start = Some(value);
        self
    }

    pub fn inset_end(mut self, value: f32) -> Self {
        self.options.inset_end = Some(value);
        self
    }

    pub fn vertical(mut self, value: bool) -> Self {
        self.options.vertical = value;
        self
    }
}

impl Widget for Divider {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::VStack {
                views: vec![],
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Divider {
                options: self.options.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn divider() -> Divider {
    Divider::new()
}

pub fn vertical_divider() -> Divider {
    Divider::new().vertical(true)
}
