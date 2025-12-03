pub mod blend_modes;

pub use blend_modes::BlendMode;

use crate::error::Result;
use image::RgbaImage;
use tiny_skia::Pixmap;

/// Layer ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId(usize);

impl LayerId {
    pub fn new() -> Self {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        LayerId(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

/// Layer struct
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub content: RgbaImage,
    pub x: f32,
    pub y: f32,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub visible: bool,
    base_width: u32,
    base_height: u32,
    font_manager: Option<std::sync::Arc<std::sync::Mutex<crate::text::font_manager::FontManager>>>,
}

impl Layer {
    pub fn new(id: LayerId, name: &str, base_width: u32, base_height: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            content: RgbaImage::new(base_width, base_height),
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            visible: true,
            base_width,
            base_height,
            font_manager: None,
        }
    }
    
    /// Set font manager for text rendering
    pub(crate) fn set_font_manager(&mut self, font_manager: std::sync::Arc<std::sync::Mutex<crate::text::font_manager::FontManager>>) {
        self.font_manager = Some(font_manager);
    }
    
    /// Get font manager reference
    pub(crate) fn font_manager(&self) -> Option<std::sync::MutexGuard<'_, crate::text::font_manager::FontManager>> {
        self.font_manager.as_ref().and_then(|fm| fm.lock().ok())
    }
    
    /// Set layer width
    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = Some(width);
        self.scale_content();
        self
    }
    
    /// Set layer height
    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = Some(height);
        self.scale_content();
        self
    }
    
    /// Set layer dimensions
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self.scale_content();
        self
    }
    
    /// Get effective dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (
            self.width.unwrap_or(self.base_width),
            self.height.unwrap_or(self.base_height),
        )
    }
    
    fn scale_content(&mut self) {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            if w != self.content.width() || h != self.content.height() {
                use image::imageops::resize;
                self.content = resize(
                    &self.content,
                    w,
                    h,
                    image::imageops::FilterType::Lanczos3,
                );
            }
        }
    }
    
    /// Draw rectangle
    pub fn draw_rect(&mut self) -> crate::shapes::RectBuilder {
        crate::shapes::RectBuilder::new(self, 0.0, 0.0, 100.0, 100.0)
    }
    
    /// Draw circle
    pub fn draw_circle(&mut self) -> crate::shapes::CircleBuilder {
        crate::shapes::CircleBuilder::new(self, 0.0, 0.0, 50.0)
    }
    
    /// Draw text
    pub fn draw_text(&mut self, text: &str) -> crate::text::TextBuilder {
        use crate::text::TextStyle;
        use crate::geometry::Point;
        let style = TextStyle::default();
        crate::text::TextBuilder::new(self, text, style, Point::ZERO)
    }
    
    /// Draw image
    pub fn draw_image(&mut self, image: crate::image::Image) -> ImageBuilder {
        ImageBuilder::new(self, image)
    }
    
    /// Apply filter to layer
    pub fn apply_filter(&mut self, filter: crate::filter::Filter) -> Result<&mut Self> {
        // Apply filter to layer content
        let filtered_content = filter.apply(&self.content)?;
        self.content = filtered_content;
        Ok(self)
    }
    
    /// Set blend mode
    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = mode;
        self
    }
    
    /// Set layer opacity
    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    
    /// Get pixmap for drawing (converts RgbaImage to Pixmap)
    pub(crate) fn get_pixmap_mut(&mut self) -> Result<Pixmap> {
        let (width, height) = self.dimensions();
        let mut pixmap = Pixmap::new(width, height)
            .ok_or_else(|| crate::error::CloveError::InvalidDimensions { width, height })?;
        
        // Copy RgbaImage data to Pixmap
        let img_data = self.content.as_raw();
        let pixmap_data = pixmap.data_mut();
        
        // tiny-skia uses premultiplied alpha, image crate uses straight alpha
        // We need to convert
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
        
        Ok(pixmap)
    }
    
    /// Update layer content from pixmap (converts Pixmap back to RgbaImage)
    pub(crate) fn update_from_pixmap(&mut self, pixmap: &Pixmap) -> Result<()> {
        let width = pixmap.width();
        let height = pixmap.height();
        
        // Create new RgbaImage
        let mut img = RgbaImage::new(width, height);
        let img_data = img.as_mut();
        let pixmap_data = pixmap.data();
        
        // Convert from premultiplied alpha to straight alpha
        for (i, chunk) in pixmap_data.chunks_exact(4).enumerate() {
            let r_pre = chunk[0];
            let g_pre = chunk[1];
            let b_pre = chunk[2];
            let a = chunk[3];
            
            let idx = i * 4;
            if idx + 3 < img_data.len() && a > 0 {
                // Unpremultiply alpha
                let alpha_f = a as f32 / 255.0;
                img_data[idx] = (r_pre as f32 / alpha_f) as u8;
                img_data[idx + 1] = (g_pre as f32 / alpha_f) as u8;
                img_data[idx + 2] = (b_pre as f32 / alpha_f) as u8;
                img_data[idx + 3] = a;
            } else if idx + 3 < img_data.len() {
                img_data[idx] = r_pre;
                img_data[idx + 1] = g_pre;
                img_data[idx + 2] = b_pre;
                img_data[idx + 3] = a;
            }
        }
        
        self.content = img;
        Ok(())
    }
}

