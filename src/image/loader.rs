use crate::error::{Result, CloveError};
use image::RgbaImage;

/// Image loader for loading images from various sources
pub struct ImageLoader;

impl ImageLoader {
    /// Load image from file path
    pub fn load_from_path(path: &str) -> Result<RgbaImage> {
        let img = image::open(path)
            .map_err(|e| CloveError::ImageLoadError(e.to_string()))?;
        Ok(img.to_rgba8())
    }
    
    /// Load image from URL (reqwest feature)
    #[cfg(feature = "reqwest")]
    pub async fn load_from_url(url: &str) -> Result<RgbaImage> {
        let bytes = reqwest::get(url)
            .await
            .map_err(|e| CloveError::NetworkError(e.to_string()))?
            .bytes()
            .await
            .map_err(|e| CloveError::NetworkError(e.to_string()))?;
        
        Self::load_from_buffer(&bytes)
    }
    
    /// Load image from byte buffer
    pub fn load_from_buffer(buffer: &[u8]) -> Result<RgbaImage> {
        let img = image::load_from_memory(buffer)
            .map_err(|e| CloveError::ImageLoadError(e.to_string()))?;
        Ok(img.to_rgba8())
    }
}

