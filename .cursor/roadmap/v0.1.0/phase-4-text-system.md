# Phase 4: Text System

## @context: v0.1.0 Phase 4
## @timeline: Week 7-8 (2 weeks)
## @priority: Critical
## @status: Planning
## @dependencies: Phase 3 (Shapes)
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Overview](#overview)
2. [Objectives](#objectives)
3. [Font Management](#font-management)
4. [Text Rendering](#text-rendering)
5. [Arabic Support](#arabic-support)
6. [Testing Requirements](#testing-requirements)

---

## Overview

Phase 4 implements advanced text rendering with full Arabic/BiDi support using cosmic-text and rustybuzz.

---

## Objectives

### Primary Goals

- ✅ Font Manager with caching
- ✅ Basic text rendering
- ✅ Multi-line text with wrapping
- ✅ Arabic text shaping
- ✅ BiDi algorithm

### Success Metrics

- [ ] Arabic text renders correctly
- [ ] BiDi algorithm works
- [ ] Text wrapping is intelligent
- [ ] Render text <20ms

---

## Font Management

**File:** `src/text/font_manager.rs`

```rust
use cosmic_text::{FontSystem, Font};
use std::collections::HashMap;

pub struct FontManager {
    font_system: FontSystem,
    fonts: HashMap<String, Font>,
    default_family: String,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            fonts: HashMap::new(),
            default_family: "Arial".to_string(),
        }
    }
    
    pub fn load(&mut self, name: &str, path: &str) -> Result<&mut Self> {
        let font_data = std::fs::read(path)
            .map_err(|e| CloveError::FontLoadError(e.to_string()))?;
        
        // Load with cosmic-text
        self.font_system.db_mut()
            .load_font_data(font_data);
        
        Ok(self)
    }
    
    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if !self.fonts.contains_key(name) {
            return Err(CloveError::FontNotFound(name.to_string()));
        }
        self.default_family = name.to_string();
        Ok(())
    }
}
```

---

## Text Rendering

**File:** `src/text/text_style.rs`

```rust
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub color: Color,
    pub letter_spacing: f32,
    pub line_height: f32,
    pub align: TextAlign,
    pub direction: TextDirection,
}

pub enum FontWeight {
    Normal,
    Bold,
    Light,
    // ...
}

pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

pub enum TextDirection {
    LTR,
    RTL,
    Auto,
}
```

---

## Arabic Support

**File:** `src/text/shaping.rs`

```rust
use rustybuzz::shape;
use unicode_bidi::BidiInfo;

pub fn shape_arabic_text(text: &str, font: &Font) -> Vec<GlyphInfo> {
    // Use rustybuzz for Arabic shaping
    let buffer = rustybuzz::UnicodeBuffer::new()
        .add_str(text);
    
    let shaped = shape(&font, &[], buffer);
    // ...
}

pub fn apply_bidi(text: &str) -> BidiInfo {
    // Use unicode-bidi for BiDi algorithm
    BidiInfo::new(text, None)
}
```

---

## Testing Requirements

```rust
#[test]
fn test_arabic_rendering() -> Result<()> {
    let mut fonts = FontManager::new();
    fonts.load("Tajawal", "assets/fonts/Tajawal-Regular.ttf")?;
    
    let mut canvas = Canvas::new(800, 400)?;
    canvas.create_layer("text")?
        .draw_text("مرحباً بكم")
            .font_family("Tajawal")
            .font_size(48)
            .direction(TextDirection::RTL)
            .draw()?;
    
    canvas.save("arabic_test.png")?;
    Ok(())
}
```

---

**Phase:** 4/6  
**Status:** Planning  
**Duration:** 2 weeks  
**Last Updated:** 2025-11-27
