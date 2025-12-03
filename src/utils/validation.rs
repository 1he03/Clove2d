use crate::error::{Result, CloveError};

/// Validate canvas dimensions
pub fn validate_dimensions(width: u32, height: u32) -> Result<()> {
    if width == 0 || height == 0 {
        return Err(CloveError::InvalidDimensions { width, height });
    }
    if width > 16384 || height > 16384 {
        return Err(CloveError::InvalidDimensions { width, height });
    }
    Ok(())
}

/// Validate opacity value (0.0 to 1.0)
pub fn validate_opacity(opacity: f32) -> Result<f32> {
    if !(0.0..=1.0).contains(&opacity) {
        return Err(CloveError::InvalidColorValue {
            component: "opacity".to_string(),
            value: opacity,
        });
    }
    Ok(opacity)
}

/// Validate color component value
pub fn validate_color_component(component: &str, value: f32, min: f32, max: f32) -> Result<f32> {
    if !(min..=max).contains(&value) {
        return Err(CloveError::InvalidColorValue {
            component: component.to_string(),
            value,
        });
    }
    Ok(value)
}

