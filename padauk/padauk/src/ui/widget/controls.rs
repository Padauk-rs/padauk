use crate::{
    impl_modifiers,
    ui::{
        app_bar::{AppBarStyle, AppBarStyleOptions},
        button::{
            ButtonShape, ButtonStyle, ButtonStyleOptions, FabOptions, FabStyle, IconButtonOptions,
            IconButtonStyle, IconType,
        },
        card::{CardShape, CardStyle, CardStyleOptions},
        chip::{ChipStyle, ChipStyleOptions},
        color::ColorValue,
        form::{self, FieldValidator, FormKey},
        modifier::Modifiers,
        text_field::{TextFieldOptions, TextFieldStyle},
        widget::{UiNode, Widget},
    },
};
use log::debug;
use std::sync::Arc;
use uuid::Uuid;

// ==========================
//      APP BAR WIDGET
// ==========================

pub struct AppBar {
    pub title: String,
    pub leading: Vec<Box<dyn Widget>>,
    pub style: AppBarStyle,
    pub options: AppBarStyleOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(AppBar);

impl Widget for AppBar {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            // iOS uses Label; AppBar not supported yet.
            UiNode::Label {
                title: self.title.clone(),
                pt_size: 16.0,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::AppBar {
                title: self.title.clone(),
                leading: self.leading.iter().map(|child| child.build()).collect(),
                style: self.style,
                options: self.options.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl AppBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            leading: Vec::new(),
            style: AppBarStyle::Small,
            options: AppBarStyleOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: AppBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn options(mut self, options: AppBarStyleOptions) -> Self {
        self.options = options;
        self
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

// ==========================
//      TEXT WIDGET
// ==========================

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
                pt_size: self.font_size,
                attributes: self.modifiers.clone(),
            }
        }

        // --- ANDROID BUILD LOGIC ---
        #[cfg(not(target_os = "ios"))]
        {
            UiNode::Text {
                text: self.content.clone(),
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
            font_size: 16.0,
            modifiers: Modifiers::default(),
        }
    }
}

pub fn text(content: &str) -> Text {
    Text::new(content)
}

// ==========================
//      TEXT FIELD WIDGET
// ==========================

pub struct TextField {
    pub field_id: String,
    pub label: String,
    pub value: String,
    pub style: TextFieldStyle,
    pub on_change_action_id: String,
    pub on_change: Arc<dyn Fn(String) + Send + Sync>,
    pub options: TextFieldOptions,
    pub form_id: Option<String>,
    pub validator: Option<FieldValidator>,
    pub autovalidate_on_user_interaction: bool,
    pub modifiers: Modifiers,
}

impl_modifiers!(TextField);

impl Widget for TextField {
    fn build(&self) -> UiNode {
        let mut error_text = None;

        let on_change_action_id = self.on_change_action_id.clone();
        let on_change = self.on_change.clone();
        let form_id = self.form_id.clone();
        let field_id = self.field_id.clone();
        let validator = self.validator.clone();
        let autovalidate_on_user_interaction = self.autovalidate_on_user_interaction;
        crate::ui::event_registry::register_action_with_string(
            on_change_action_id,
            move |payload| {
                on_change(payload.clone());
                if let Some(fid) = form_id.as_deref() {
                    form::sync_field(fid, &field_id, &payload, validator.clone());
                    if autovalidate_on_user_interaction {
                        form::validate_field(fid, &field_id);
                    }
                }
            },
        );

        if let Some(form_id) = &self.form_id {
            form::sync_field(form_id, &self.field_id, &self.value, self.validator.clone());
            error_text = form::field_error(form_id, &self.field_id);
        }

        #[cfg(target_os = "ios")]
        {
            let mut line = format!("{}: {}", self.label, self.value);
            if let Some(error) = error_text {
                line = format!("{line} ({error})");
            }

            UiNode::Label {
                title: line,
                pt_size: 16.0,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::TextField {
                label: self.label.clone(),
                value: self.value.clone(),
                style: self.style,
                on_change_action_id: self.on_change_action_id.clone(),
                options: self.options.clone(),
                error_text,
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl TextField {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        style: TextFieldStyle,
        on_change: impl Fn(String) + Send + Sync + 'static,
    ) -> Self {
        let label = label.into();
        let on_change_action_id = Uuid::new_v4().to_string();
        let on_change = Arc::new(on_change);
        crate::ui::event_registry::register_action_with_string(on_change_action_id.clone(), {
            let on_change = on_change.clone();
            move |payload| on_change(payload)
        });

        Self {
            field_id: label.clone(),
            label,
            value: value.into(),
            style,
            on_change_action_id,
            on_change,
            options: TextFieldOptions::default(),
            form_id: None,
            validator: None,
            autovalidate_on_user_interaction: false,
            modifiers: Modifiers::default(),
        }
    }

    pub fn options(mut self, options: TextFieldOptions) -> Self {
        self.options = options;
        self
    }

    pub fn field_key(mut self, key: impl Into<String>) -> Self {
        self.field_id = key.into();
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.options.placeholder = Some(text.into());
        self
    }

    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.options.supporting_text = Some(text.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.options.enabled = enabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.options.read_only = read_only;
        self
    }

    pub fn single_line(mut self, single_line: bool) -> Self {
        self.options.single_line = single_line;
        self
    }

    pub fn max_lines(mut self, max_lines: i32) -> Self {
        self.options.max_lines = max_lines.max(1);
        self
    }

    pub fn password(mut self, is_password: bool) -> Self {
        self.options.is_password = is_password;
        self
    }

    pub fn leading_icon(mut self, icon: IconType) -> Self {
        self.options.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconType) -> Self {
        self.options.trailing_icon = Some(icon);
        self
    }

    pub fn form(mut self, key: &FormKey) -> Self {
        self.form_id = Some(key.id().to_string());
        self
    }

    pub fn validator(
        mut self,
        key: &FormKey,
        validate: impl Fn(&str) -> Option<String> + Send + Sync + 'static,
    ) -> Self {
        self.form_id = Some(key.id().to_string());
        self.validator = Some(Arc::new(validate));
        self
    }

    pub fn required(self, key: &FormKey, message: impl Into<String>) -> Self {
        let message = message.into();
        self.validator(key, move |value| {
            if value.trim().is_empty() {
                Some(message.clone())
            } else {
                None
            }
        })
    }

    pub fn autovalidate_on_user_interaction(mut self, enabled: bool) -> Self {
        self.autovalidate_on_user_interaction = enabled;
        self
    }
}

pub fn filled_text_field(
    label: impl Into<String>,
    value: impl Into<String>,
    on_change: impl Fn(String) + Send + Sync + 'static,
) -> TextField {
    TextField::new(label, value, TextFieldStyle::Filled, on_change)
}

pub fn outlined_text_field(
    label: impl Into<String>,
    value: impl Into<String>,
    on_change: impl Fn(String) + Send + Sync + 'static,
) -> TextField {
    TextField::new(label, value, TextFieldStyle::Outlined, on_change)
}

// ==========================
//      BUTTON WIDGET
// ==========================

pub struct Button {
    pub label: String,
    pub action_id: String,
    pub style: ButtonStyle,
    pub options: ButtonStyleOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(Button);

impl Widget for Button {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            let child_node = UiNode::Label {
                title: self.label.clone(),
                pt_size: 16.0,
                attributes: Modifiers::default(),
            };

            UiNode::Button {
                action_id: self.action_id.clone(),
                label: vec![child_node],
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            let child_node = UiNode::Text {
                text: self.label.clone(),
                sp_size: 16.0,
                modifiers: Modifiers::default(),
            };

            UiNode::Button {
                action_id: self.action_id.clone(),
                content: vec![child_node],
                style: self.style,
                options: self.options.clone(),
                modifiers: self.modifiers.clone(),
            }
        }
    }
}

impl Button {
    pub fn new(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        debug!("Button created with action id: {}", action_id);
        crate::ui::event_registry::register_action(action_id.clone(), on_click);

        Self {
            label: label.into(),
            action_id,
            style: ButtonStyle::Filled,
            options: ButtonStyleOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn options(mut self, options: ButtonStyleOptions) -> Self {
        self.options = options;
        self
    }
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Button {
    Button::new(label, on_click)
}

pub fn filled_button(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Button {
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

pub fn text_button(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Button {
    Button::new(label, on_click).style(ButtonStyle::Text)
}

// ==========================
//      ICON BUTTON
// ==========================

pub struct IconButton {
    pub icon: IconType,
    pub style: IconButtonStyle,
    pub action_id: String,
    pub options: IconButtonOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(IconButton);

impl Widget for IconButton {
    fn build(&self) -> UiNode {
        #[cfg(target_os = "ios")]
        {
            UiNode::Label {
                title: "Icon".to_string(),
                pt_size: 16.0,
                attributes: self.modifiers.clone(),
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            UiNode::IconButton {
                action_id: self.action_id.clone(),
                icon: self.icon,
                style: self.style,
                options: self.options.clone(),
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
            options: IconButtonOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn options(mut self, options: IconButtonOptions) -> Self {
        self.options = options;
        self
    }
}

pub fn icon_button(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> IconButton {
    IconButton::new(icon, on_click).style(IconButtonStyle::Standard)
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

// ==========================
//      CARD
// ==========================

pub struct Card {
    pub children: Vec<Box<dyn Widget>>,
    pub style: CardStyle,
    pub action_id: Option<String>,
    pub options: CardStyleOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(Card);

impl Widget for Card {
    fn build(&self) -> UiNode {
        let children = self.children.iter().map(|child| child.build()).collect();

        UiNode::Card {
            children,
            style: self.style,
            action_id: self.action_id.clone(),
            options: self.options.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl Card {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            children,
            style: CardStyle::Filled,
            action_id: None,
            options: CardStyleOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    pub fn action(mut self, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);
        self.action_id = Some(action_id);
        self
    }

    pub fn options(mut self, options: CardStyleOptions) -> Self {
        self.options = options;
        self
    }
}

pub fn card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children).style(CardStyle::Filled)
}

pub fn elevated_card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children).style(CardStyle::Elevated)
}

pub fn outlined_card(children: Vec<Box<dyn Widget>>) -> Card {
    Card::new(children).style(CardStyle::Outlined)
}

// ==========================
//      CHECKBOX
// ==========================

pub struct Checkbox {
    pub checked: bool,
    pub action_id: String,
    pub enabled: bool,
    pub color_checked: Option<ColorValue>,
    pub color_unchecked: Option<ColorValue>,
    pub color_checkmark: Option<ColorValue>,
    pub modifiers: Modifiers,
}

impl_modifiers!(Checkbox);

impl Widget for Checkbox {
    fn build(&self) -> UiNode {
        UiNode::Checkbox {
            checked: self.checked,
            action_id: self.action_id.clone(),
            enabled: self.enabled,
            color_checked: self.color_checked.clone(),
            color_unchecked: self.color_unchecked.clone(),
            color_checkmark: self.color_checkmark.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl Checkbox {
    pub fn new(checked: bool, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);

        Self {
            checked,
            action_id,
            enabled: true,
            color_checked: None,
            color_unchecked: None,
            color_checkmark: None,
            modifiers: Modifiers::default(),
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn colors(
        mut self,
        checked: Option<ColorValue>,
        unchecked: Option<ColorValue>,
        checkmark: Option<ColorValue>,
    ) -> Self {
        self.color_checked = checked;
        self.color_unchecked = unchecked;
        self.color_checkmark = checkmark;
        self
    }
}

pub fn checkbox(checked: bool, on_click: impl Fn() + Send + Sync + 'static) -> Checkbox {
    Checkbox::new(checked, on_click)
}

// ==========================
//      CHIP
// ==========================

pub struct Chip {
    pub label: String,
    pub style: ChipStyle,
    pub selected: bool,
    pub action_id: String,
    pub leading_icon: Option<IconType>,
    pub trailing_icon: Option<IconType>,
    pub close_action_id: Option<String>,
    pub options: ChipStyleOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(Chip);

impl Widget for Chip {
    fn build(&self) -> UiNode {
        UiNode::Chip {
            label: self.label.clone(),
            style: self.style,
            selected: self.selected,
            action_id: self.action_id.clone(),
            leading_icon: self.leading_icon.clone(),
            trailing_icon: self.trailing_icon.clone(),
            close_action_id: self.close_action_id.clone(),
            options: self.options.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl Chip {
    pub fn new(
        label: impl Into<String>,
        style: ChipStyle,
        on_click: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);

        Self {
            label: label.into(),
            style,
            selected: false,
            action_id,
            leading_icon: None,
            trailing_icon: None,
            close_action_id: None,
            options: ChipStyleOptions::default(),
            modifiers: Modifiers::default(),
        }
    }

    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    pub fn leading_icon(mut self, icon: IconType) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconType) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn close_action(mut self, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let close_action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(close_action_id.clone(), on_click);
        self.close_action_id = Some(close_action_id);
        self
    }

    pub fn options(mut self, options: ChipStyleOptions) -> Self {
        self.options = options;
        self
    }
}

pub fn assist_chip(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Chip {
    Chip::new(label, ChipStyle::Assist, on_click)
}

pub fn filter_chip(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Chip {
    Chip::new(label, ChipStyle::Filter, on_click)
}

pub fn input_chip(label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Chip {
    Chip::new(label, ChipStyle::Input, on_click)
}

pub fn suggestion_chip(
    label: impl Into<String>,
    on_click: impl Fn() + Send + Sync + 'static,
) -> Chip {
    Chip::new(label, ChipStyle::Suggestion, on_click)
}

// ==========================
//      FAB
// ==========================

pub struct Fab {
    pub action_id: String,
    pub icon: IconType,
    pub style: FabStyle,
    pub label: Option<String>,
    pub options: FabOptions,
    pub modifiers: Modifiers,
}

impl_modifiers!(Fab);

impl Widget for Fab {
    fn build(&self) -> UiNode {
        UiNode::Fab {
            action_id: self.action_id.clone(),
            icon: self.icon,
            style: self.style,
            label: self.label.clone(),
            options: self.options.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl Fab {
    pub fn new(icon: IconType, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let action_id = Uuid::new_v4().to_string();
        crate::ui::event_registry::register_action(action_id.clone(), on_click);

        Self {
            action_id,
            icon,
            style: FabStyle::Normal,
            label: None,
            options: FabOptions::default(),
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

    pub fn options(mut self, options: FabOptions) -> Self {
        self.options = options;
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
