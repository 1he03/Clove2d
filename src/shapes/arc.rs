use crate::layer::Layer;
use crate::color::Color;
use crate::geometry::Point;
use crate::error::Result;
use tiny_skia::{Paint, PathBuilder, Transform};

/// Arc builder
pub struct ArcBuilder<'a> {
    layer: &'a mut Layer,
    center: Point,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    stroke: Option<(Color, f32)>,
    fill: Option<Color>,
}

impl<'a> ArcBuilder<'a> {
    pub fn new(layer: &'a mut Layer, center: Point, radius: f32, start_angle: f32, end_angle: f32) -> Self {
        Self {
            layer,
            center,
            radius,
            start_angle,
            end_angle,
            stroke: None,
            fill: None,
        }
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Convert angles from degrees to radians
        let start_rad = self.start_angle.to_radians();
        let end_rad = self.end_angle.to_radians();
        
        // Create arc path using smooth bezier curves
        let mut path_builder = PathBuilder::new();
        
        // Start point
        let start_x = self.center.x + self.radius * start_rad.cos();
        let start_y = self.center.y + self.radius * start_rad.sin();
        path_builder.move_to(start_x, start_y);
        
        // Create arc using cubic bezier curves for smooth arc
        let angle_diff = end_rad - start_rad;
        
        // Use bezier curves to approximate circular arc smoothly
        // Split arc into segments of max 90 degrees each
        let max_segment_angle = std::f32::consts::PI / 2.0;
        let num_segments = (angle_diff.abs() / max_segment_angle).ceil() as u32;
        let segment_angle = angle_diff / num_segments as f32;
        
        for i in 0..num_segments {
            let seg_start_angle = start_rad + segment_angle * i as f32;
            let seg_end_angle = start_rad + segment_angle * (i as f32 + 1.0);
            
            // Calculate points for bezier curve
            let p0_x = self.center.x + self.radius * seg_start_angle.cos();
            let p0_y = self.center.y + self.radius * seg_start_angle.sin();
            
            let p3_x = self.center.x + self.radius * seg_end_angle.cos();
            let p3_y = self.center.y + self.radius * seg_end_angle.sin();
            
            // Control points for smooth circular arc approximation
            // Using the formula: control_point_distance = radius * (4/3) * tan(angle/4)
            let control_distance = self.radius * (4.0 / 3.0) * (segment_angle.abs() / 4.0).tan();
            
            let p1_x = p0_x - control_distance * seg_start_angle.sin();
            let p1_y = p0_y + control_distance * seg_start_angle.cos();
            
            let p2_x = p3_x + control_distance * seg_end_angle.sin();
            let p2_y = p3_y - control_distance * seg_end_angle.cos();
            
            if i == 0 {
                path_builder.move_to(p0_x, p0_y);
            }
            
            path_builder.cubic_to(p1_x, p1_y, p2_x, p2_y, p3_x, p3_y);
        }
        
        // If fill is requested, close the path to center
        if self.fill.is_some() {
            path_builder.line_to(self.center.x, self.center.y);
            path_builder.close();
        }
        
        let path = path_builder.finish().ok_or_else(|| {
            crate::error::CloveError::InvalidState("Failed to create arc path".to_string())
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

