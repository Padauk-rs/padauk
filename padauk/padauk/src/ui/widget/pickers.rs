use crate::{
    impl_modifiers,
    ui::{
        modifier::Modifiers,
        widget::{UiNode, Widget},
    },
};
use uuid::Uuid;

// ==========================
//      DATE PICKERS
// ==========================

pub struct DatePickerDialog {
    pub title: Option<String>,
    pub initial_selected_millis: Option<i64>,
    pub show_mode_toggle: bool,
    pub confirm_label: String,
    pub confirm_action_id: String,
    pub dismiss_label: Option<String>,
    pub dismiss_action_id: Option<String>,
    pub dismissible: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(DatePickerDialog);

impl DatePickerDialog {
    pub fn new(
        title: Option<impl Into<String>>,
        initial_selected_millis: Option<i64>,
        confirm_label: impl Into<String>,
        on_confirm: impl Fn(i64) + Send + Sync + 'static,
    ) -> Self {
        let confirm_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action_with_string(
            confirm_action_id.clone(),
            move |payload| {
                if let Ok(value) = payload.parse::<i64>() {
                    on_confirm(value);
                }
            },
        );

        Self {
            title: title.map(|t| t.into()),
            initial_selected_millis,
            show_mode_toggle: true,
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

    pub fn show_mode_toggle(mut self, value: bool) -> Self {
        self.show_mode_toggle = value;
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }
}

impl Widget for DatePickerDialog {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::DatePickerDialog {
                title: self.title.clone(),
                initial_selected_millis: self.initial_selected_millis,
                show_mode_toggle: self.show_mode_toggle,
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
            UiNode::DatePickerDialog {
                title: self.title.clone(),
                initial_selected_millis: self.initial_selected_millis,
                show_mode_toggle: self.show_mode_toggle,
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

pub fn date_picker_dialog(
    title: Option<impl Into<String>>,
    initial_selected_millis: Option<i64>,
    confirm_label: impl Into<String>,
    on_confirm: impl Fn(i64) + Send + Sync + 'static,
) -> DatePickerDialog {
    DatePickerDialog::new(title, initial_selected_millis, confirm_label, on_confirm)
}

pub struct DateRangePickerDialog {
    pub title: Option<String>,
    pub initial_start_millis: Option<i64>,
    pub initial_end_millis: Option<i64>,
    pub show_mode_toggle: bool,
    pub confirm_label: String,
    pub confirm_action_id: String,
    pub dismiss_label: Option<String>,
    pub dismiss_action_id: Option<String>,
    pub dismissible: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(DateRangePickerDialog);

impl DateRangePickerDialog {
    pub fn new(
        title: Option<impl Into<String>>,
        initial_start_millis: Option<i64>,
        initial_end_millis: Option<i64>,
        confirm_label: impl Into<String>,
        on_confirm: impl Fn(Option<i64>, Option<i64>) + Send + Sync + 'static,
    ) -> Self {
        let confirm_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action_with_string(
            confirm_action_id.clone(),
            move |payload| {
                let mut parts = payload.split('|');
                let start = parts.next().and_then(|v| v.parse::<i64>().ok());
                let end = parts.next().and_then(|v| v.parse::<i64>().ok());
                on_confirm(start, end);
            },
        );

        Self {
            title: title.map(|t| t.into()),
            initial_start_millis,
            initial_end_millis,
            show_mode_toggle: true,
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

    pub fn show_mode_toggle(mut self, value: bool) -> Self {
        self.show_mode_toggle = value;
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }
}

impl Widget for DateRangePickerDialog {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::DateRangePickerDialog {
                title: self.title.clone(),
                initial_start_millis: self.initial_start_millis,
                initial_end_millis: self.initial_end_millis,
                show_mode_toggle: self.show_mode_toggle,
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
            UiNode::DateRangePickerDialog {
                title: self.title.clone(),
                initial_start_millis: self.initial_start_millis,
                initial_end_millis: self.initial_end_millis,
                show_mode_toggle: self.show_mode_toggle,
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

pub fn date_range_picker_dialog(
    title: Option<impl Into<String>>,
    initial_start_millis: Option<i64>,
    initial_end_millis: Option<i64>,
    confirm_label: impl Into<String>,
    on_confirm: impl Fn(Option<i64>, Option<i64>) + Send + Sync + 'static,
) -> DateRangePickerDialog {
    DateRangePickerDialog::new(
        title,
        initial_start_millis,
        initial_end_millis,
        confirm_label,
        on_confirm,
    )
}

pub struct TimePickerDialog {
    pub title: Option<String>,
    pub initial_hour: Option<i32>,
    pub initial_minute: Option<i32>,
    pub is_24_hour: bool,
    pub show_mode_toggle: bool,
    pub confirm_label: String,
    pub confirm_action_id: String,
    pub dismiss_label: Option<String>,
    pub dismiss_action_id: Option<String>,
    pub dismissible: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(TimePickerDialog);

impl TimePickerDialog {
    pub fn new(
        title: Option<impl Into<String>>,
        initial_hour: Option<i32>,
        initial_minute: Option<i32>,
        confirm_label: impl Into<String>,
        on_confirm: impl Fn(i32, i32) + Send + Sync + 'static,
    ) -> Self {
        let confirm_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action_with_string(
            confirm_action_id.clone(),
            move |payload| {
                let mut parts = payload.split('|');
                let hour = parts.next().and_then(|v| v.parse::<i32>().ok());
                let minute = parts.next().and_then(|v| v.parse::<i32>().ok());
                if let (Some(hour), Some(minute)) = (hour, minute) {
                    on_confirm(hour, minute);
                }
            },
        );

        Self {
            title: title.map(|t| t.into()),
            initial_hour,
            initial_minute,
            is_24_hour: true,
            show_mode_toggle: true,
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

    pub fn use_24_hour(mut self, value: bool) -> Self {
        self.is_24_hour = value;
        self
    }

    pub fn show_mode_toggle(mut self, value: bool) -> Self {
        self.show_mode_toggle = value;
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }
}

impl Widget for TimePickerDialog {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::TimePickerDialog {
                title: self.title.clone(),
                initial_hour: self.initial_hour,
                initial_minute: self.initial_minute,
                is_24_hour: self.is_24_hour,
                show_mode_toggle: self.show_mode_toggle,
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
            UiNode::TimePickerDialog {
                title: self.title.clone(),
                initial_hour: self.initial_hour,
                initial_minute: self.initial_minute,
                is_24_hour: self.is_24_hour,
                show_mode_toggle: self.show_mode_toggle,
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

pub fn time_picker_dialog(
    title: Option<impl Into<String>>,
    initial_hour: Option<i32>,
    initial_minute: Option<i32>,
    confirm_label: impl Into<String>,
    on_confirm: impl Fn(i32, i32) + Send + Sync + 'static,
) -> TimePickerDialog {
    TimePickerDialog::new(
        title,
        initial_hour,
        initial_minute,
        confirm_label,
        on_confirm,
    )
}
