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
                self.modifiers.padding = value;
                self
            }

            // Adds a .bg() method to the struct
            pub fn bg(mut self, color: &str) -> Self {
                self.modifiers.background_color = Some(color.to_string());
                self
            }

            pub fn size(mut self, width: Option<f32>, height: Option<f32>) -> Self {
                // assuming you add size to Modifiers struct
                self.modifiers.width = width;
                self.modifiers.height = height;
                self
            }
        }
    };
}
