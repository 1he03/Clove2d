use crate::geometry::Rect;

/// Transform struct for geometric transformations
#[derive(Debug, Clone)]
pub struct Transform {
    pub rotation: f32,      // degrees
    pub scale_x: f32,
    pub scale_y: f32,
    pub translate_x: f32,
    pub translate_y: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
        }
    }
    
    pub fn rotate(mut self, degrees: f32) -> Self {
        self.rotation = degrees;
        self
    }
    
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }
    
    pub fn crop(self, _rect: Rect) -> Self {
        // Crop would be handled differently in practice
        self
    }
    
    pub fn flip_horizontal(mut self) -> Self {
        self.scale_x = -self.scale_x;
        self
    }
    
    pub fn flip_vertical(mut self) -> Self {
        self.scale_y = -self.scale_y;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

