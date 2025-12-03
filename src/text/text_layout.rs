use crate::text::TextStyle;
use crate::geometry::Point;

/// Text layout with line breaking
pub struct TextLayout {
    pub text: String,
    pub style: TextStyle,
    pub position: Point,
    pub lines: Vec<String>,
}

impl TextLayout {
    pub fn new(text: &str, style: TextStyle, position: Point) -> Self {
        Self {
            text: text.to_string(),
            style,
            position,
            lines: Vec::new(),
        }
    }
    
    /// Break text into lines based on width
    pub fn break_lines(&mut self) {
        let max_width = match self.style.width {
            crate::text::TextWidth::None => None,
            crate::text::TextWidth::Max(w) => Some(w),
            crate::text::TextWidth::FullPage => None, // Use canvas width
            crate::text::TextWidth::Layer => None, // Use layer width
        };
        
        if let Some(width) = max_width {
            // Simple word wrapping
            let words: Vec<&str> = self.text.split_whitespace().collect();
            let mut current_line = String::new();
            
            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };
                
                // Estimate width (simplified - would use actual font metrics)
                let estimated_width = test_line.len() as f32 * self.style.font_size * 0.6;
                
                if estimated_width <= width {
                    current_line = test_line;
                } else {
                    if !current_line.is_empty() {
                        self.lines.push(current_line);
                    }
                    current_line = word.to_string();
                }
            }
            
            if !current_line.is_empty() {
                self.lines.push(current_line);
            }
        } else {
            // No width constraint - single line
            self.lines.push(self.text.clone());
        }
    }
}

