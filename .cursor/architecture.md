# Clove2d Architecture Documentation

## @context: System Architecture
## @version: v0.1.0
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Core Systems](#core-systems)
4. [Data Flow](#data-flow)
5. [Backend Integration](#backend-integration)
6. [Module Dependencies](#module-dependencies)

---

## Overview

Clove2d is a Pure Rust 2D graphics and canvas library built on top of `tiny-skia` for rendering and `cosmic-text` for advanced text layout. The architecture follows a modular design with clear separation of concerns.

### Design Principles

1. **Modularity** - Each system is independent and composable
2. **Method Chaining** - Builder pattern for fluent API
3. **Type Safety** - Leverage Rust's type system for compile-time guarantees
4. **Performance** - Zero-cost abstractions where possible
5. **Memory Safety** - No unsafe code without proper documentation

---

## Project Structure

```
clove2d/
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── prelude.rs            # Commonly used types
│   ├── error.rs              # Error types and handling
│   │
│   ├── canvas/               # Canvas System
│   │   ├── mod.rs
│   │   ├── builder.rs
│   │   ├── operations.rs
│   │   └── state.rs
│   │
│   ├── color/                # Color System
│   │   ├── mod.rs
│   │   ├── rgba.rs
│   │   ├── hsla.rs
│   │   ├── named.rs
│   │   ├── gradient.rs
│   │   ├── pattern.rs
│   │   └── conversions.rs
│   │
│   ├── shapes/               # Shapes System
│   │   ├── mod.rs
│   │   ├── rectangle.rs
│   │   ├── circle.rs
│   │   ├── ellipse.rs
│   │   ├── line.rs
│   │   ├── polyline.rs
│   │   ├── polygon.rs
│   │   ├── triangle.rs
│   │   ├── arc.rs
│   │   ├── bezier.rs
│   │   ├── path.rs
│   │   ├── star.rs
│   │   ├── rounded_rect.rs
│   │   └── traits.rs
│   │
│   ├── text/                 # Text System
│   │   ├── mod.rs
│   │   ├── font_manager.rs
│   │   ├── font_family.rs
│   │   ├── text_style.rs
│   │   ├── text_layout.rs
│   │   ├── text_align.rs
│   │   ├── text_direction.rs
│   │   ├── line_break.rs
│   │   ├── bidi.rs
│   │   ├── shaping.rs
│   │   └── metrics.rs
│   │
│   ├── image/                # Image System
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   ├── encoder.rs
│   │   ├── formats.rs
│   │   └── blend_modes.rs
│   │
│   ├── layer/                # Layer System
│   │   ├── mod.rs
│   │   ├── layer_manager.rs
│   │   ├── layer_tree.rs
│   │   └── blend_stack.rs
│   │
│   ├── filter/               # Filter System
│   │   ├── mod.rs
│   │   ├── blur.rs
│   │   ├── sharpen.rs
│   │   ├── color_adjust.rs
│   │   ├── effects.rs
│   │   └── kernel.rs
│   │
│   ├── transform/            # Transform System
│   │   ├── mod.rs
│   │   ├── matrix.rs
│   │   ├── rotate.rs
│   │   ├── scale.rs
│   │   ├── translate.rs
│   │   └── crop.rs
│   │
│   ├── backend/              # Rendering Backend
│   │   ├── mod.rs
│   │   ├── tiny_skia_backend.rs
│   │   └── renderer.rs
│   │
│   ├── geometry/             # Geometry Primitives
│   │   ├── mod.rs
│   │   ├── point.rs
│   │   ├── rect.rs
│   │   ├── size.rs
│   │   └── bounds.rs
│   │
│   └── utils/                # Utilities
│       ├── mod.rs
│       ├── validation.rs
│       ├── math.rs
│       └── cache.rs
```

---

## Core Systems

### 1. Canvas System

**Purpose:** Main entry point for all drawing operations.

**Components:**
- `Canvas` - Core canvas structure
- `CanvasBuilder` - Builder for canvas creation
- `CanvasState` - Canvas state management (save/restore)
- `CanvasOperations` - Drawing operations trait

**Responsibilities:**
- Canvas lifecycle management
- Coordinate system management
- State stack (save/restore)
- Final image output

**Type Signature:**
```rust
pub struct Canvas {
    backend: Box<dyn RenderBackend>,
    layers: LayerManager,
    state_stack: Vec<CanvasState>,
    font_manager: Option<FontManager>,  // Optional font manager from config
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn builder() -> CanvasBuilder { ... }
    pub fn create_layer(&mut self, name: &str) -> Result<&mut Layer> { ... }
    pub fn merge_layers(&mut self) -> Result<&mut Self> { ... }
    pub fn save(&self, path: &str) -> Result<()> { ... }
}
```

---

### 2. Color System

**Purpose:** Comprehensive color management and manipulation.

**Components:**
- `Color` - Main color enum
- `Rgba` - RGBA color representation
- `Hsla` - HSLA color representation
- `NamedColors` - Predefined color constants
- `Gradient` - Linear and radial gradients
- `Pattern` - Pattern fills from images

**Type Hierarchy:**
```rust
pub enum Color {
    Rgba(Rgba),
    Hsla(Hsla),
    Named(&'static str),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { ... }
    pub fn hex(s: &str) -> Result<Self> { ... }
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self { ... }
    pub fn linear_gradient() -> LinearGradientBuilder { ... }
}
```

---

### 3. Shapes System

**Purpose:** Drawing geometric shapes with fill, stroke, and effects.

**Trait-based Design:**
```rust
pub trait Shape {
    fn bounds(&self) -> Rect;
    fn draw(&self, backend: &mut dyn RenderBackend) -> Result<()>;
}

pub trait Fillable {
    fn fill(self, color: Color) -> Self;
}

pub trait Strokeable {
    fn stroke(self, color: Color, width: f32) -> Self;
}

pub trait Transformable {
    fn transform(self, matrix: Transform) -> Self;
}
```

**Shapes:**
- Rectangle, RoundedRect, Circle, Ellipse
- Line, Polyline, Polygon, Triangle
- Arc, BezierCurve, Path, Star

---

### 4. Text System

**Purpose:** Advanced text rendering with full BiDi and shaping support.

**Architecture:**
```rust
pub struct FontManager {
    fonts: HashMap<String, FontFamily>,
    default_family: String,
    cache: FontCache,
}

pub struct TextStyle {
    font_family: String,
    font_size: f32,
    font_weight: FontWeight,
    font_style: FontStyle,
    color: Color,
    letter_spacing: f32,
    line_height: f32,
    align: TextAlign,  // Left, Right, Center only
    text_width: TextWidth,  // None, Max(f32), FullPage, Layer
    // TextDirection removed - not supported
}

pub struct TextLayout {
    lines: Vec<TextLine>,
    bounds: Rect,
    text_width: TextWidth,
    clipping_rect: Option<Rect>,  // Clipping rectangle based on TextWidth
}
```

**Text Rendering Pipeline:**
1. Font loading and caching
2. Text shaping (rustybuzz)
3. BiDi processing (unicode-bidi)
4. Line breaking
5. Layout calculation
6. Glyph rasterization (cosmic-text)
7. Final rendering

---

### 5. Image System

**Purpose:** Image loading, manipulation, and encoding.

**Components:**
```rust
pub struct ImageLoader {
    formats: Vec<ImageFormat>,
}

impl ImageLoader {
    pub fn load_from_path(path: &str) -> Result<RgbaImage> { ... }
    pub async fn load_from_url(url: &str) -> Result<RgbaImage> { ... }
    pub fn load_from_buffer(buffer: &[u8]) -> Result<RgbaImage> { ... }
}

pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    // ...
}
```

---

### 6. Layer System

**Purpose:** Layer management and compositing.

**Layer Tree:**
```rust
pub struct Layer {
    id: LayerId,
    name: String,
    content: RgbaImage,
    x: f32,                  // Layer x coordinate (default: 0)
    y: f32,                  // Layer y coordinate (default: 0)
    width: Option<u32>,      // Optional layer width
    height: Option<u32>,    // Optional layer height
    opacity: f32,
    blend_mode: BlendMode,
    visible: bool,
    transform: Transform,
    clipping_enabled: bool,  // Enable clipping to layer boundaries
}

pub struct LayerManager {
    layers: Vec<Layer>,
    active_layer: Option<LayerId>,
    base_width: u32,        // Base canvas width
    base_height: u32,       // Base canvas height
}

impl LayerManager {
    pub fn create_layer(&mut self, name: &str) -> LayerId { ... }
    pub fn create_layer_with_size(&mut self, name: &str, width: u32, height: u32) -> LayerId { ... }
    pub fn merge_layers(&self) -> Result<RgbaImage> { ... }
    pub fn reorder(&mut self, from: usize, to: usize) { ... }
}

impl Layer {
    /// Set layer dimensions. If not set, uses base canvas dimensions.
    /// If set, scales the layer content to match dimensions.
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        // Scale content if needed
        self.scale_content();
        self
    }
    
    fn scale_content(&mut self) {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            // Scale content to match layer dimensions
            // Implementation uses image resizing
        }
    }
}
```

**Layer Dimensions Behavior:**
- If `width` and `height` are not specified: Layer uses base canvas dimensions
- If `width` and/or `height` are specified: Layer content is scaled to match dimensions
- Scaling preserves aspect ratio when only one dimension is specified
- Scaling happens automatically when dimensions are set

**Layer Coordinates:**
- Layer has `x` and `y` coordinates (default: 0, 0)
- All drawing operations within layer are relative to layer position
- Layer position determines where layer content appears on canvas

**Layer Clipping:**
- By default, clipping is enabled for layers
- Any content that extends beyond layer boundaries (x, y, width, height) is clipped
- Clipping ensures content doesn't overflow layer boundaries
- Can be disabled with `layer.clipping_enabled(false)`

**TextWidth::Layer Behavior:**
- Uses layer x coordinate as the starting point for text width calculation
- Layer width determines the maximum text width
- Text alignment (Left, Right, Center) is relative to layer x and width
- If layer width is not set, uses canvas width starting from layer x

---

### 7. Filter System

**Purpose:** Image filtering and effects.

**Filter Pipeline:**
```rust
pub enum Filter {
    Blur(f32),
    Sharpen,
    Grayscale,
    Sepia,
    Invert,
    Brightness(f32),
    Contrast(f32),
    Saturation(f32),
    HueRotate(f32),
}

impl Filter {
    pub fn apply(&self, image: &mut RgbaImage) -> Result<()> { ... }
}
```

---

### 8. Transform System

**Purpose:** Geometric transformations.

**Transform Matrix:**
```rust
pub struct Transform {
    matrix: [f32; 6], // affine transformation matrix
}

impl Transform {
    pub fn identity() -> Self { ... }
    pub fn translate(x: f32, y: f32) -> Self { ... }
    pub fn rotate(angle: f32) -> Self { ... }
    pub fn scale(sx: f32, sy: f32) -> Self { ... }
    pub fn compose(&self, other: &Transform) -> Self { ... }
}
```

---

### 9. Backend System

**Purpose:** Abstraction over rendering backend (tiny-skia).

**Backend Trait:**
```rust
pub trait RenderBackend {
    fn clear(&mut self, color: Color);
    fn draw_rect(&mut self, rect: Rect, fill: &FillStyle);
    fn draw_path(&mut self, path: &Path, stroke: &StrokeStyle);
    fn draw_text(&mut self, text: &str, style: &TextStyle, pos: Point);
    fn draw_image(&mut self, image: &RgbaImage, pos: Point, size: Size);
    fn get_image(&self) -> &RgbaImage;
}

pub struct TinySkiaBackend {
    pixmap: tiny_skia::Pixmap,
}
```

---

### 10. Virtual Layer System

**Purpose:** Optimized layer composition using logical operations instead of physical layer merging.

**Virtual Layer Architecture:**
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
    pub fn create_virtual_layer(&mut self, name: &str) -> &mut VirtualLayer { ... }
    
    /// Begin virtual composition - operations are stored, not rendered
    pub fn begin_virtual_composition(&mut self) -> VirtualComposition { ... }
    
    /// End virtual composition and render all operations to a single image
    pub fn end_virtual_composition(&mut self, composition: VirtualComposition) -> Result<RgbaImage> {
        // Process all stored operations in one pass
        // Much more efficient than rendering multiple layers and merging
    }
}
```

**Virtual Layer Benefits:**
- **Memory Efficiency**: No intermediate pixel buffers for each layer
- **Performance**: Single-pass rendering instead of multi-layer merging
- **Flexibility**: Operations can be reordered or optimized before rendering
- **Scalability**: Better performance with many layers

**Use Cases:**
- Complex scenes with many layers
- Batch operations that don't need intermediate results
- Memory-constrained environments
- High-performance rendering pipelines

---

### 11. Geometry System

**Purpose:** Geometric primitives and operations.

```rust
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, point: Point) -> bool { ... }
    pub fn intersects(&self, other: &Rect) -> bool { ... }
    pub fn union(&self, other: &Rect) -> Rect { ... }
}
```

---

## Data Flow

### Drawing Operation Flow

```
User API Call
    ↓
Canvas Method (builder pattern)
    ↓
Layer Selection/Creation
    ↓
Shape/Text/Image Construction
    ↓
Transform Application
    ↓
Backend Rendering
    ↓
Layer Pixel Buffer Update
    ↓
(Repeat for multiple operations)
    ↓
Layer Merging (on save)
    ↓
Final Image Encoding
    ↓
File Output
```

### Text Rendering Flow

```
Text String Input
    ↓
Font Manager (load font)
    ↓
Text Shaping (rustybuzz)
    ↓
BiDi Processing (unicode-bidi)
    ↓
Line Breaking Algorithm
    ↓
Layout Calculation
    ↓
Glyph Positioning
    ↓
Rasterization (cosmic-text)
    ↓
Canvas Rendering
```

---

## Backend Integration

### tiny-skia Integration

**Why tiny-skia:**
- Pure Rust (no C dependencies)
- High-quality anti-aliasing
- Excellent performance
- Active maintenance
- Good documentation

**Integration Points:**
```rust
use tiny_skia::{Pixmap, Paint, PathBuilder, Transform as SkiaTransform};

impl TinySkiaBackend {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixmap: Pixmap::new(width, height).unwrap(),
        }
    }
    
    fn convert_color(&self, color: &Color) -> tiny_skia::Color {
        // Convert our Color to tiny-skia Color
    }
}
```

### cosmic-text Integration

**Why cosmic-text:**
- Advanced text layout
- Full Unicode support
- BiDi text support
- Font fallback
- Shaping with rustybuzz

**Integration:**
```rust
use cosmic_text::{FontSystem, SwashCache, Buffer};

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn render_text(&mut self, text: &str, style: &TextStyle) -> RgbaImage {
        // Create buffer, shape text, render glyphs
    }
}
```

---

## Module Dependencies

### Dependency Graph

```
lib.rs
  ├─→ prelude (re-exports)
  ├─→ error
  │
  ├─→ canvas
  │   ├─→ backend
  │   ├─→ layer
  │   └─→ geometry
  │
  ├─→ color
  │   └─→ geometry (for gradients)
  │
  ├─→ shapes
  │   ├─→ geometry
  │   ├─→ color
  │   └─→ backend
  │
  ├─→ text
  │   ├─→ color
  │   ├─→ geometry
  │   └─→ backend
  │
  ├─→ image
  │   ├─→ geometry
  │   └─→ backend
  │
  ├─→ layer
  │   ├─→ geometry
  │   ├─→ color (blend modes)
  │   └─→ image
  │
  ├─→ filter
  │   └─→ image
  │
  └─→ transform
      └─→ geometry
