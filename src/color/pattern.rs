use crate::color::Rgba;

/// Pattern fill
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub color: Rgba,
    // Future: image patterns, repeating patterns, etc.
}

impl Pattern {
    pub fn new(color: Rgba) -> Self {
        Self { color }
    }
}

