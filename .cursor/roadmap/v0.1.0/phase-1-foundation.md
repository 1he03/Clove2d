# Phase 1: Foundation

## @context: v0.1.0 Phase 1
## @timeline: Week 1-2 (2 weeks)
## @priority: Critical
## @status: Planning
## @dependencies: None
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

Phase 1 establishes the foundational types and utilities needed for the entire Clove2d library. This phase focuses on creating robust, well-tested primitives that all other systems will depend on.

### Key Focus Areas

1. **Error Handling** - Comprehensive error types and Result patterns
2. **Geometry Primitives** - Point, Rect, Size, Bounds
3. **Color System** - RGBA, HSLA, Hex, Named colors, Gradients
4. **Utilities** - Validation, math helpers, caching

---

## Objectives

### Primary Goals

- ✅ Create type-safe, ergonomic foundation types
- ✅ Establish error handling patterns
- ✅ Implement comprehensive color system
- ✅ Build reusable utility functions

### Success Metrics

- [ ] 100% of foundation types tested
- [ ] Zero unsafe code in this phase
- [ ] All public APIs documented
- [ ] Examples for each major type

---

## Deliverables

### 1. Error Handling System

**Files:**
- `src/error.rs`

**Types:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum CloveError {
    // Canvas errors
    #[error("Invalid canvas dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    
    #[error("Invalid canvas state: {0}")]
    InvalidState(String),
    
    // Color errors
    #[error("Invalid hex color: {0}")]
    InvalidHexColor(String),
    
    #[error("Invalid color value for {component}: {value}")]
    InvalidColorValue { component: String, value: f32 },
    
    // Font errors
    #[error("Font not found: {0}")]
    FontNotFound(String),
    
    #[error("Font load error: {0}")]
    FontLoadError(String),
    
    #[error("Invalid font family: {0}")]
    InvalidFontFamily(String),
    
    // Image errors
    #[error("Image load error: {0}")]
    ImageLoadError(String),
    
    #[error("Image encode error: {0}")]
    ImageEncodeError(String),
    
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),
    
    // Layer errors
    #[error("Layer not found: {0}")]
    LayerNotFound(String),
    
    #[error("Invalid layer index: {0}")]
    InvalidLayerIndex(usize),
    
    #[error("Layer limit exceeded (max: 100)")]
    LayerLimitExceeded,
    
    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    // Network errors (async feature)
    #[cfg(feature = "async")]
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub type Result<T> = std::result::Result<T, CloveError>;
```

**Tasks:**
- [ ] Define all error variants
- [ ] Implement Display and Error traits
- [ ] Add context to error messages
- [ ] Write error conversion From impls
- [ ] Document each error variant
- [ ] Add error examples

---

### 2. Geometry Primitives

**Files:**
- `src/geometry/mod.rs`
- `src/geometry/point.rs`
- `src/geometry/rect.rs`
- `src/geometry/size.rs`
- `src/geometry/bounds.rs`

#### 2.1 Point

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };
    
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn midpoint(&self, other: Point) -> Point {
        Point::new(
            (self.x + other.x) / 2.0,
            (self.y + other.y) / 2.0,
        )
    }
}
```

**Tasks:**
- [ ] Implement Point struct
- [ ] Add arithmetic operations (Add, Sub, Mul, Div)
- [ ] Add distance and midpoint methods
- [ ] Implement From<(f32, f32)>
- [ ] Write comprehensive tests
- [ ] Add usage examples

#### 2.2 Size

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
    
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }
}
```

**Tasks:**
- [ ] Implement Size struct
- [ ] Add validation (width/height > 0)
- [ ] Add area and aspect ratio methods
- [ ] Implement From<(f32, f32)>
- [ ] Write tests
- [ ] Add examples

#### 2.3 Rect

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs();
        let height = (p1.y - p2.y).abs();
        Self::new(x, y, width, height)
    }
    
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
    
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);
        Rect::new(x, y, right - x, bottom - y)
    }
}
```

