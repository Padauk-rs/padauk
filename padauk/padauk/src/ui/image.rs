use crate::UiNode;
use crate::{Widget, impl_modifiers, ui::modifier::Modifiers};

#[derive(uniffi::Enum, Clone)]
pub enum BoxFit {
    Contain,
    Cover,
    Fill,
    FitWidth,
    FitHeight,
    None,
    ScaleDown,
}

#[derive(uniffi::Enum, Clone)]
pub enum ImageSource {
    Asset { name: String },
    Network { url: String },
    File { path: String },
    Memory { data: Vec<u8> },
}

pub struct Image {
    pub source: ImageSource,
    pub fit: BoxFit,
    pub modifiers: Modifiers,
}

impl Image {
    // --- Constructors ---

    pub fn asset(name: impl Into<String>) -> Self {
        Self {
            source: ImageSource::Asset { name: name.into() },
            fit: BoxFit::Contain, // Default
            modifiers: Modifiers::default(),
        }
    }

    pub fn network(url: impl Into<String>) -> Self {
        Self {
            source: ImageSource::Network { url: url.into() },
            fit: BoxFit::Contain,
            modifiers: Modifiers::default(),
        }
    }

    pub fn file(path: impl Into<String>) -> Self {
        Self {
            source: ImageSource::File { path: path.into() },
            fit: BoxFit::Contain,
            modifiers: Modifiers::default(),
        }
    }

    pub fn memory(data: Vec<u8>) -> Self {
        Self {
            source: ImageSource::Memory { data },
            fit: BoxFit::Contain,
            modifiers: Modifiers::default(),
        }
    }

    // --- Chainable Setters ---

    pub fn fit(mut self, fit: BoxFit) -> Self {
        self.fit = fit;
        self
    }
}

impl_modifiers!(Image);

impl Widget for Image {
    fn build(&self) -> UiNode {
        UiNode::Image {
            source: self.source.clone(),
            fit: self.fit.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

// DSL Helpers (Optional, or rely on Image::*)
pub fn image_asset(name: impl Into<String>) -> Image {
    Image::asset(name)
}

pub fn image_network(url: impl Into<String>) -> Image {
    Image::network(url)
}

pub fn image_memory(data: Vec<u8>) -> Image {
    Image::memory(data)
}

pub fn image_file(path: impl Into<String>) -> Image {
    Image::file(path)
}
