use crate::color::Color;
use crate::geometry::Point;
use tiny_skia::{Shader, Transform};

/// Gradient stop with color and position
#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub position: f32, // 0.0 to 1.0
}

impl GradientStop {
    pub fn new(color: Color, position: f32) -> Self {
        Self { color, position }
    }
}

/// Linear gradient builder
#[derive(Debug, Clone)]
pub struct LinearGradientBuilder {
    start: Option<Point>,
    end: Option<Point>,
    stops: Vec<GradientStop>,
}

impl LinearGradientBuilder {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            stops: Vec::new(),
        }
    }
    
    pub fn start(mut self, x: f32, y: f32) -> Self {
        self.start = Some(Point::new(x, y));
        self
    }
    
    pub fn end(mut self, x: f32, y: f32) -> Self {
        self.end = Some(Point::new(x, y));
        self
    }
    
    pub fn add_stop(mut self, position: f32, color: Color) -> Self {
        self.stops.push(GradientStop::new(color, position));
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        self
    }
    
    pub fn build(self) -> crate::color::Color {
        let gradient = LinearGradient {
            start: self.start.unwrap_or(Point::ZERO),
            end: self.end.unwrap_or(Point::ZERO),
            stops: self.stops,
        };
        crate::color::Color::LinearGradient(gradient)
    }
}

impl Default for LinearGradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear gradient
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub start: Point,
    pub end: Point,
    pub stops: Vec<GradientStop>,
}

impl LinearGradient {
    pub fn new() -> LinearGradientBuilder {
        LinearGradientBuilder::new()
    }
    
    /// Convert to tiny-skia LinearGradient shader
    pub fn to_skia_linear_gradient(&self, _transform: Transform) -> Option<Shader> {
        if self.stops.is_empty() {
            return None;
        }
        
        // Convert stops to tiny-skia format
        let skia_stops: Vec<tiny_skia::GradientStop> = self.stops.iter()
            .map(|stop| {
                let rgba = stop.color.to_rgba();
                let pos = stop.position.clamp(0.0, 1.0);
                tiny_skia::GradientStop::new(pos, tiny_skia::Color::from_rgba8(rgba.r, rgba.g, rgba.b, rgba.a))
            })
            .collect();
        
        // Create linear gradient shader
        tiny_skia::LinearGradient::new(
            tiny_skia::Point::from_xy(self.start.x, self.start.y),
            tiny_skia::Point::from_xy(self.end.x, self.end.y),
            skia_stops,
            tiny_skia::SpreadMode::Pad,
            Transform::identity(),
        )
    }
}

/// Radial gradient builder
#[derive(Debug, Clone)]
pub struct RadialGradientBuilder {
    center: Option<Point>,
    radius: Option<f32>,
    stops: Vec<GradientStop>,
}

impl RadialGradientBuilder {
    pub fn new() -> Self {
        Self {
            center: None,
            radius: None,
            stops: Vec::new(),
        }
    }
    
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center = Some(Point::new(x, y));
        self
    }
    
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = Some(r);
        self
    }
    
    pub fn add_stop(mut self, position: f32, color: Color) -> Self {
        self.stops.push(GradientStop::new(color, position));
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        self
    }
    
    pub fn build(self) -> crate::color::Color {
        let gradient = RadialGradient {
            center: self.center.unwrap_or(Point::ZERO),
            radius: self.radius.unwrap_or(50.0),
            stops: self.stops,
        };
        crate::color::Color::RadialGradient(gradient)
    }
}

impl Default for RadialGradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Radial gradient
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    pub center: Point,
    pub radius: f32,
    pub stops: Vec<GradientStop>,
}

impl RadialGradient {
    pub fn new() -> RadialGradientBuilder {
        RadialGradientBuilder::new()
    }
    
    /// Convert to tiny-skia RadialGradient shader
    pub fn to_skia_radial_gradient(&self, _transform: Transform) -> Option<Shader> {
        if self.stops.is_empty() {
            return None;
        }
        
        // Convert stops to tiny-skia format
        let skia_stops: Vec<tiny_skia::GradientStop> = self.stops.iter()
            .map(|stop| {
                let rgba = stop.color.to_rgba();
                let pos = stop.position.clamp(0.0, 1.0);
                tiny_skia::GradientStop::new(pos, tiny_skia::Color::from_rgba8(rgba.r, rgba.g, rgba.b, rgba.a))
            })
            .collect();
        
        // Create radial gradient shader (circular)
        // tiny-skia RadialGradient::new expects: (center, focal_point, radius, stops, spread_mode, transform)
        // For circular gradient, center and focal_point are the same
        tiny_skia::RadialGradient::new(
            tiny_skia::Point::from_xy(self.center.x, self.center.y),
            tiny_skia::Point::from_xy(self.center.x, self.center.y),
            self.radius,
            skia_stops,
            tiny_skia::SpreadMode::Pad,
            Transform::identity(),
        )
    }
}
