use crate::{
    impl_modifiers,
    ui::{
        modifier::Modifiers,
        widget::{UiNode, Widget},
    },
};
use uuid::Uuid;

// ==========================
//      DIALOG
// ==========================

pub struct Dialog {
    pub title: Option<String>,
    pub text: String,
    pub confirm_label: String,
    pub confirm_action_id: String,
    pub dismiss_label: Option<String>,
    pub dismiss_action_id: Option<String>,
    pub dismissible: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(Dialog);

impl Dialog {
    pub fn new(
        title: Option<impl Into<String>>,
        text: impl Into<String>,
        confirm_label: impl Into<String>,
        on_confirm: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let confirm_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(confirm_action_id.clone(), on_confirm);

        Self {
            title: title.map(|t| t.into()),
            text: text.into(),
            confirm_label: confirm_label.into(),
            confirm_action_id,
            dismiss_label: None,
            dismiss_action_id: None,
            dismissible: true,
            modifiers: Modifiers::default(),
        }
    }

    pub fn dismiss(
        mut self,
        label: impl Into<String>,
        on_dismiss: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let dismiss_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(dismiss_action_id.clone(), on_dismiss);
        self.dismiss_label = Some(label.into());
        self.dismiss_action_id = Some(dismiss_action_id);
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }
}

impl Widget for Dialog {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::Dialog {
                title: self.title.clone(),
                text: self.text.clone(),
                confirm_label: self.confirm_label.clone(),
                confirm_action_id: self.confirm_action_id.clone(),
                dismiss_label: self.dismiss_label.clone(),
                dismiss_action_id: self.dismiss_action_id.clone(),
                dismissible: self.dismissible,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Dialog {
                title: self.title.clone(),
                text: self.text.clone(),
                confirm_label: self.confirm_label.clone(),
                confirm_action_id: self.confirm_action_id.clone(),
                dismiss_label: self.dismiss_label.clone(),
                dismiss_action_id: self.dismiss_action_id.clone(),
                dismissible: self.dismissible,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn dialog(
    title: Option<impl Into<String>>,
    text: impl Into<String>,
    confirm_label: impl Into<String>,
    on_confirm: impl Fn() + Send + Sync + 'static,
) -> Dialog {
    Dialog::new(title, text, confirm_label, on_confirm)
}

// ==========================
//   FULLSCREEN DIALOG
// ==========================

pub struct FullscreenDialog {
    pub title: String,
    pub content: Box<dyn Widget>,
    pub confirm_label: Option<String>,
    pub confirm_action_id: Option<String>,
    pub dismiss_label: String,
    pub dismiss_action_id: String,
    pub dismissible: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(FullscreenDialog);

impl FullscreenDialog {
    pub fn new(
        title: impl Into<String>,
        content: impl Widget + 'static,
        dismiss_label: impl Into<String>,
        on_dismiss: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let dismiss_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(dismiss_action_id.clone(), on_dismiss);

        Self {
            title: title.into(),
            content: Box::new(content),
            confirm_label: None,
            confirm_action_id: None,
            dismiss_label: dismiss_label.into(),
            dismiss_action_id,
            dismissible: true,
            modifiers: Modifiers::default(),
        }
    }

    pub fn confirm(
        mut self,
        label: impl Into<String>,
        on_confirm: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let confirm_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(confirm_action_id.clone(), on_confirm);
        self.confirm_label = Some(label.into());
        self.confirm_action_id = Some(confirm_action_id);
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }
}

impl Widget for FullscreenDialog {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::FullscreenDialog {
                title: self.title.clone(),
                content: vec![self.content.build()],
                confirm_label: self.confirm_label.clone(),
                confirm_action_id: self.confirm_action_id.clone(),
                dismiss_label: self.dismiss_label.clone(),
                dismiss_action_id: self.dismiss_action_id.clone(),
                dismissible: self.dismissible,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::FullscreenDialog {
                title: self.title.clone(),
                content: vec![self.content.build()],
                confirm_label: self.confirm_label.clone(),
                confirm_action_id: self.confirm_action_id.clone(),
                dismiss_label: self.dismiss_label.clone(),
                dismiss_action_id: self.dismiss_action_id.clone(),
                dismissible: self.dismissible,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

pub fn dialog_fullscreen(
    title: impl Into<String>,
    content: impl Widget + 'static,
    dismiss_label: impl Into<String>,
    on_dismiss: impl Fn() + Send + Sync + 'static,
) -> FullscreenDialog {
    FullscreenDialog::new(title, content, dismiss_label, on_dismiss)
}
