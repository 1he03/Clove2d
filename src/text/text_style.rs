use crate::color::Color;

/// Font weight enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
    // Future: 100-900 numeric weights
}

/// Font style enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
}

/// Text alignment - Left, Right, Center only (no Justify)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

/// Text width enum - None, Max(f32), FullPage, Layer (no Auto, Fixed)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWidth {
    None,
    Max(f32),
    FullPage,
    Layer,
}

/// Text style configuration
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub color: Color,
    pub letter_spacing: f32,
    pub line_height: f32,
    pub align: TextAlign,
    pub width: TextWidth,
}

impl TextStyle {
    pub fn new(font_family: &str, font_size: f32) -> Self {
        Self {
            font_family: font_family.to_string(),
            font_size,
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            color: Color::default(),
            letter_spacing: 0.0,
            line_height: 1.0,
            align: TextAlign::Left,
            width: TextWidth::None,
        }
    }
    
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }
    
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }
    
    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }
    
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    
    pub fn letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }
    
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }
    
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
    
    pub fn width(mut self, width: TextWidth) -> Self {
        self.width = width;
        self
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new("Arial", 16.0)
    }
}

