use crate::layer::Layer;
use crate::color::Color;
use crate::geometry::Point;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Triangle builder
pub struct TriangleBuilder<'a> {
    layer: &'a mut Layer,
    p1: Point,
    p2: Point,
    p3: Point,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
}

impl<'a> TriangleBuilder<'a> {
    pub fn new(layer: &'a mut Layer, p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            layer,
            p1,
            p2,
            p3,
            fill: None,
            stroke: None,
        }
    }
    
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Create triangle path
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(self.p1.x, self.p1.y);
        path_builder.line_to(self.p2.x, self.p2.y);
        path_builder.line_to(self.p3.x, self.p3.y);
        path_builder.close();
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create triangle path".to_string())
        })?;
        
        // Draw fill if specified
        if let Some(fill_color) = self.fill.clone() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            
            match fill_color {
                Color::LinearGradient(grad) => {
                    if let Some(shader) = grad.to_skia_linear_gradient(Transform::identity()) {
                        paint.shader = shader;
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
                Color::RadialGradient(grad) => {
                    if let Some(shader) = grad.to_skia_radial_gradient(Transform::identity()) {
                        paint.shader = shader;
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
                _ => {
                    let rgba = fill_color.to_rgba();
                    paint.set_color_rgba8(rgba.r, rgba.g, rgba.b, rgba.a);
                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                }
            }
        }
        
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

