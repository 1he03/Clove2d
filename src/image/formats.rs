use crate::error::{Result, CloveError};

/// Image format enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}

impl ImageFormat {
    /// Detect format from file path extension
    pub fn from_path(path: &str) -> Result<Self> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| CloveError::UnsupportedFormat("No file extension".to_string()))?;
        
        match ext.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(CloveError::UnsupportedFormat(format!("Unsupported extension: {}", ext))),
        }
    }
    
    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::WebP => "image/webp",
        }
    }
}

