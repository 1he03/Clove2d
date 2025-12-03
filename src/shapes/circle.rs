use crate::layer::Layer;
use crate::color::Color;
use crate::error::Result;
use crate::shapes::Shadow;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Circle builder
pub struct CircleBuilder<'a> {
    layer: &'a mut Layer,
    center_x: f32,
    center_y: f32,
    radius: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
    shadow: Option<Shadow>,
    opacity: f32,
}

impl<'a> CircleBuilder<'a> {
    pub fn new(layer: &'a mut Layer, center_x: f32, center_y: f32, radius: f32) -> Self {
        Self {
            layer,
            center_x,
            center_y,
            radius,
            fill: None,
            stroke: None,
            shadow: None,
            opacity: 1.0,
        }
    }
    
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x;
        self.center_y = y;
        self
    }
    
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
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
        
        // Create path for circle
        let path = PathBuilder::from_circle(self.center_x, self.center_y, self.radius)
            .ok_or_else(|| {
                crate::error::CloveError::InvalidState("Failed to create circle path".to_string())
            })?;
        
        // Draw fill if specified
        let fill = self.fill.clone();
        
        if let Some(fill_color) = fill {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            
            match fill_color {
                Color::RadialGradient(grad) => {
                    // Use tiny-skia shader for radial gradient
                    if let Some(shader) = grad.to_skia_radial_gradient(Transform::identity()) {
                        paint.shader = shader;
                        // Apply opacity by modifying the shader colors or using a separate paint
                        // For now, we'll apply opacity to the final result
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
                Color::LinearGradient(grad) => {
                    // Use tiny-skia shader for linear gradient
                    if let Some(shader) = grad.to_skia_linear_gradient(Transform::identity()) {
                        paint.shader = shader;
                        // Apply opacity by modifying the shader colors or using a separate paint
                        // For now, we'll apply opacity to the final result
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
                _ => {
                    // Solid color
                    let rgba = fill_color.to_rgba();
                    // Apply opacity to alpha channel
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
            // Apply opacity to alpha channel
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

