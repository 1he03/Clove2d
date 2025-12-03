use crate::error::{Result, CloveError};

/// HSLA color with floating-point components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub h: f32, // Hue: 0-360
    pub s: f32, // Saturation: 0-1
    pub l: f32, // Lightness: 0-1
    pub a: f32, // Alpha: 0-1
}

impl Hsla {
    /// Create new HSLA color
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Result<Self> {
        if !(0.0..=360.0).contains(&h) {
            return Err(CloveError::InvalidColorValue {
                component: "hue".to_string(),
                value: h,
            });
        }
        if !(0.0..=1.0).contains(&s) {
            return Err(CloveError::InvalidColorValue {
                component: "saturation".to_string(),
                value: s,
            });
        }
        if !(0.0..=1.0).contains(&l) {
            return Err(CloveError::InvalidColorValue {
                component: "lightness".to_string(),
                value: l,
            });
        }
        if !(0.0..=1.0).contains(&a) {
            return Err(CloveError::InvalidColorValue {
                component: "alpha".to_string(),
                value: a,
            });
        }
        
        Ok(Self { h, s, l, a })
    }
    
    /// Convert to RGBA
    pub fn to_rgba(&self) -> crate::color::Rgba {
        let c = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.l - c / 2.0;
        
        let (r, g, b) = if self.h < 60.0 {
            (c, x, 0.0)
        } else if self.h < 120.0 {
            (x, c, 0.0)
        } else if self.h < 180.0 {
            (0.0, c, x)
        } else if self.h < 240.0 {
            (0.0, x, c)
        } else if self.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        crate::color::Rgba::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hsla_to_rgba() {
        let hsla = Hsla::new(0.0, 1.0, 0.5, 1.0).unwrap();
        let rgba = hsla.to_rgba();
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 0);
    }
}

