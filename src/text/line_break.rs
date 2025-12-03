use unicode_segmentation::UnicodeSegmentation;

/// Line break algorithm for text wrapping
pub struct LineBreaker;

impl LineBreaker {
    /// Break text into lines based on width
    pub fn break_lines(text: &str, max_width: Option<f32>, font_size: f32) -> Vec<String> {
        if let Some(width) = max_width {
            // Estimate character width (simplified)
            let char_width = font_size * 0.6;
            let max_chars = (width / char_width) as usize;
            
            // Break by grapheme clusters for proper Unicode handling
            let graphemes: Vec<&str> = text.graphemes(true).collect();
            let mut lines = Vec::new();
            let mut current_line = String::new();
            
            for grapheme in graphemes {
                if current_line.len() + grapheme.len() > max_chars && !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = grapheme.to_string();
                } else {
                    current_line.push_str(grapheme);
                }
            }
            
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            
            lines
        } else {
            vec![text.to_string()]
        }
    }
}