**Tasks:**
- [ ] Implement Rect struct
- [ ] Add constructors (new, from_points, etc.)
- [ ] Implement contains, intersects, union
- [ ] Add center, corners methods
- [ ] Write comprehensive tests
- [ ] Add examples

---

### 3. Color System

**Files:**
- `src/color/mod.rs`
- `src/color/rgba.rs`
- `src/color/hsla.rs`
- `src/color/named.rs`
- `src/color/gradient.rs`
- `src/color/pattern.rs`
- `src/color/conversions.rs`

#### 3.1 RGBA Color

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
    
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}
```

**Tasks:**
- [ ] Implement Rgba struct
- [ ] Add constructors (new, rgb, from_hex)
- [ ] Implement color space conversions
- [ ] Add manipulation methods
- [ ] Write tests
- [ ] Add examples

#### 3.2 HSLA Color

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub h: f32, // 0-360
    pub s: f32, // 0-1
    pub l: f32, // 0-1
    pub a: f32, // 0-1
}

impl Hsla {
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Result<Self> {
        if !(0.0..=360.0).contains(&h) {
            return Err(CloveError::InvalidColorValue {
                component: "hue".to_string(),
                value: h,
            });
        }
        // ... validate s, l, a
        Ok(Self { h, s, l, a })
    }
    
    pub fn to_rgba(&self) -> Rgba {
        // HSL to RGB conversion algorithm
        // ...
    }
}
```

**Tasks:**
- [ ] Implement Hsla struct
- [ ] Add validation
- [ ] Implement HSL to RGB conversion
- [ ] Add manipulation methods
- [ ] Write tests
- [ ] Add examples

#### 3.3 Color Enum

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgba(Rgba),
    Hsla(Hsla),
    Named(&'static str),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
}

impl Color {
    // Constructors
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::Rgba(Rgba::new(r, g, b, a))
    }
    
    pub fn hex(s: &str) -> Result<Self> {
        Rgba::from_hex(s).map(Color::Rgba)
    }
    
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Result<Self> {
        Hsla::new(h, s, l, a).map(Color::Hsla)
    }
    
    // Named colors
    pub const RED: Color = Color::Rgba(Rgba { r: 255, g: 0, b: 0, a: 255 });
    pub const BLUE: Color = Color::Rgba(Rgba { r: 0, g: 0, b: 255, a: 255 });
    // ... 140+ web colors
    
    // Manipulation
    pub fn lighten(&self, amount: f32) -> Self { /* ... */ }
    pub fn darken(&self, amount: f32) -> Self { /* ... */ }
    pub fn saturate(&self, amount: f32) -> Self { /* ... */ }
}
```

**Tasks:**
- [ ] Implement Color enum
- [ ] Add all constructors
- [ ] Define 140+ named colors
- [ ] Implement manipulation methods
- [ ] Add gradient builders
- [ ] Write tests
- [ ] Add examples

#### 3.4 Gradients

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    start: Point,
    end: Point,
    stops: Vec<GradientStop>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    position: f32, // 0.0 to 1.0
    color: Rgba,
}

pub struct LinearGradientBuilder {
    start: Option<Point>,
    end: Option<Point>,
    stops: Vec<GradientStop>,
}

impl LinearGradientBuilder {
    pub fn start(mut self, x: f32, y: f32) -> Self {
        self.start = Some(Point::new(x, y));
        self
    }
    
    pub fn end(mut self, x: f32, y: f32) -> Self {
        self.end = Some(Point::new(x, y));
        self
    }
    
    pub fn add_stop(mut self, position: f32, color: Color) -> Self {
        // Convert color to Rgba and add stop
        self
    }
    
    pub fn build(self) -> Color {
        Color::LinearGradient(LinearGradient {
            start: self.start.unwrap(),
            end: self.end.unwrap(),
            stops: self.stops,
        })
    }
}
```

**Tasks:**
- [ ] Implement LinearGradient
- [ ] Implement RadialGradient
- [ ] Add builder patterns
- [ ] Validate stop positions
- [ ] Write tests
- [ ] Add examples

---

### 4. Utilities

