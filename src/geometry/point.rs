/// 2D point with floating-point coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Origin point (0, 0)
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };
    
    /// Create a new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Calculate distance to another point
    pub fn distance_to(&self, other: Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Calculate midpoint between this point and another
    pub fn midpoint(&self, other: Point) -> Point {
        Point::new(
            (self.x + other.x) / 2.0,
            (self.y + other.y) / 2.0,
        )
    }
    
    /// Translate point by offset
    pub fn translate(&self, dx: f32, dy: f32) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);
    }
    
    #[test]
    fn test_point_midpoint() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(4.0, 6.0);
        let mid = p1.midpoint(p2);
        assert_eq!(mid.x, 2.0);
        assert_eq!(mid.y, 3.0);
    }
}

