use thiserror::Error;

/// Main error type for Clove2d operations
#[derive(Debug, Error)]
pub enum CloveError {
    // Canvas errors
    #[error("Invalid canvas dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    
    #[error("Invalid canvas state: {0}")]
    InvalidState(String),
    
    #[error("Canvas state stack is empty")]
    NoSavedState,
    
    // Color errors
    #[error("Invalid hex color: {0}")]
    InvalidHexColor(String),
    
    #[error("Invalid color value for {component}: {value}")]
    InvalidColorValue { component: String, value: f32 },
    
    #[error("Invalid RGB value: r={r}, g={g}, b={b}")]
    InvalidRgb { r: u8, g: u8, b: u8 },
    
    #[error("Invalid HSL value: h={h}, s={s}, l={l}")]
    InvalidHsl { h: f32, s: f32, l: f32 },
    
    // Font errors
    #[error("Font not found: {0}")]
    FontNotFound(String),
    
    #[error("Font load error: {0}")]
    FontLoadError(String),
    
    #[error("Invalid font family: {0}")]
    InvalidFontFamily(String),
    
    // Image errors
    #[error("Image load error: {0}")]
    ImageLoadError(String),
    
    #[error("Image encode error: {0}")]
    ImageEncodeError(String),
    
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),
    
    // Layer errors
    #[error("Layer not found: {0}")]
    LayerNotFound(String),
    
    #[error("Invalid layer index: {0}")]
    InvalidLayerIndex(usize),
    
    #[error("Layer limit exceeded (max: {max})")]
    LayerLimitExceeded { max: usize },
    
    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    // Network errors (reqwest feature)
    #[cfg(feature = "reqwest")]
    #[error("Network error: {0}")]
    NetworkError(String),
    
    // Transform errors
    #[error("Invalid transform: {0}")]
    InvalidTransform(String),
    
    // Filter errors
    #[error("Filter error: {0}")]
    FilterError(String),
}

/// Result type alias for Clove2d operations
pub type Result<T> = std::result::Result<T, CloveError>;

