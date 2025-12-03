pub mod builder;
pub mod state;

pub use builder::CanvasBuilder;
pub use state::CanvasState;

use crate::backend::{RenderBackend, TinySkiaBackend};
use crate::color::Color;
use crate::error::{Result, CloveError};
use crate::image::formats::ImageFormat;
use crate::text::font_manager::FontManager;
use crate::layer::{Layer, LayerManager};

/// Main canvas struct
pub struct Canvas {
    backend: TinySkiaBackend,
    state_stack: Vec<CanvasState>,
    font_manager: Option<FontManager>,
    layer_manager: LayerManager,
    width: u32,
    height: u32,
}

impl Canvas {
    /// Create a new canvas using builder
    pub fn builder() -> CanvasBuilder {
        CanvasBuilder::new()
    }
    
    /// Create canvas with default settings
    pub fn new(width: u32, height: u32) -> Result<Self> {
        Self::builder()
            .size(width, height)
            .build()
    }
    
    /// Get canvas width
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get canvas height
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Clear canvas with color
    pub fn clear(&mut self, color: Color) -> &mut Self {
        self.backend.clear(color);
        self
    }
    
    /// Save current state
    pub fn save_state(&mut self) -> &mut Self {
        let state = CanvasState::from_canvas(self);
        self.state_stack.push(state);
        self
    }
    
    /// Restore previous state
    pub fn restore_state(&mut self) -> Result<&mut Self> {
        let state = self.state_stack.pop()
            .ok_or(CloveError::NoSavedState)?;
        state.apply_to_canvas(self);
        Ok(self)
    }
    
    /// Set font manager
    pub fn set_font_manager(&mut self, font_manager: FontManager) -> &mut Self {
        self.font_manager = Some(font_manager);
        self
    }
    
    /// Get font manager reference
    pub fn font_manager(&self) -> Option<&FontManager> {
        self.font_manager.as_ref()
    }
    
    /// Get font manager mutable reference
    pub fn font_manager_mut(&mut self) -> Option<&mut FontManager> {
        self.font_manager.as_mut()
    }
    
    /// Save canvas to file
    pub fn save(&mut self, path: &str) -> Result<()> {
        // Merge all layers with backend before saving
        self.merge_layers()?;
        
        let format = ImageFormat::from_path(path)?;
        let buffer = self.to_buffer(format)?;
        std::fs::write(path, buffer)?;
        Ok(())
    }
    
    /// Save with quality (JPEG only)
    pub fn save_with_quality(&mut self, path: &str, quality: u8) -> Result<()> {
        // Merge all layers with backend before saving
        self.merge_layers()?;
        
        let buffer = self.backend.encode_jpeg(quality)?;
        std::fs::write(path, buffer)?;
        Ok(())
    }
    
    /// Export to buffer
    pub fn to_buffer(&self, format: ImageFormat) -> Result<Vec<u8>> {
        match format {
            ImageFormat::Png => self.backend.encode_png(),
            ImageFormat::Jpeg => self.backend.encode_jpeg(90),
            ImageFormat::WebP => self.backend.encode_webp(),
        }
    }
    
    /// Merge all layers with backend pixmap
    fn merge_layers(&mut self) -> Result<()> {
        use tiny_skia::{Pixmap, Transform};
        
        // Get merged layers image
        let merged_image = self.layer_manager.merge_all()?;
        
        // Convert merged RgbaImage to Pixmap
        let merged_pixmap = {
            let width = merged_image.width();
            let height = merged_image.height();
            let mut pixmap = Pixmap::new(width, height)
                .ok_or_else(|| CloveError::InvalidDimensions { width, height })?;
            
            // Copy image data to pixmap (convert from straight alpha to premultiplied)
            let img_data = merged_image.as_raw();
            let pixmap_data = pixmap.data_mut();
            
            for (i, chunk) in img_data.chunks_exact(4).enumerate() {
                let r = chunk[0];
                let g = chunk[1];
                let b = chunk[2];
                let a = chunk[3];
                
                let idx = i * 4;
                if idx + 3 < pixmap_data.len() {
                    // Premultiply alpha
                    let alpha_f = a as f32 / 255.0;
                    pixmap_data[idx] = (r as f32 * alpha_f) as u8;
                    pixmap_data[idx + 1] = (g as f32 * alpha_f) as u8;
                    pixmap_data[idx + 2] = (b as f32 * alpha_f) as u8;
                    pixmap_data[idx + 3] = a;
                }
            }
            
            pixmap
        };
        
        // Get backend pixmap
        let backend_pixmap = self.backend.get_pixmap_mut();
        
        // Use draw_pixmap to composite
        let pixmap_paint = tiny_skia::PixmapPaint {
            quality: tiny_skia::FilterQuality::Nearest,
            opacity: 1.0,
            blend_mode: tiny_skia::BlendMode::SourceOver,
        };
        
        backend_pixmap.draw_pixmap(
            0, 0,
            merged_pixmap.as_ref(),
            &pixmap_paint,
            Transform::identity(),
            None,
        );
        
        Ok(())
    }
    
    /// Get backend reference (for advanced operations)
    pub fn backend(&self) -> &TinySkiaBackend {
        &self.backend
    }
    
    /// Get backend mutable reference
    pub fn backend_mut(&mut self) -> &mut TinySkiaBackend {
        &mut self.backend
    }
}

impl Canvas {
    /// Create a layer with default dimensions (matches canvas size)
    pub fn create_layer(&mut self, name: &str) -> Result<&mut Layer> {
        let font_manager_arc = self.font_manager.as_ref().map(|fm| {
            std::sync::Arc::new(std::sync::Mutex::new(fm.clone()))
        });
        let id = self.layer_manager.create(name, font_manager_arc);
        let layer = self.layer_manager.get(id)
            .ok_or_else(|| CloveError::LayerNotFound(name.to_string()))?;
        Ok(layer)
    }
    
    /// Create a layer with custom dimensions
    pub fn create_layer_with_size(&mut self, name: &str, width: u32, height: u32) -> Result<&mut Layer> {
        let font_manager_arc = self.font_manager.as_ref().map(|fm| {
            std::sync::Arc::new(std::sync::Mutex::new(fm.clone()))
        });
        let id = self.layer_manager.create_with_size(name, width, height, font_manager_arc);
        let layer = self.layer_manager.get(id)
            .ok_or_else(|| CloveError::LayerNotFound(name.to_string()))?;
        Ok(layer)
    }
    
    pub(crate) fn new_internal(
        backend: TinySkiaBackend,
        font_manager: Option<FontManager>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            backend,
            state_stack: Vec::new(),
            font_manager,
            layer_manager: LayerManager::new(width, height),
            width,
            height,
        }
    }
}

