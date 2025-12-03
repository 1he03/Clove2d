use crate::layer::Layer;
use crate::color::Color;
use crate::error::Result;
use crate::shapes::Shadow;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Ellipse builder
pub struct EllipseBuilder<'a> {
    layer: &'a mut Layer,
    center_x: f32,
    center_y: f32,
    radius_x: f32,
    radius_y: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
    shadow: Option<Shadow>,
    opacity: f32,
}

impl<'a> EllipseBuilder<'a> {
    pub fn new(layer: &'a mut Layer, center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            layer,
            center_x,
            center_y,
            radius_x,
            radius_y,
            fill: None,
            stroke: None,
            shadow: None,
            opacity: 1.0,
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
        
        // Create ellipse path manually using bezier curves
        let mut path_builder = PathBuilder::new();
        
        // Approximate ellipse using cubic bezier curves
        // Using the method from: https://spencermortensen.com/articles/bezier-circle/
        let magic = 0.551915024494; // 4/3 * (sqrt(2) - 1)
        let cpx = self.radius_x * magic;
        let cpy = self.radius_y * magic;
        
        // Start at top
        path_builder.move_to(self.center_x, self.center_y - self.radius_y);
        
        // Top-right quadrant
        path_builder.cubic_to(
            self.center_x + cpx, self.center_y - self.radius_y,
            self.center_x + self.radius_x, self.center_y - cpy,
            self.center_x + self.radius_x, self.center_y,
        );
        
        // Bottom-right quadrant
        path_builder.cubic_to(
            self.center_x + self.radius_x, self.center_y + cpy,
            self.center_x + cpx, self.center_y + self.radius_y,
            self.center_x, self.center_y + self.radius_y,
        );
        
        // Bottom-left quadrant
        path_builder.cubic_to(
            self.center_x - cpx, self.center_y + self.radius_y,
            self.center_x - self.radius_x, self.center_y + cpy,
            self.center_x - self.radius_x, self.center_y,
        );
        
        // Top-left quadrant
        path_builder.cubic_to(
            self.center_x - self.radius_x, self.center_y - cpy,
            self.center_x - cpx, self.center_y - self.radius_y,
            self.center_x, self.center_y - self.radius_y,
        );
        
        path_builder.close();
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create ellipse path".to_string())
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
                    let alpha = (self.opacity * (rgba.a as f32 / 255.0) * 255.0) as u8;
                    paint.set_color_rgba8(rgba.r, rgba.g, rgba.b, alpha);
                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                }
            }
        }
        
        // Draw stroke if specified
        if let Some((stroke_color, stroke_width)) = self.stroke {
            let rgba = stroke_color.to_rgba();
            let mut paint = Paint::default();
            let alpha = (self.opacity * (rgba.a as f32 / 255.0) * 255.0) as u8;
            paint.set_color_rgba8(rgba.r, rgba.g, rgba.b, alpha);
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

