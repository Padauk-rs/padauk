#[macro_export]
macro_rules! children {
    ( $($widget:expr),* $(,)? ) => {
        vec![
            $($crate::ui::widget::IntoWidget::into_widget($widget)),* ]
    };
}

#[macro_export]
macro_rules! impl_modifiers {
    // Matches the Struct Name (e.g., Text, Button)
    ($widget_type:ident) => {
        impl $widget_type {
            // Adds a .padding() method to the struct
            pub fn padding(mut self, value: f32) -> Self {
                self.modifiers.padding = Some(value);
                self
            }

            pub fn padding_horizontal(mut self, value: f32) -> Self {
                self.modifiers.padding_horizontal = Some(value);
                self
            }

            pub fn padding_vertical(mut self, value: f32) -> Self {
                self.modifiers.padding_vertical = Some(value);
                self
            }

            // Adds a .bg() method to the struct (hex color)
            pub fn bg(mut self, color: &str) -> Self {
                self.modifiers.background_color = Some(crate::ui::color::color_hex(color));
                self
            }

            pub fn bg_color(mut self, color: crate::ui::color::ColorValue) -> Self {
                self.modifiers.background_color = Some(color);
                self
            }

            pub fn size(mut self, width: Option<f32>, height: Option<f32>) -> Self {
                self.modifiers.width = width;
                self.modifiers.height = height;
                self
            }

            pub fn width(mut self, value: f32) -> Self {
                self.modifiers.width = Some(value);
                self
            }

            pub fn height(mut self, value: f32) -> Self {
                self.modifiers.height = Some(value);
                self
            }

            pub fn fill_max_width(mut self) -> Self {
                self.modifiers.fill_max_width = true;
                self
            }

            pub fn fill_max_height(mut self) -> Self {
                self.modifiers.fill_max_height = true;
                self
            }

            pub fn weight(mut self, value: f32) -> Self {
                self.modifiers.weight = Some(value);
                self
            }

            pub fn weight_fill(mut self, value: bool) -> Self {
                self.modifiers.weight_fill = Some(value);
                self
            }

            pub fn border(mut self, width: f32, color: crate::ui::color::ColorValue) -> Self {
                self.modifiers.border_width = Some(width);
                self.modifiers.border_color = Some(color);
                self
            }

            pub fn border_hex(mut self, width: f32, color: &str) -> Self {
                self.modifiers.border_width = Some(width);
                self.modifiers.border_color = Some(crate::ui::color::color_hex(color));
                self
            }

            pub fn alpha(mut self, value: f32) -> Self {
                self.modifiers.alpha = Some(value);
                self
            }

            pub fn clip(mut self, value: bool) -> Self {
                self.modifiers.clip = value;
                self
            }

            pub fn corner_radius(mut self, value: f32) -> Self {
                self.modifiers.corner_radius = Some(value);
                self
            }

            pub fn offset(mut self, x: f32, y: f32) -> Self {
                self.modifiers.offset_x = Some(x);
                self.modifiers.offset_y = Some(y);
                self
            }

            pub fn z_index(mut self, value: f32) -> Self {
                self.modifiers.z_index = Some(value);
                self
            }

            pub fn modifier_enabled(mut self, value: bool) -> Self {
                self.modifiers.enabled = Some(value);
                self
            }
        }
    };
}
