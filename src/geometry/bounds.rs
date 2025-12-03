use crate::geometry::{Point, Rect};

/// Bounding box for geometric shapes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    /// Create bounds from min and max points
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }
    
    /// Create bounds from rectangle
    pub fn from_rect(rect: Rect) -> Self {
        Self {
            min: rect.origin(),
            max: rect.bottom_right(),
        }
    }
    
    /// Get width
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    
    /// Get height
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
    
    /// Check if point is within bounds
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
    
    /// Expand bounds to include point
    pub fn expand_to_include(&mut self, point: Point) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min: Point::ZERO,
            max: Point::ZERO,
        }
    }
}

