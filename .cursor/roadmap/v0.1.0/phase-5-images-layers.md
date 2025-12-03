# Phase 5: Images & Layers

## @context: v0.1.0 Phase 5
## @timeline: Week 9 (1 week)
## @priority: High
## @status: Planning
## @dependencies: Phase 4 (Text System)
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Overview](#overview)
2. [Image Loading](#image-loading)
3. [Layer System](#layer-system)
4. [Blend Modes](#blend-modes)
5. [Testing Requirements](#testing-requirements)

---

## Overview

Phase 5 adds image loading and layer management for complex compositions.

---

## Image Loading

**File:** `src/image/loader.rs`

```rust
use image::{DynamicImage, RgbaImage};

pub struct ImageLoader;

impl ImageLoader {
    pub fn load_from_path(path: &str) -> Result<RgbaImage> {
        let img = image::open(path)
            .map_err(|e| CloveError::ImageLoadError(e.to_string()))?;
        Ok(img.to_rgba8())
    }
    
    #[cfg(feature = "async")]
    pub async fn load_from_url(url: &str) -> Result<RgbaImage> {
        let bytes = reqwest::get(url)
            .await
            .map_err(|e| CloveError::NetworkError(e.to_string()))?
            .bytes()
            .await
            .map_err(|e| CloveError::NetworkError(e.to_string()))?;
        
        Self::load_from_buffer(&bytes)
    }
    
    pub fn load_from_buffer(buffer: &[u8]) -> Result<RgbaImage> {
        let img = image::load_from_memory(buffer)
            .map_err(|e| CloveError::ImageLoadError(e.to_string()))?;
        Ok(img.to_rgba8())
    }
}
```

---

## Layer System

**File:** `src/layer/mod.rs`

```rust
pub struct Layer {
    id: LayerId,
    name: String,
    content: RgbaImage,
    width: Option<u32>,      // Optional: if None, uses canvas width
    height: Option<u32>,    // Optional: if None, uses canvas height
    opacity: f32,
    blend_mode: BlendMode,
    visible: bool,
    transform: Transform,
}

pub struct LayerManager {
    layers: Vec<Layer>,
    active: Option<LayerId>,
    base_width: u32,
    base_height: u32,
}

impl LayerManager {
    /// Create layer with canvas dimensions
    pub fn create(&mut self, name: &str) -> LayerId {
        let id = LayerId::new();
        let layer = Layer::new(id, name, self.base_width, self.base_height);
        self.layers.push(layer);
        id
    }
    
    /// Create layer with custom dimensions
    pub fn create_with_size(&mut self, name: &str, width: u32, height: u32) -> LayerId {
        let id = LayerId::new();
        let mut layer = Layer::new(id, name, width, height);
        layer.width = Some(width);
        layer.height = Some(height);
        layer.scale_content(); // Scale content to match dimensions
        self.layers.push(layer);
        id
    }
    
    pub fn merge_all(&self) -> Result<RgbaImage> {
        // Merge layers with blend modes using parallel processing
        self.layers.par_iter()
            .filter(|layer| layer.visible)
            .map(|layer| layer.render())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .reduce(|a, b| blend(a, b))
    }
}

impl Layer {
    /// Set layer width. Scales content if width differs.
    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = Some(width);
        self.scale_content();
        self
    }
    
    /// Set layer height. Scales content if height differs.
    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = Some(height);
        self.scale_content();
        self
    }
    
    /// Set layer dimensions. Scales content if dimensions differ.
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self.scale_content();
        self
    }
    
    /// Get effective dimensions (layer dimensions or canvas dimensions)
    pub fn dimensions(&self) -> (u32, u32) {
        (
            self.width.unwrap_or(self.base_width),
            self.height.unwrap_or(self.base_height)
        )
    }
    
    fn scale_content(&mut self) {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            if w != self.content.width() || h != self.content.height() {
                // Resize content using image scaling
                self.content = resize_image(&self.content, w, h);
            }
        }
    }
}
```

**Layer Dimensions Behavior:**
- **Default**: Layer uses canvas dimensions (width/height = None)
- **Custom Dimensions**: When set, layer content is automatically scaled
- **Scaling**: Preserves aspect ratio, uses high-quality resampling
- **Performance**: Scaling happens once when dimensions are set

---

## Virtual Layer System

**File:** `src/layer/virtual.rs`

```rust
pub struct VirtualLayer {
    id: LayerId,
    name: String,
    operations: Vec<LayerOperation>,
    opacity: f32,
    blend_mode: BlendMode,
    visible: bool,
}

pub enum LayerOperation {
    DrawShape(Shape, Style),
    DrawText(Text, TextStyle),
    DrawImage(Image, ImageStyle),
    ApplyFilter(Filter),
    Transform(Transform),
}

pub struct VirtualLayerManager {
    virtual_layers: Vec<VirtualLayer>,
    base_canvas: Canvas,
}

impl VirtualLayerManager {
    /// Create a virtual layer that stores operations instead of pixels
    pub fn create_virtual_layer(&mut self, name: &str) -> &mut VirtualLayer {
        let id = LayerId::new();
        let layer = VirtualLayer::new(id, name);
        self.virtual_layers.push(layer);
        self.virtual_layers.last_mut().unwrap()
    }
    
    /// Begin virtual composition - operations are stored, not rendered
    pub fn begin_virtual_composition(&mut self) -> VirtualComposition {
        VirtualComposition::new()
    }
    
    /// End virtual composition and render all operations to a single image
    pub fn end_virtual_composition(&mut self, composition: VirtualComposition) -> Result<RgbaImage> {
        // Process all stored operations in one pass using parallel processing
        composition.operations.par_iter()
            .map(|op| self.render_operation(op))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .reduce(|a, b| blend(a, b))
    }
}

impl VirtualLayer {
    /// Add operation to virtual layer
    pub fn add_operation(&mut self, operation: LayerOperation) -> &mut Self {
        self.operations.push(operation);
        self
    }
    
    /// Render all operations to image
    pub fn render(&self) -> Result<RgbaImage> {
        // Single-pass rendering of all operations
        // Much more efficient than rendering multiple layers
    }
}
```

**Virtual Layer Benefits:**
- **Memory**: No intermediate pixel buffers
- **Performance**: Single-pass rendering instead of multi-layer merge
- **Scalability**: Better with many layers
- **Flexibility**: Operations can be optimized before rendering

**Use Cases:**
- Complex scenes with many layers
- Batch operations
- Memory-constrained environments
- High-performance rendering

---

## Blend Modes

```rust
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
}
```

---

**Phase:** 5/6  
**Status:** Planning  
**Duration:** 1 week  
**Last Updated:** 2025-11-27
