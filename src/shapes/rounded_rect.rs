use crate::layer::Layer;
use crate::color::Color;
use crate::error::Result;
use crate::shapes::Shadow;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Rounded rectangle builder
pub struct RoundedRectBuilder<'a> {
    layer: &'a mut Layer,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_radius: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
    shadow: Option<Shadow>,
    opacity: f32,
}

impl<'a> RoundedRectBuilder<'a> {
    pub fn new(layer: &'a mut Layer, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            layer,
            x,
            y,
            width,
            height,
            corner_radius: 0.0,
            fill: None,
            stroke: None,
            shadow: None,
            opacity: 1.0,
        }
    }
    
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
    
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn shadow(mut self, color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        self.shadow = Some(Shadow::new(color, offset_x, offset_y, blur));
        self
    }
    
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Create rounded rectangle path manually
        let mut path_builder = PathBuilder::new();
        let r = self.corner_radius.min(self.width / 2.0).min(self.height / 2.0);
        
        // Top-left corner
        path_builder.move_to(self.x + r, self.y);
        // Top edge
        path_builder.line_to(self.x + self.width - r, self.y);
        // Top-right corner
        path_builder.quad_to(
            self.x + self.width, self.y,
            self.x + self.width, self.y + r,
        );
        // Right edge
        path_builder.line_to(self.x + self.width, self.y + self.height - r);
        // Bottom-right corner
        path_builder.quad_to(
            self.x + self.width, self.y + self.height,
            self.x + self.width - r, self.y + self.height,
        );
        // Bottom edge
        path_builder.line_to(self.x + r, self.y + self.height);
        // Bottom-left corner
        path_builder.quad_to(
            self.x, self.y + self.height,
            self.x, self.y + self.height - r,
        );
        // Left edge
        path_builder.line_to(self.x, self.y + r);
        // Top-left corner
        path_builder.quad_to(
            self.x, self.y,
            self.x + r, self.y,
        );
        path_builder.close();
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create rounded rectangle path".to_string())
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