/// Image builder for drawing images on layers
pub struct ImageBuilder<'a> {
    layer: &'a mut Layer,
    image: crate::image::Image,
    x: f32,
    y: f32,
    opacity: f32,
}

impl<'a> ImageBuilder<'a> {
    pub fn new(layer: &'a mut Layer, image: crate::image::Image) -> Self {
        Self {
            layer,
            image,
            x: 0.0,
            y: 0.0,
            opacity: 1.0,
        }
    }
    
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        use crate::error::CloveError;
        
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Create pixmap from image data
        let image_pixmap = tiny_skia::Pixmap::from_vec(
            self.image.data.to_vec(),
            tiny_skia::IntSize::from_wh(
                self.image.data.width(),
                self.image.data.height(),
            ).ok_or_else(|| CloveError::InvalidState("Invalid image size".to_string()))?,
        ).ok_or_else(|| CloveError::InvalidState("Failed to create pixmap from image".to_string()))?;
        
        // Create paint with opacity
        let mut paint = tiny_skia::PixmapPaint::default();
        paint.opacity = self.opacity;
        
        // Draw image onto layer pixmap
        pixmap.draw_pixmap(
            self.x as i32,
            self.y as i32,
            image_pixmap.as_ref(),
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );
        
        // Update layer content from pixmap
        self.layer.update_from_pixmap(&pixmap)?;
        
        Ok(self.layer)
    }
}

/// Layer manager
pub struct LayerManager {
    layers: Vec<Layer>,
    active: Option<LayerId>,
    base_width: u32,
    base_height: u32,
}

impl LayerManager {
    pub fn new(base_width: u32, base_height: u32) -> Self {
        Self {
            layers: Vec::new(),
            active: None,
            base_width,
            base_height,
        }
    }
    
    /// Create layer with canvas dimensions
    pub fn create(&mut self, name: &str, font_manager: Option<std::sync::Arc<std::sync::Mutex<crate::text::font_manager::FontManager>>>) -> LayerId {
        let id = LayerId::new();
        let mut layer = Layer::new(id, name, self.base_width, self.base_height);
        if let Some(fm) = font_manager {
            layer.font_manager = Some(fm);
        }
        self.layers.push(layer);
        id
    }
    
    /// Create layer with custom dimensions
    pub fn create_with_size(&mut self, name: &str, width: u32, height: u32, font_manager: Option<std::sync::Arc<std::sync::Mutex<crate::text::font_manager::FontManager>>>) -> LayerId {
        let id = LayerId::new();
        let mut layer = Layer::new(id, name, width, height);
        layer.width = Some(width);
        layer.height = Some(height);
        if let Some(fm) = font_manager {
            layer.font_manager = Some(fm);
        }
        self.layers.push(layer);
        id
    }
    
    /// Get layer by ID
    pub fn get(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }
    
