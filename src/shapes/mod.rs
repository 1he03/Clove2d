pub mod rectangle;
pub mod rounded_rect;
pub mod circle;
pub mod ellipse;
pub mod line;
pub mod polyline;
pub mod polygon;
pub mod triangle;
pub mod arc;
pub mod bezier;
pub mod path;
pub mod star;

pub use rectangle::RectBuilder;
pub use rounded_rect::RoundedRectBuilder;
pub use circle::CircleBuilder;
pub use ellipse::EllipseBuilder;
pub use line::LineBuilder;
pub use polyline::PolylineBuilder;
pub use polygon::PolygonBuilder;
pub use triangle::TriangleBuilder;
pub use arc::ArcBuilder;
pub use bezier::BezierBuilder;
pub use path::PathBuilder;
pub use star::StarBuilder;

use crate::color::Color;

/// Shadow for shapes
#[derive(Debug, Clone)]
pub struct Shadow {
    pub color: Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
}

impl Shadow {
    pub fn new(color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        Self {
            color,
            offset_x,
            offset_y,
            blur,
        }
    }
}

