//! Prelude module - re-export commonly used types

pub use crate::error::{Result, CloveError};
pub use crate::geometry::{Point, Size, Rect, Bounds};
pub use crate::color::{Color, Rgba, Hsla, NamedColor, LinearGradient, RadialGradient, LinearGradientBuilder, RadialGradientBuilder};
pub use crate::canvas::{Canvas, CanvasBuilder};
pub use crate::text::{FontManager, TextStyle, TextAlign, TextWidth, FontWeight, FontStyle, TextBuilder};
pub use crate::layer::{Layer, LayerManager, BlendMode};
pub use crate::shapes::{RectBuilder, CircleBuilder, RoundedRectBuilder, EllipseBuilder, LineBuilder};
pub use crate::image::{ImageFormat, ImageLoader, Image};
pub use crate::filter::Filter;
pub use crate::transform::Transform;

