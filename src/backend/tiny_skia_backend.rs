use crate::backend::RenderBackend;
use crate::color::Color;
use crate::error::{Result, CloveError};
use tiny_skia::{Pixmap, Color as SkiaColor};

/// Tiny-skia rendering backend
pub struct TinySkiaBackend {
    pixmap: Pixmap,
}

impl TinySkiaBackend {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let pixmap = Pixmap::new(width, height)
            .ok_or_else(|| CloveError::InvalidDimensions { width, height })?;
        Ok(Self { pixmap })
    }
    
    fn convert_color(color: &Color) -> SkiaColor {
        let rgba = color.to_rgba();
        SkiaColor::from_rgba8(rgba.r, rgba.g, rgba.b, rgba.a)
    }
    
    pub fn encode_png(&self) -> Result<Vec<u8>> {
        self.pixmap.encode_png()
            .map_err(|e| CloveError::ImageEncodeError(e.to_string()))
    }
    
    pub fn encode_jpeg(&self, quality: u8) -> Result<Vec<u8>> {
        // tiny-skia doesn't have JPEG encoding, so we'll use image crate
        let rgba_image = image::RgbaImage::from_raw(
            self.pixmap.width(),
            self.pixmap.height(),
            self.pixmap.data().to_vec(),
        ).ok_or_else(|| CloveError::ImageEncodeError("Failed to create image".to_string()))?;
        
        let mut buffer = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
        encoder.encode(
            rgba_image.as_raw(),
            self.pixmap.width(),
            self.pixmap.height(),
            image::ExtendedColorType::Rgba8,
        ).map_err(|e| CloveError::ImageEncodeError(e.to_string()))?;
        
        Ok(buffer)
    }
    
    pub fn encode_webp(&self) -> Result<Vec<u8>> {
        // WebP encoding would require webp crate
        Err(CloveError::UnsupportedFormat("WebP encoding not yet implemented".to_string()))
    }
}

impl RenderBackend for TinySkiaBackend {
    fn width(&self) -> u32 {
        self.pixmap.width()
    }
    
    fn height(&self) -> u32 {
        self.pixmap.height()
    }
    
    fn clear(&mut self, color: Color) {
        let skia_color = Self::convert_color(&color);
        self.pixmap.fill(skia_color);
    }
    
    fn get_pixmap(&self) -> &Pixmap {
        &self.pixmap
    }
    
    fn get_pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }
}

