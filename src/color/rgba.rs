use crate::error::{Result, CloveError};

/// RGBA color with 8-bit components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Transparent color
    pub const TRANSPARENT: Rgba = Rgba { r: 0, g: 0, b: 0, a: 0 };
    
    /// White color
    pub const WHITE: Rgba = Rgba { r: 255, g: 255, b: 255, a: 255 };
    
    /// Black color
    pub const BLACK: Rgba = Rgba { r: 0, g: 0, b: 0, a: 255 };
    
    /// Create new RGBA color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create RGB color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    /// Parse hex color string (#RRGGBB or #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 && hex.len() != 8 {
            return Err(CloveError::InvalidHexColor(hex.to_string()));
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| CloveError::InvalidHexColor(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| CloveError::InvalidHexColor(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| CloveError::InvalidHexColor(hex.to_string()))?;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| CloveError::InvalidHexColor(hex.to_string()))?
        } else {
            255
        };
        
        Ok(Self::new(r, g, b, a))
    }
    
    /// Convert to HSLA
    pub fn to_hsla(&self) -> Result<crate::color::Hsla> {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let a = self.a as f32 / 255.0;
        
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };
        
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let l = (max + min) / 2.0;
        
        crate::color::Hsla::new(h, s, l, a)
    }
    
    /// Get as f32 tuple (0.0-1.0 range)
    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_parsing() {
        let color = Rgba::from_hex("#FF5733").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);
        assert_eq!(color.a, 255);
    }
    
    #[test]
    fn test_rgba_to_hsla() {
        let rgba = Rgba::rgb(255, 0, 0);
        let hsla = rgba.to_hsla().unwrap();
        assert_eq!(hsla.h, 0.0);
        assert_eq!(hsla.s, 1.0);
        assert_eq!(hsla.l, 0.5);
    }
}