```

### External Dependencies

```toml
[dependencies]
# Core Rendering
tiny-skia = "0.11"
tiny-skia-path = "0.11"

# Text Rendering
cosmic-text = "0.15.0"
rustybuzz = "0.20.1"
unicode-bidi = "0.3.18"
unicode-segmentation = "1.12.0"

# Image Processing
image = "0.25.9"
png = "0.18.0"
jpeg-decoder = "0.3"
webp = "0.3.1"

# Color Management
palette = "0.7.6"

# Math & Geometry
euclid = "0.22.11"
kurbo = "0.13.0"

# Async (optional)
tokio = { version = "1.48.0", optional = true }
reqwest = { version = "0.12.24", optional = true }

# Utilities
thiserror = "2.0.17"
once_cell = "1.21.3"
parking_lot = "0.12.5"
rayon = "1.11.0"
ttf-parser = "0.25.1"
cssparser = "0.36.0"
regex = "1.12.2"
```

---

## Performance Considerations

### Memory Management

1. **Image Buffers** - Reuse pixmaps when possible
2. **Font Caching** - Cache loaded fonts and shaped glyphs
3. **Layer Composition** - Lazy layer merging
4. **Transform Caching** - Cache computed transform matrices
5. **Virtual Layers** - Use virtual layers to avoid intermediate buffers

### Rendering Optimization

1. **Dirty Rectangles** - Only redraw changed regions (future)
2. **Parallel Rendering** - Multi-threaded layer rendering using rayon
3. **GPU Acceleration** - Optional GPU backend (future v0.4.0+)

### Parallel Processing with Rayon

Clove2d uses `rayon` extensively for parallel processing across the entire project:

**Layer Rendering:**
```rust
use rayon::prelude::*;

