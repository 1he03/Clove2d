use crate::color::Color;

/// Trait for rendering backends
pub trait RenderBackend {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn get_pixmap(&self) -> &tiny_skia::Pixmap;
    fn get_pixmap_mut(&mut self) -> &mut tiny_skia::Pixmap;
}

