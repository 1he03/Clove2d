use crate::layer::Layer;
use crate::color::Color;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Line builder
pub struct LineBuilder<'a> {
    layer: &'a mut Layer,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke: Option<(Color, f32)>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(layer: &'a mut Layer, x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            layer,
            x1,
            y1,
            x2,
            y2,
            stroke: None,
        }
    }
    
    pub fn from(mut self, x: f32, y: f32) -> Self {
        self.x1 = x;
        self.y1 = y;
        self
    }
    
    pub fn to(mut self, x: f32, y: f32) -> Self {
        self.x2 = x;
        self.y2 = y;
        self
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Create line path
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(self.x1, self.y1);
        path_builder.line_to(self.x2, self.y2);
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create line path".to_string())
        })?;
        
        // Draw stroke if specified
        if let Some((stroke_color, stroke_width)) = self.stroke {
            let rgba = stroke_color.to_rgba();
            let mut paint = Paint::default();
            paint.set_color_rgba8(rgba.r, rgba.g, rgba.b, rgba.a);
            paint.anti_alias = true;
            
            let mut stroke = tiny_skia::Stroke::default();
            stroke.width = stroke_width;
            
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
        
        // Update layer content from pixmap
        self.layer.update_from_pixmap(&pixmap)?;
        
        Ok(self.layer)
    }
}