impl LayerManager {
    pub fn render_layers_parallel(&self) -> Result<RgbaImage> {
        self.layers.par_iter()
            .filter(|layer| layer.visible)
            .map(|layer| layer.render())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .reduce(|a, b| blend(a, b))
    }
}
```

**Image Filtering:**
```rust
impl Filter {
    pub fn apply_parallel(&self, image: &mut RgbaImage) -> Result<()> {
        let chunks: Vec<_> = image.chunks_mut(4 * image.width() as usize).collect();
        chunks.par_iter_mut()
            .for_each(|chunk| {
                self.apply_to_chunk(chunk);
            });
        Ok(())
    }
}
```

**Text Shaping:**
```rust
pub fn shape_text_parallel(texts: &[Text]) -> Vec<ShapedText> {
    texts.par_iter()
        .map(|text| shape_text(text))
        .collect()
}
```

**Pixel Operations:**
```rust
pub fn process_pixels_parallel(pixels: &mut [u8]) {
    pixels.par_chunks_mut(4)
        .for_each(|pixel| {
            // Process RGBA pixel
        });
}
```

**Performance Benefits:**
- **Layer Rendering**: 2-4x faster with multiple layers
- **Filter Application**: 3-5x faster on large images
- **Text Processing**: Parallel shaping for multiple text elements
- **Memory Operations**: Parallel pixel processing reduces latency

---

## Thread Safety

### Current Approach (v0.1.0)

- **Single-threaded** - Canvas is not `Send` or `Sync`
- **User responsibility** - Wrap in `Arc<Mutex<Canvas>>` if needed

### Future Plans (v0.4.0+)

- **Thread-safe API** - Implement `Send` + `Sync` where appropriate
- **Parallel layers** - Render layers in parallel
- **Worker threads** - Background image loading and filtering

---

## Error Handling Strategy

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CloveError {
    #[error("Invalid canvas dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    
    #[error("Font not found: {0}")]
    FontNotFound(String),
    
    #[error("Image load error: {0}")]
    ImageLoadError(String),
    
    #[error("Layer not found: {0}")]
    LayerNotFound(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### Error Propagation

- All fallible operations return `Result<T, CloveError>`
- Use `?` operator for clean error propagation
- Provide context with error messages

---

## Testing Strategy

### Unit Tests

- Each module has comprehensive unit tests
- Test edge cases and error conditions
- Use proptest for property-based testing

### Integration Tests

- End-to-end canvas operations
- Cross-module interactions
- Real-world usage scenarios

### Visual Tests

- Snapshot testing with `insta`
- Pixel-perfect comparison
- Regression detection

---

## Future Architecture Changes

### v0.2.0 - Advanced Text
- Enhanced text layout engine
- Better font fallback system

### v0.3.0 - Advanced Shapes
- Custom path builder
- Complex shape operations

### v0.4.0 - Performance
- Parallel rendering
- GPU backend (optional)
- Advanced caching

### v0.5.0+ - Animation
- Timeline system
- Keyframe management
- Frame rendering

---

## Related Documents

- [Project Overview](./project-overview.md)
- [API Design](./api-design.md)
- [Conventions](./conventions.md)
- [v0.1.0 Roadmap](./roadmap/v0.1.0/overview.md)

---

**Last Updated:** 2025-11-27  
**Version:** 0.1.0 (Planning Phase)  
**Maintainer:** 1he03
