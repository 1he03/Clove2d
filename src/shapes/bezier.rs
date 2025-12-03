use crate::layer::Layer;
use crate::color::Color;
use crate::geometry::Point;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Bezier curve builder (Quadratic & Cubic)
pub struct BezierBuilder<'a> {
    layer: &'a mut Layer,
    start: Point,
    control1: Point,
    control2: Option<Point>, // None for quadratic, Some for cubic
    end: Point,
    stroke: Option<(Color, f32)>,
}

impl<'a> BezierBuilder<'a> {
    /// Create quadratic bezier curve
    pub fn quadratic(layer: &'a mut Layer, start: Point, control: Point, end: Point) -> Self {
        Self {
            layer,
            start,
            control1: control,
            control2: None,
            end,
            stroke: None,
        }
    }
    
    /// Create cubic bezier curve
    pub fn cubic(layer: &'a mut Layer, start: Point, control1: Point, control2: Point, end: Point) -> Self {
        Self {
            layer,
            start,
            control1,
            control2: Some(control2),
            end,
            stroke: None,
        }
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Create bezier path
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(self.start.x, self.start.y);
        
        match self.control2 {
            Some(control2) => {
                // Cubic bezier
                path_builder.cubic_to(
                    self.control1.x, self.control1.y,
                    control2.x, control2.y,
                    self.end.x, self.end.y,
                );
            }
            None => {
                // Quadratic bezier
                path_builder.quad_to(
                    self.control1.x, self.control1.y,
                    self.end.x, self.end.y,
                );
            }
        }
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create bezier path".to_string())
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

