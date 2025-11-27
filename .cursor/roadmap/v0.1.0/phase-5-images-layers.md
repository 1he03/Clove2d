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
    opacity: f32,
    blend_mode: BlendMode,
    visible: bool,
    transform: Transform,
}

pub struct LayerManager {
    layers: Vec<Layer>,
    active: Option<LayerId>,
}

impl LayerManager {
    pub fn create(&mut self, name: &str) -> LayerId {
        let id = LayerId::new();
        let layer = Layer::new(id, name);
        self.layers.push(layer);
        id
    }
    
    pub fn merge_all(&self) -> Result<RgbaImage> {
        // Merge layers with blend modes
        todo!()
    }
}
```

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
