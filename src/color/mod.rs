pub mod rgba;
pub mod hsla;
pub mod named;
pub mod gradient;
pub mod pattern;
pub mod conversions;

pub use rgba::Rgba;
pub use hsla::Hsla;
pub use named::NamedColor;
pub use gradient::{LinearGradient, RadialGradient, LinearGradientBuilder, RadialGradientBuilder, GradientStop};
pub use pattern::Pattern;

use crate::error::Result;

/// Color enum supporting multiple color formats
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgba(Rgba),
    Hsla(Hsla),
    Named(NamedColor),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
}

impl Color {
    /// Create color from hex string
    pub fn hex(hex: &str) -> Result<Self> {
        Rgba::from_hex(hex).map(Color::Rgba)
    }
    
    /// Create color from named color enum
    pub fn named(name: NamedColor) -> Self {
        Color::Named(name)
    }
    
    /// Create color from RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::Rgba(Rgba::new(r, g, b, a))
    }
    
    /// Create linear gradient builder
    pub fn linear_gradient() -> LinearGradientBuilder {
        LinearGradientBuilder::new()
    }
    
    /// Create radial gradient builder
    pub fn radial_gradient() -> RadialGradientBuilder {
        RadialGradientBuilder::new()
    }
    
    /// Convert to RGBA
    pub fn to_rgba(&self) -> Rgba {
        match self {
            Color::Rgba(rgba) => *rgba,
            Color::Hsla(hsla) => hsla.to_rgba(),
            Color::Named(named) => named.to_rgba(),
            Color::LinearGradient(grad) => grad.stops.first()
                .map(|stop| stop.color.to_rgba())
                .unwrap_or(Rgba::TRANSPARENT),
            Color::RadialGradient(grad) => grad.stops.first()
                .map(|stop| stop.color.to_rgba())
                .unwrap_or(Rgba::TRANSPARENT),
            Color::Pattern(pattern) => pattern.color,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Rgba(Rgba::TRANSPARENT)
    }
}

