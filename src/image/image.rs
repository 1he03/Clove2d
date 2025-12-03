use crate::error::{Result, CloveError};
use image::RgbaImage;

/// Image struct for loading and drawing images
#[derive(Clone)]
pub struct Image {
    pub data: RgbaImage,
}

impl Image {
    /// Load image from file path
    pub fn from_path(path: &str) -> Result<Self> {
        let img = image::open(path)
            .map_err(|e| CloveError::ImageLoadError(e.to_string()))?;
        Ok(Self {
            data: img.to_rgba8(),
        })
    }
    
    /// Get image width
    pub fn width(&self) -> u32 {
        self.data.width()
    }
    
    /// Get image height
    pub fn height(&self) -> u32 {
        self.data.height()
    }
}

