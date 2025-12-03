use crate::canvas::Canvas;
use crate::text::font_manager::FontManager;

/// Canvas state for save/restore
#[derive(Clone)]
pub struct CanvasState {
    font_manager: Option<FontManager>,
    // Future: transforms, clip regions, etc.
}

impl CanvasState {
    pub fn from_canvas(canvas: &Canvas) -> Self {
        Self {
            font_manager: canvas.font_manager().cloned(),
        }
    }
    
    pub fn apply_to_canvas(&self, canvas: &mut Canvas) {
        if let Some(ref fm) = self.font_manager {
            canvas.set_font_manager(fm.clone());
        }
    }
}

