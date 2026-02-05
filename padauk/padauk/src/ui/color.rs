#[derive(uniffi::Enum, Clone, Debug)]
pub enum ColorValue {
    Rgb { r: u8, g: u8, b: u8, a: u8 },
    Hex { value: String },
}

pub fn color_rgb(r: u8, g: u8, b: u8) -> ColorValue {
    ColorValue::Rgb { r, g, b, a: 255 }
}

pub fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorValue {
    ColorValue::Rgb { r, g, b, a }
}

pub fn color_hex(value: &str) -> ColorValue {
    ColorValue::Hex {
        value: value.to_string(),
    }
}