**Files:**
- `src/utils/mod.rs`
- `src/utils/validation.rs`
- `src/utils/math.rs`
- `src/utils/cache.rs`

#### 4.1 Validation

```rust
pub fn validate_dimensions(width: u32, height: u32) -> Result<()> {
    if width == 0 || height == 0 {
        return Err(CloveError::InvalidDimensions { width, height });
    }
    if width > 16384 || height > 16384 {
        return Err(CloveError::InvalidDimensions { width, height });
    }
    Ok(())
}

pub fn validate_opacity(opacity: f32) -> Result<f32> {
    if !(0.0..=1.0).contains(&opacity) {
        return Err(CloveError::InvalidColorValue {
            component: "opacity".to_string(),
            value: opacity,
        });
    }
    Ok(opacity)
}
```

**Tasks:**
- [ ] Implement validation functions
- [ ] Add range checking helpers
- [ ] Write tests
- [ ] Document validation rules

#### 4.2 Math Helpers

```rust
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min { min }
    else if value > max { max }
    else { value }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut a = angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }
    a
}
```

**Tasks:**
- [ ] Implement math utilities
- [ ] Add interpolation functions
- [ ] Write tests
- [ ] Add examples

---

## Testing Requirements

### Unit Tests

**Coverage target: 100% for Phase 1**

**Error handling:**
```rust
#[test]
fn test_invalid_dimensions_error() {
    let err = CloveError::InvalidDimensions { width: 0, height: 600 };
    assert_eq!(err.to_string(), "Invalid canvas dimensions: 0x600");
}
```

**Geometry:**
```rust
#[test]
fn test_point_distance() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    assert_eq!(p1.distance_to(p2), 5.0);
}

#[test]
fn test_rect_contains() {
    let rect = Rect::new(10.0, 10.0, 100.0, 100.0);
    assert!(rect.contains(Point::new(50.0, 50.0)));
    assert!(!rect.contains(Point::new(5.0, 5.0)));
}
```

**Color:**
```rust
#[test]
fn test_hex_parsing() {
    let color = Color::hex("#FF5733").unwrap();
    if let Color::Rgba(rgba) = color {
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 87);
        assert_eq!(rgba.b, 51);
    }
}

#[test]
fn test_hsla_to_rgba() {
    let hsla = Hsla::new(0.0, 1.0, 0.5, 1.0).unwrap();
    let rgba = hsla.to_rgba();
    assert_eq!(rgba.r, 255);
    assert_eq!(rgba.g, 0);
    assert_eq!(rgba.b, 0);
}
```

### Property Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_color_roundtrip(
        r in 0u8..=255,
        g in 0u8..=255,
        b in 0u8..=255,
        a in 0u8..=255
    ) {
        let rgba = Rgba::new(r, g, b, a);
        let hsla = rgba.to_hsla();
        let back = hsla.to_rgba();
        // Allow small rounding errors
        assert!((rgba.r as i16 - back.r as i16).abs() <= 1);
    }
}
```

---

## Definition of Done

### Code Complete

- [ ] All types implemented
- [ ] All methods implemented
- [ ] No TODO comments
- [ ] No commented-out code
- [ ] All clippy warnings resolved

### Testing Complete

- [ ] All unit tests passing
- [ ] Property tests passing
- [ ] Code coverage >95%
- [ ] Edge cases tested
- [ ] Error cases tested

### Documentation Complete

- [ ] All public APIs documented
- [ ] Module documentation written
- [ ] Examples added
- [ ] Doctests passing

### Review Complete

- [ ] Self-review completed
- [ ] Code formatted (rustfmt)
- [ ] No warnings in CI
- [ ] Ready for Phase 2

---

## Related Documents

- [v0.1.0 Overview](./overview.md)
- [Phase 2: Canvas Core](./phase-2-canvas-core.md)
- [Architecture](../../architecture.md)
- [Conventions](../../conventions.md)

---

**Phase:** 1/6  
**Status:** Planning  
**Duration:** 2 weeks  
**Last Updated:** 2025-11-27
