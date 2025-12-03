use crate::layer::Layer;
use crate::color::Color;
use crate::geometry::Point;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Star builder
pub struct StarBuilder<'a> {
    layer: &'a mut Layer,
    center: Point,
    outer_radius: f32,
    inner_radius: f32,
    points: u32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
}

impl<'a> StarBuilder<'a> {
    pub fn new(layer: &'a mut Layer, center: Point, outer_radius: f32, inner_radius: f32, points: u32) -> Self {
        Self {
            layer,
            center,
            outer_radius,
            inner_radius,
            points,
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
        
        // Create star path
        let mut path_builder = PathBuilder::new();
        let angle_step = std::f32::consts::PI * 2.0 / (self.points as f32);
        
        for i in 0..self.points {
            // Outer point
            let outer_angle = i as f32 * angle_step - std::f32::consts::PI / 2.0;
            let outer_x = self.center.x + self.outer_radius * outer_angle.cos();
            let outer_y = self.center.y + self.outer_radius * outer_angle.sin();
            
            if i == 0 {
                path_builder.move_to(outer_x, outer_y);
            } else {
                path_builder.line_to(outer_x, outer_y);
            }
            
            // Inner point
            let inner_angle = (i as f32 + 0.5) * angle_step - std::f32::consts::PI / 2.0;
            let inner_x = self.center.x + self.inner_radius * inner_angle.cos();
            let inner_y = self.center.y + self.inner_radius * inner_angle.sin();
            path_builder.line_to(inner_x, inner_y);
        }
        path_builder.close();
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create star path".to_string())
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

