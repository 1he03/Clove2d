use unicode_bidi::BidiInfo;

/// Apply BiDi algorithm to text
pub fn apply_bidi(text: &str) -> BidiInfo {
    BidiInfo::new(text, None)
}

/// Get text direction (LTR or RTL)
pub fn get_text_direction(bidi_info: &BidiInfo) -> bool {
    // Returns true for RTL, false for LTR
    // Simplified implementation
    bidi_info.paragraphs.iter()
        .any(|p| {
            let start = p.range.start;
            let end = p.range.end;
            bidi_info.levels[start..end].iter().any(|&l| l.is_rtl())
        })
}

