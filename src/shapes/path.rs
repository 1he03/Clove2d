use crate::layer::Layer;
use crate::color::Color;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder as SkiaPathBuilder, Transform};

/// Path command
#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    ClosePath,
}

/// Path builder for custom paths
pub struct PathBuilder<'a> {
    layer: &'a mut Layer,
    commands: Vec<PathCommand>,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
}

impl<'a> PathBuilder<'a> {
    pub fn new(layer: &'a mut Layer) -> Self {
        Self {
            layer,
            commands: Vec::new(),
            fill: None,
            stroke: None,
        }
    }
    
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(x, y));
        self
    }
    
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::LineTo(x, y));
        self
    }
    
    pub fn curve_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::CurveTo(x1, y1, x2, y2, x, y));
        self
    }
    
    pub fn close_path(mut self) -> Self {
        self.commands.push(PathCommand::ClosePath);
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
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        if self.commands.is_empty() {
            return Ok(self.layer);
        }
        
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Build path from commands
        let mut path_builder = SkiaPathBuilder::new();
        let mut has_started = false;
        
        for command in &self.commands {
            match command {
                PathCommand::MoveTo(x, y) => {
                    path_builder.move_to(*x, *y);
                    has_started = true;
                }
                PathCommand::LineTo(x, y) => {
                    if !has_started {
                        return Err(crate::error::CloveError::InvalidState(
                            "Path must start with MoveTo".to_string()
                        ));
                    }
                    path_builder.line_to(*x, *y);
                }
                PathCommand::CurveTo(x1, y1, x2, y2, x, y) => {
                    if !has_started {
                        return Err(crate::error::CloveError::InvalidState(
                            "Path must start with MoveTo".to_string()
                        ));
                    }
                    path_builder.cubic_to(*x1, *y1, *x2, *y2, *x, *y);
                }
                PathCommand::ClosePath => {
                    path_builder.close();
                }
            }
        }
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create path".to_string())
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

