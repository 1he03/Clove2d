use crate::backend::TinySkiaBackend;
use crate::color::Color;
use crate::error::{Result, CloveError};
use crate::text::font_manager::FontManager;
use crate::utils::validate_dimensions;
use crate::canvas::Canvas;

/// Canvas builder for fluent API
pub struct CanvasBuilder {
    width: Option<u32>,
    height: Option<u32>,
    background: Option<Color>,
    font_manager: Option<FontManager>,
}

impl CanvasBuilder {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            background: None,
            font_manager: None,
        }
    }
    
    /// Set canvas size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    
    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
    
    /// Set font manager
    pub fn font_manager(mut self, font_manager: FontManager) -> Self {
        self.font_manager = Some(font_manager);
        self
    }
    
    /// Build the canvas
    pub fn build(self) -> Result<Canvas> {
        let width = self.width.ok_or_else(|| {
            CloveError::InvalidState("Canvas width not set".to_string())
        })?;
        let height = self.height.ok_or_else(|| {
            CloveError::InvalidState("Canvas height not set".to_string())
        })?;
        
        validate_dimensions(width, height)?;
        
        let backend = TinySkiaBackend::new(width, height)?;
        let mut canvas = Canvas::new_internal(backend, self.font_manager, width, height);
        
        if let Some(bg) = self.background {
            canvas.clear(bg);
        }
        
        Ok(canvas)
    }
}

impl Default for CanvasBuilder {
    fn default() -> Self {
        Self::new()
    }
}