    /// Merge all visible layers
    pub fn merge_all(&self) -> Result<RgbaImage> {
        use tiny_skia::{Pixmap, Paint, Transform};
        
        let visible_layers: Vec<_> = self.layers.iter()
            .filter(|l| l.visible)
            .collect();
        
        if visible_layers.is_empty() {
            return Ok(RgbaImage::new(self.base_width, self.base_height));
        }
        
        // Create result pixmap
        let mut result_pixmap = Pixmap::new(self.base_width, self.base_height)
            .ok_or_else(|| crate::error::CloveError::InvalidDimensions { 
                width: self.base_width, 
                height: self.base_height 
            })?;
        
        // Convert each layer to pixmap and composite
        for layer in visible_layers.iter() {
            // Convert layer content to pixmap
            let layer_pixmap = {
                let (width, height) = layer.dimensions();
                let mut pixmap = Pixmap::new(width, height)
                    .ok_or_else(|| crate::error::CloveError::InvalidDimensions { width, height })?;
                
                // Copy layer content to pixmap
                let img_data = layer.content.as_raw();
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
            
            // Create paint with opacity and blend mode
            let mut paint = Paint::default();
            paint.anti_alias = false;
            
            // Set blend mode
            match layer.blend_mode {
                BlendMode::Normal => paint.blend_mode = tiny_skia::BlendMode::SourceOver,
                BlendMode::Multiply => paint.blend_mode = tiny_skia::BlendMode::Multiply,
                BlendMode::Screen => paint.blend_mode = tiny_skia::BlendMode::Screen,
                BlendMode::Overlay => paint.blend_mode = tiny_skia::BlendMode::Overlay,
                BlendMode::Darken => paint.blend_mode = tiny_skia::BlendMode::Darken,
                BlendMode::Lighten => paint.blend_mode = tiny_skia::BlendMode::Lighten,
                BlendMode::ColorDodge => paint.blend_mode = tiny_skia::BlendMode::ColorDodge,
                BlendMode::ColorBurn => paint.blend_mode = tiny_skia::BlendMode::ColorBurn,
                BlendMode::HardLight => paint.blend_mode = tiny_skia::BlendMode::HardLight,
                BlendMode::SoftLight => paint.blend_mode = tiny_skia::BlendMode::SoftLight,
            }
            
            // Apply opacity by creating a pixmap paint
            let blend_mode = match layer.blend_mode {
                BlendMode::Normal => tiny_skia::BlendMode::SourceOver,
                BlendMode::Multiply => tiny_skia::BlendMode::Multiply,
                BlendMode::Screen => tiny_skia::BlendMode::Screen,
                BlendMode::Overlay => tiny_skia::BlendMode::Overlay,
                BlendMode::Darken => tiny_skia::BlendMode::Darken,
                BlendMode::Lighten => tiny_skia::BlendMode::Lighten,
                BlendMode::ColorDodge => tiny_skia::BlendMode::ColorDodge,
                BlendMode::ColorBurn => tiny_skia::BlendMode::ColorBurn,
                BlendMode::HardLight => tiny_skia::BlendMode::HardLight,
                BlendMode::SoftLight => tiny_skia::BlendMode::SoftLight,
            };
            
            let pixmap_paint = tiny_skia::PixmapPaint {
                quality: tiny_skia::FilterQuality::Nearest,
                opacity: layer.opacity,
                blend_mode,
            };
            
            // Draw layer onto result with position and transform
            let transform = Transform::from_translate(layer.x, layer.y);
            result_pixmap.draw_pixmap(
                0, 0,
                layer_pixmap.as_ref(),
                &pixmap_paint,
                transform,
                None,
            );
        }
        
        // Convert result pixmap back to RgbaImage
        let mut result = RgbaImage::new(self.base_width, self.base_height);
        let result_data = result.as_mut();
        let pixmap_data = result_pixmap.data();
        
        for (i, chunk) in pixmap_data.chunks_exact(4).enumerate() {
            let r_pre = chunk[0];
            let g_pre = chunk[1];
            let b_pre = chunk[2];
            let a = chunk[3];
            
            let idx = i * 4;
            if idx + 3 < result_data.len() && a > 0 {
                // Unpremultiply alpha
                let alpha_f = a as f32 / 255.0;
                result_data[idx] = (r_pre as f32 / alpha_f) as u8;
                result_data[idx + 1] = (g_pre as f32 / alpha_f) as u8;
                result_data[idx + 2] = (b_pre as f32 / alpha_f) as u8;
                result_data[idx + 3] = a;
            } else if idx + 3 < result_data.len() {
                result_data[idx] = r_pre;
                result_data[idx + 1] = g_pre;
                result_data[idx + 2] = b_pre;
                result_data[idx + 3] = a;
            }
        }
        
        Ok(result)
    }
}

