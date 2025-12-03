# Phase 2: Canvas Core

## @context: v0.1.0 Phase 2
## @timeline: Week 3-4 (2 weeks)
## @priority: Critical
## @status: Planning
## @dependencies: Phase 1 (Foundation)
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Overview](#overview)
2. [Objectives](#objectives)
3. [Deliverables](#deliverables)
4. [Implementation Details](#implementation-details)
5. [Testing Requirements](#testing-requirements)
6. [Definition of Done](#definition-of-done)

---

## Overview

Phase 2 builds the core Canvas system - the main entry point for all drawing operations. This phase integrates tiny-skia as the rendering backend and establishes the builder pattern API.

### Key Focus Areas

1. **Canvas Struct** - Main canvas type and state
2. **Canvas Builder** - Fluent construction API
3. **Backend Integration** - tiny-skia renderer
4. **State Management** - Save/restore stack
5. **Save/Load Operations** - Image encoding and file I/O

---

## Objectives

### Primary Goals

- ✅ Create functional Canvas with builder pattern
- ✅ Integrate tiny-skia rendering backend
- ✅ Implement state save/restore
- ✅ Support PNG, JPEG, WebP export

### Success Metrics

- [ ] Canvas creation <10ms for 800x600
- [ ] Save PNG without quality loss
- [ ] State stack works correctly
- [ ] All public APIs documented

---

## Deliverables

### 1. Canvas Struct

**Files:**
- `src/canvas/mod.rs`
- `src/canvas/builder.rs`
- `src/canvas/operations.rs`
- `src/canvas/state.rs`

#### Canvas Type

```rust
pub struct Canvas {
    backend: TinySkiaBackend,
    layers: LayerManager,
    state_stack: Vec<CanvasState>,
    width: u32,
    height: u32,
}

impl Canvas {
    /// Create a new canvas builder
    pub fn builder() -> CanvasBuilder {
        CanvasBuilder::new()
    }
    
    /// Create canvas with default settings
    pub fn new(width: u32, height: u32) -> Result<Self> {
        Self::builder()
            .size(width, height)
            .build()
    }
    
    /// Get canvas width
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get canvas height
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Clear canvas with color
    pub fn clear(&mut self, color: Color) -> &mut Self {
        self.backend.clear(color);
        self
    }
    
    /// Save current state
    pub fn save_state(&mut self) -> &mut Self {
        let state = CanvasState::from_canvas(self);
        self.state_stack.push(state);
        self
    }
    
    /// Restore previous state
    pub fn restore_state(&mut self) -> Result<&mut Self> {
        if let Some(state) = self.state_stack.pop() {
            state.apply_to_canvas(self);
            Ok(self)
        } else {
            Err(CloveError::InvalidState(
                "No saved state to restore".to_string()
            ))
        }
    }
    
    /// Save to file
    pub fn save(&self, path: &str) -> Result<()> {
        let format = ImageFormat::from_path(path)?;
        let buffer = self.to_buffer(format)?;
        std::fs::write(path, buffer)?;
        Ok(())
    }
    
    /// Save with quality (JPEG only)
    pub fn save_with_quality(&self, path: &str, quality: u8) -> Result<()> {
        let buffer = self.backend.encode_jpeg(quality)?;
        std::fs::write(path, buffer)?;
        Ok(())
    }
    
    /// Export to buffer
    pub fn to_buffer(&self, format: ImageFormat) -> Result<Vec<u8>> {
        match format {
            ImageFormat::Png => self.backend.encode_png(),
            ImageFormat::Jpeg => self.backend.encode_jpeg(90),
            ImageFormat::WebP => self.backend.encode_webp(),
        }
    }
}
```

**Tasks:**
- [ ] Implement Canvas struct
- [ ] Add width/height getters
- [ ] Implement clear method
- [ ] Add state management
- [ ] Implement save methods
- [ ] Write tests
- [ ] Add examples

---

### 2. Canvas Builder

```rust
pub struct CanvasBuilder {
    width: Option<u32>,
    height: Option<u32>,
    background: Option<Color>,
    font_manager: Option<FontManager>,  // Optional font manager
}

impl CanvasBuilder {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            background: None,
            font_manager: None,
        }
    }
    
    /// Set canvas size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    
    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
    
    /// Set font manager (optional)
    /// If set, all text drawing operations use this font manager
    pub fn font_manager(mut self, font_manager: FontManager) -> Self {
        self.font_manager = Some(font_manager);
        self
    }
    
    /// Build the canvas
    pub fn build(self) -> Result<Canvas> {
        let width = self.width.ok_or_else(|| {
            CloveError::InvalidState("Canvas width not set".to_string())
        })?;
        let height = self.height.ok_or_else(|| {
            CloveError::InvalidState("Canvas height not set".to_string())
        })?;
        
        validate_dimensions(width, height)?;
        
        let mut canvas = Canvas {
            backend: TinySkiaBackend::new(width, height),
            layers: LayerManager::new(),
            state_stack: Vec::new(),
            font_manager: self.font_manager,  // Store font manager in canvas
            width,
            height,
        };
        
        if let Some(bg) = self.background {
            canvas.clear(bg);
        }
        
        Ok(canvas)
    }
}

impl Default for CanvasBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

**Tasks:**
- [ ] Implement CanvasBuilder
- [ ] Add size method
- [ ] Add background method
- [ ] Implement build method
- [ ] Add validation
- [ ] Write tests
- [ ] Add examples

---

### 3. Backend Integration

**Files:**
- `src/backend/mod.rs`
- `src/backend/tiny_skia_backend.rs`
- `src/backend/renderer.rs`

#### Backend Trait

```rust
pub trait RenderBackend {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn get_pixmap(&self) -> &tiny_skia::Pixmap;
    fn get_pixmap_mut(&mut self) -> &mut tiny_skia::Pixmap;
}
```

#### TinySkia Backend

```rust
use tiny_skia::{Pixmap, Paint, Color as SkiaColor};

pub struct TinySkiaBackend {
    pixmap: Pixmap,
}

impl TinySkiaBackend {
    pub fn new(width: u32, height: u32) -> Self {
        let pixmap = Pixmap::new(width, height)
            .expect("Failed to create pixmap");
        Self { pixmap }
    }
    
    fn convert_color(color: &Color) -> SkiaColor {
        match color {
            Color::Rgba(rgba) => SkiaColor::from_rgba8(
                rgba.r,
                rgba.g,
                rgba.b,
                rgba.a,
            ),
            Color::Hsla(hsla) => {
                let rgba = hsla.to_rgba();
                SkiaColor::from_rgba8(rgba.r, rgba.g, rgba.b, rgba.a)
            },
            Color::Named(name) => {
                // Look up named color and convert
                todo!()
            },
            _ => unimplemented!("Gradients/patterns in future phases"),
        }
    }
    
    pub fn encode_png(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let encoder = png::Encoder::new(
            &mut buffer,
            self.pixmap.width(),
            self.pixmap.height(),
        );
        // Configure and encode
        // ...
        Ok(buffer)
    }
    
    pub fn encode_jpeg(&self, quality: u8) -> Result<Vec<u8>> {
        // JPEG encoding
        // ...
        Ok(vec![])
    }
    
    pub fn encode_webp(&self) -> Result<Vec<u8>> {
        // WebP encoding
        // ...
        Ok(vec![])
    }
}

impl RenderBackend for TinySkiaBackend {
    fn width(&self) -> u32 {
        self.pixmap.width()
    }
    
    fn height(&self) -> u32 {
        self.pixmap.height()
    }
    
    fn clear(&mut self, color: Color) {
        let skia_color = Self::convert_color(&color);
        self.pixmap.fill(skia_color);
    }
    
    fn get_pixmap(&self) -> &Pixmap {
        &self.pixmap
    }
    
    fn get_pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }
}
```

**Tasks:**
- [ ] Define RenderBackend trait
- [ ] Implement TinySkiaBackend
- [ ] Add color conversion
- [ ] Implement PNG encoding
- [ ] Implement JPEG encoding
- [ ] Implement WebP encoding
- [ ] Write tests
- [ ] Add examples

---

### 4. State Management

```rust
#[derive(Clone)]
pub struct CanvasState {
    // Future: transforms, clip regions, etc.
    _private: (),
}

impl CanvasState {
    pub fn from_canvas(canvas: &Canvas) -> Self {
        Self {
            _private: (),
        }
    }
    
    pub fn apply_to_canvas(&self, _canvas: &mut Canvas) {
        // Apply saved state
    }
}
```

**Tasks:**
- [ ] Implement CanvasState
- [ ] Add state capture
- [ ] Add state restore
- [ ] Write tests
- [ ] Add examples

---

### 5. Image Format Support

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}

impl ImageFormat {
    pub fn from_path(path: &str) -> Result<Self> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| {
                CloveError::UnsupportedFormat(
                    "No file extension".to_string()
                )
            })?;
        
        match ext.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(CloveError::UnsupportedFormat(
                format!("Unsupported extension: {}", ext)
            )),
        }
    }
}
```

**Tasks:**
- [ ] Implement ImageFormat enum
- [ ] Add from_path method
- [ ] Add mime type detection
- [ ] Write tests

---

## Testing Requirements

### Unit Tests

```rust
#[test]
fn test_canvas_creation() {
    let canvas = Canvas::new(800, 600);
    assert!(canvas.is_ok());
    let canvas = canvas.unwrap();
    assert_eq!(canvas.width(), 800);
    assert_eq!(canvas.height(), 600);
}

#[test]
fn test_canvas_builder() {
    let canvas = Canvas::builder()
        .size(1024, 768)
        .background(Color::WHITE)
        .build();
    assert!(canvas.is_ok());
}

#[test]
fn test_invalid_dimensions() {
    let canvas = Canvas::new(0, 0);
    assert!(canvas.is_err());
    
    let canvas = Canvas::new(20000, 20000);
    assert!(canvas.is_err());
}

#[test]
fn test_state_save_restore() {
    let mut canvas = Canvas::new(100, 100).unwrap();
    canvas.save_state();
    canvas.clear(Color::RED);
    canvas.restore_state().unwrap();
}

#[test]
fn test_state_restore_without_save() {
    let mut canvas = Canvas::new(100, 100).unwrap();
    let result = canvas.restore_state();
    assert!(result.is_err());
}
```

### Integration Tests

```rust
#[test]
fn test_save_png() -> Result<()> {
    let mut canvas = Canvas::new(100, 100)?;
    canvas.clear(Color::BLUE);
    canvas.save("/tmp/test.png")?;
    
    // Verify file exists and is valid PNG
    assert!(std::path::Path::new("/tmp/test.png").exists());
    Ok(())
}

#[test]
fn test_save_jpeg_quality() -> Result<()> {
    let mut canvas = Canvas::new(200, 200)?;
    canvas.clear(Color::GREEN);
    canvas.save_with_quality("/tmp/test.jpg", 95)?;
    
    assert!(std::path::Path::new("/tmp/test.jpg").exists());
    Ok(())
}
```

### Performance Tests

```rust
use std::time::Instant;

#[test]
fn test_canvas_creation_performance() {
    let start = Instant::now();
    let _canvas = Canvas::new(800, 600).unwrap();
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 10, 
            "Canvas creation took {}ms, expected <10ms", 
            duration.as_millis());
}
```

---

## Definition of Done

### Code Complete

- [ ] Canvas struct implemented
- [ ] CanvasBuilder implemented
- [ ] TinySkiaBackend implemented
- [ ] State management working
- [ ] Save/load operations working
- [ ] All encoding formats working

### Testing Complete

- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Performance tests passing
- [ ] Code coverage >90%

### Documentation Complete

- [ ] All public APIs documented
- [ ] Module documentation written
- [ ] 3+ examples added
- [ ] Doctests passing

### Integration Ready

- [ ] Builds without warnings
- [ ] Clippy satisfied
- [ ] Ready for Phase 3

---

## Examples

### Example 1: Basic Canvas

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::hex("#F5F5F5")?)
        .build()?;
    
    canvas.save("canvas.png")?;
    Ok(())
}
```

### Example 2: State Management

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::new(400, 300)?;
    
    canvas.clear(Color::WHITE);
    
    canvas.save_state();
    canvas.clear(Color::BLUE);
    canvas.save("blue.png")?;
    
    canvas.restore_state()?;
    canvas.save("white.png")?;
    
    Ok(())
}
```

### Example 3: Multiple Formats

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::new(1920, 1080)?;
    canvas.clear(Color::hex("#667EEA")?);   
    canvas.save("output.png")?;
    canvas.save_with_quality("output.jpg", 95)?;
    canvas.save("output.webp")?;
    
    Ok(())
}
```

---

## Related Documents

- [v0.1.0 Overview](./overview.md)
- [Phase 1: Foundation](./phase-1-foundation.md)
- [Phase 3: Shapes](./phase-3-shapes.md)
- [Architecture](../../architecture.md)

---

**Phase:** 2/6  
**Status:** Planning  
**Duration:** 2 weeks  
**Last Updated:** 2025-11-27
