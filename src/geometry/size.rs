/// 2D size with width and height
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    /// Zero size (0, 0)
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };
    
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    
    /// Calculate area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
    
    /// Calculate aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.height != 0.0 {
            self.width / self.height
        } else {
            0.0
        }
    }
    
    /// Check if size is valid (both dimensions > 0)
    pub fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_size_area() {
        let size = Size::new(10.0, 20.0);
        assert_eq!(size.area(), 200.0);
    }
    
    #[test]
    fn test_size_aspect_ratio() {
        let size = Size::new(16.0, 9.0);
        assert_eq!(size.aspect_ratio(), 16.0 / 9.0);
    }
}

