# Clove2d API Design

## @context: API Design & Patterns
## @version: v0.1.0
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Method Chaining Pattern](#method-chaining-pattern)
3. [Type Signatures](#type-signatures)
4. [Complete API Examples](#complete-api-examples)
5. [Error Handling](#error-handling)
6. [Best Practices](#best-practices)

---

## Design Philosophy

### Core Principles

1. **Fluent Interface** - Method chaining for readable, declarative code
2. **Type Safety** - Leverage Rust's type system
3. **Discoverability** - IntelliSense-friendly API
4. **Predictability** - Consistent patterns across all systems
5. **Fail Fast** - Return `Result` for all fallible operations

### API Style: Method Chaining

```rust
// ✅ Good - Fluent, readable
canvas.create_layer("shapes")?
    .draw_circle()
        .center(200, 200)
        .radius(50)
        .fill(Color::RED)
        .stroke(Color::BLACK, 2.0)
        .draw()?;

// ❌ Avoid - Imperative, verbose
let mut layer = canvas.create_layer("shapes")?;
let circle = Circle::new();
circle.set_center(200, 200);
circle.set_radius(50);
circle.set_fill(Color::RED);
circle.set_stroke(Color::BLACK, 2.0);
layer.draw_circle(circle)?;
```

---

## Method Chaining Pattern

### Builder Pattern Implementation

```rust
pub struct Canvas {
    // internal fields
}

impl Canvas {
    // Builder constructor
    pub fn builder() -> CanvasBuilder {
        CanvasBuilder::new()
    }
    
    // Chaining methods return &mut Self
    pub fn clear(&mut self, color: Color) -> &mut Self {
        self.backend.clear(color);
        self
    }
    
    // Terminal operations return Result
    pub fn save(&self, path: &str) -> Result<()> {
        // ...
    }
}
```

### Shape Builder Pattern

```rust
pub struct CircleBuilder<'a> {
    layer: &'a mut Layer,
    center: Point,
    radius: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
    // ...
}

impl<'a> CircleBuilder<'a> {
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center = Point::new(x, y);
        self
    }
    
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Draw the circle
        Ok(self.layer)
    }
}
```

---

## Type Signatures

### Canvas API

```rust
// Creation
impl Canvas {
    pub fn builder() -> CanvasBuilder;
    pub fn new(width: u32, height: u32) -> Result<Self>;
}

impl CanvasBuilder {
    pub fn size(self, width: u32, height: u32) -> Self;
    pub fn background(self, color: Color) -> Self;
    pub fn build(self) -> Result<Canvas>;
}

// Layer Management
impl Canvas {
    pub fn create_layer(&mut self, name: &str) -> Result<&mut Layer>;
    pub fn layer(&mut self, name: &str) -> Result<&mut Layer>;
    pub fn layer_mut(&mut self, name: &str) -> Result<&mut Layer>;
    pub fn merge_layers(&mut self) -> Result<&mut Self>;
    pub fn remove_layer(&mut self, name: &str) -> Result<()>;
}

// Save/Export
impl Canvas {
    pub fn save(&self, path: &str) -> Result<()>;
    pub fn save_with_quality(&self, path: &str, quality: u8) -> Result<()>;
    pub fn to_buffer(&self, format: ImageFormat) -> Result<Vec<u8>>;
}
```

### Color API

```rust
impl Color {
    // Construction
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self;
    pub fn rgb(r: u8, g: u8, b: u8) -> Self;
    pub fn hex(s: &str) -> Result<Self>;
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self;
    pub fn hsl(h: f32, s: f32, l: f32) -> Self;
    
    // Named colors
    pub const RED: Color;
    pub const BLUE: Color;
    pub const GREEN: Color;
    // ... more
    
    // Gradients
    pub fn linear_gradient() -> LinearGradientBuilder;
    pub fn radial_gradient() -> RadialGradientBuilder;
    
    // Manipulation
    pub fn lighten(&self, amount: f32) -> Self;
    pub fn darken(&self, amount: f32) -> Self;
    pub fn saturate(&self, amount: f32) -> Self;
    pub fn desaturate(&self, amount: f32) -> Self;
}

impl LinearGradientBuilder {
    pub fn start(self, x: f32, y: f32) -> Self;
    pub fn end(self, x: f32, y: f32) -> Self;
    pub fn add_stop(self, position: f32, color: Color) -> Self;
    pub fn build(self) -> Color;
}
```

### Shapes API

```rust
// Rectangle
impl Layer {
    pub fn draw_rect(&mut self) -> RectBuilder;
}

impl RectBuilder<'_> {
    pub fn position(self, x: f32, y: f32) -> Self;
    pub fn size(self, width: f32, height: f32) -> Self;
    pub fn fill(self, color: Color) -> Self;
    pub fn stroke(self, color: Color, width: f32) -> Self;
    pub fn corner_radius(self, radius: f32) -> Self; // for rounded rect
    pub fn shadow(self, color: Color, x: f32, y: f32, blur: f32) -> Self;
    pub fn draw(self) -> Result<&'a mut Layer>;
}

// Circle
impl Layer {
    pub fn draw_circle(&mut self) -> CircleBuilder;
}

impl CircleBuilder<'_> {
    pub fn center(self, x: f32, y: f32) -> Self;
    pub fn radius(self, r: f32) -> Self;
    pub fn fill(self, color: Color) -> Self;
    pub fn stroke(self, color: Color, width: f32) -> Self;
    pub fn draw(self) -> Result<&'a mut Layer>;
}

// Line
impl Layer {
    pub fn draw_line(&mut self) -> LineBuilder;
}

impl LineBuilder<'_> {
    pub fn from(self, x: f32, y: f32) -> Self;
    pub fn to(self, x: f32, y: f32) -> Self;
    pub fn stroke(self, color: Color, width: f32) -> Self;
    pub fn dash_pattern(self, pattern: &[f32]) -> Self;
    pub fn draw(self) -> Result<&'a mut Layer>;
}
```

### Text API

```rust
// Font Manager
impl FontManager {
    pub fn new() -> Self;
    pub fn load(&mut self, name: &str, path: &str) -> Result<&mut Self>;
    pub fn set_default(&mut self, name: &str) -> Result<()>;
}

// Text Drawing
impl Layer {
    pub fn draw_text(&mut self, text: &str) -> TextBuilder;
}

impl TextBuilder<'_> {
    // Font
    pub fn font_family(self, family: &str) -> Self;
    pub fn font_size(self, size: f32) -> Self;
    pub fn font_weight(self, weight: FontWeight) -> Self;
    pub fn font_style(self, style: FontStyle) -> Self;
    
    // Position & Alignment
    pub fn position(self, x: f32, y: f32) -> Self;
    pub fn align(self, align: TextAlign) -> Self;
    pub fn vertical_align(self, align: VerticalAlign) -> Self;
    pub fn direction(self, dir: TextDirection) -> Self;
    
    // Styling
    pub fn color(self, color: Color) -> Self;
    pub fn stroke(self, color: Color, width: f32) -> Self;
    pub fn shadow(self, color: Color, x: f32, y: f32, blur: f32) -> Self;
    pub fn background(self, color: Color) -> Self;
    
    // Layout
    pub fn max_width(self, width: f32) -> Self;
    pub fn line_height(self, height: f32) -> Self;
    pub fn letter_spacing(self, spacing: f32) -> Self;
    pub fn word_spacing(self, spacing: f32) -> Self;
    
    // Decoration
    pub fn underline(self, enabled: bool) -> Self;
    pub fn strikethrough(self, enabled: bool) -> Self;
    
    pub fn draw(self) -> Result<&'a mut Layer>;
}
```

### Image API

```rust
impl Layer {
    pub fn load_image(&mut self, path: &str) -> Result<ImageBuilder>;
    pub async fn load_image_async(&mut self, url: &str) -> Result<ImageBuilder>;
    pub fn load_image_from_buffer(&mut self, buffer: &[u8]) -> Result<ImageBuilder>;
}

impl ImageBuilder<'_> {
    pub fn position(self, x: f32, y: f32) -> Self;
    pub fn size(self, width: f32, height: f32) -> Self;
    pub fn opacity(self, opacity: f32) -> Self;
    pub fn blend_mode(self, mode: BlendMode) -> Self;
    pub fn draw(self) -> Result<&'a mut Layer>;
}
```

### Layer API

```rust
impl Layer {
    pub fn opacity(&mut self, opacity: f32) -> &mut Self;
    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self;
    pub fn visible(&mut self, visible: bool) -> &mut Self;
    pub fn apply_filter(&mut self, filter: Filter) -> &mut Self;
    pub fn rotate(&mut self, angle: f32) -> &mut Self;
    pub fn scale(&mut self, sx: f32, sy: f32) -> &mut Self;
    pub fn translate(&mut self, x: f32, y: f32) -> &mut Self;
}
```

---

## Complete API Examples

### Example 1: Basic Shapes

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::WHITE)
        .build()?;
    
    canvas.create_layer("shapes")?
        .draw_rect()
            .position(50, 50)
            .size(200, 150)
            .fill(Color::hex("#FF5733")?)
            .stroke(Color::BLACK, 2.0)
            .draw()?
        .draw_circle()
            .center(400, 300)
            .radius(100)
            .fill(Color::rgba(0, 128, 255, 200))
            .stroke(Color::WHITE, 3.0)
            .draw()?;
    
    canvas.save("shapes.png")?;
    Ok(())
}
```

### Example 2: Gradients

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    let gradient = Color::linear_gradient()
        .start(0.0, 0.0)
        .end(800.0, 600.0)
        .add_stop(0.0, Color::hex("#FF0080")?)
        .add_stop(0.5, Color::hex("#7928CA")?)
        .add_stop(1.0, Color::hex("#0070F3")?)
        .build();
    
    canvas.create_layer("background")?
        .draw_rect()
            .position(0, 0)
            .size(800, 600)
            .fill(gradient)
            .draw()?;
    
    canvas.save("gradient.png")?;
    Ok(())
}
```

### Example 3: Arabic Text

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    // Setup fonts
    let mut fonts = FontManager::new();
    fonts.load("Tajawal", "assets/fonts/Tajawal-Regular.ttf")?;
    fonts.load("TajawalBold", "assets/fonts/Tajawal-Bold.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(800, 400)
        .background(Color::hex("#F5F5F5")?)
        .build()?;
    
    canvas.create_layer("text")?
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("TajawalBold")
            .font_size(48)
            .color(Color::hex("#2C3E50")?)
            .position(400, 150)
            .align(TextAlign::Center)
            .direction(TextDirection::RTL)
            .stroke(Color::WHITE, 2.0)
            .shadow(Color::rgba(0, 0, 0, 50), 2, 2, 4)
            .draw()?
        .draw_text("مكتبة رسومات ثنائية الأبعاد بلغة Rust")
            .font_family("Tajawal")
            .font_size(24)
            .color(Color::hex("#34495E")?)
            .position(400, 220)
            .align(TextAlign::Center)
            .direction(TextDirection::RTL)
            .draw()?;
    
    canvas.save("arabic_text.png")?;
    Ok(())
}
```

### Example 4: Multi-line Text with Wrapping

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut fonts = FontManager::new();
    fonts.load("Roboto", "assets/fonts/Roboto-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(600, 400)
        .background(Color::WHITE)
        .build()?;
    
    let long_text = "This is a very long text that will automatically \
                     wrap when it reaches the maximum width specified. \
                     The text layout engine will handle line breaks \
                     intelligently.";
    
    canvas.create_layer("text")?
        .draw_text(long_text)
            .font_family("Roboto")
            .font_size(18)
            .color(Color::BLACK)
            .position(50, 50)
            .max_width(500)
            .line_height(1.5)
            .align(TextAlign::Justify)
            .draw()?;
    
    canvas.save("wrapped_text.png")?;
    Ok(())
}
```

### Example 5: Image Loading and Manipulation

```rust
use clove2d::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::hex("#1A1A1A")?)
        .build()?;
    
    canvas.create_layer("image")?
        .load_image_async("https://example.com/photo.jpg").await?
            .position(100, 100)
            .size(600, 400)
            .opacity(0.9)
            .draw()?
        .apply_filter(Filter::blur(5.0))
        .apply_filter(Filter::brightness(1.2));
    
    canvas.save("processed_image.png")?;
    Ok(())
}
```

### Example 6: Layers and Compositing

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    // Background layer
    canvas.create_layer("background")?
        .draw_rect()
            .position(0, 0)
            .size(800, 600)
            .fill(Color::hex("#2C3E50")?)
            .draw()?;
    
    // Shapes layer
    canvas.create_layer("shapes")?
        .draw_circle()
            .center(400, 300)
            .radius(150)
            .fill(Color::rgba(255, 100, 100, 180))
            .draw()?
        .blend_mode(BlendMode::Multiply)
        .opacity(0.8);
    
    // Text layer
    canvas.create_layer("text")?
        .draw_text("Layered Composition")
            .font_family("Arial")
            .font_size(36)
            .color(Color::WHITE)
            .position(400, 50)
            .align(TextAlign::Center)
            .draw()?;
    
    // Merge and save
    canvas.merge_layers()?
        .save("layered.png")?;
    
    Ok(())
}
```

### Example 7: Complex Scene

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut fonts = FontManager::new();
    fonts.load("Arial", "assets/fonts/Arial.ttf")?;
    fonts.load("Tajawal", "assets/fonts/Tajawal-Bold.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1200, 800)
        .background(Color::hex("#ECEFF1")?)
        .build()?;
    
    // Gradient background
    canvas.create_layer("background")?
        .draw_rect()
            .position(0, 0)
            .size(1200, 800)
            .fill(Color::linear_gradient()
                .start(0, 0)
                .end(1200, 800)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .draw()?;
    
    // Card
    canvas.create_layer("card")?
        .draw_rect()
            .position(300, 200)
            .size(600, 400)
            .fill(Color::WHITE)
            .corner_radius(20)
            .shadow(Color::rgba(0, 0, 0, 30), 0, 10, 40)
            .draw()?
        .draw_text("Clove2d")
            .font_family("Arial")
            .font_size(64)
            .font_weight(FontWeight::Bold)
            .color(Color::hex("#2C3E50")?)
            .position(600, 320)
            .align(TextAlign::Center)
            .draw()?
        .draw_text("مكتبة رسومات احترافية")
            .font_family("Tajawal")
            .font_size(32)
            .color(Color::hex("#667EEA")?)
            .position(600, 400)
            .align(TextAlign::Center)
            .direction(TextDirection::RTL)
            .draw()?
        .draw_circle()
            .center(600, 480)
            .radius(30)
            .fill(Color::hex("#4CAF50")?)
            .draw()?;
    
    canvas.merge_layers()?
        .save("complex_scene.png")?;
    
    Ok(())
}
```

---

## Error Handling

### Result Type Pattern

```rust
// All fallible operations return Result
pub type Result<T> = std::result::Result<T, CloveError>;

// Usage
fn create_image() -> Result<()> {
    let canvas = Canvas::builder()
        .size(800, 600)
        .build()?; // Propagate errors with ?
    
    canvas.create_layer("layer1")?;
    canvas.save("output.png")?;
    
    Ok(())
}
```

### Error Handling Examples

```rust
use clove2d::prelude::*;

fn main() {
    match create_canvas() {
        Ok(_) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn create_canvas() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    // Font loading might fail
    let mut fonts = FontManager::new();
    if let Err(e) = fonts.load("MyFont", "fonts/missing.ttf") {
        eprintln!("Font load failed: {}, using default", e);
        // Continue with default font
    }
    
    canvas.create_layer("shapes")?
        .draw_rect()
            .position(0, 0)
            .size(800, 600)
            .fill(Color::hex("#INVALID")?) // Will fail and propagate
            .draw()?;
    
    Ok(())
}
```

---

## Best Practices

### 1. Use the Prelude

```rust
// ✅ Good
use clove2d::prelude::*;

// ❌ Verbose
use clove2d::canvas::Canvas;
use clove2d::color::Color;
use clove2d::shapes::*;
// ...
```

### 2. Builder Pattern for Complex Objects

```rust
// ✅ Good - Fluent and clear
let canvas = Canvas::builder()
    .size(800, 600)
    .background(Color::WHITE)
    .build()?;

// ❌ Avoid - Too many parameters
let canvas = Canvas::new(800, 600, Some(Color::WHITE), None, None)?;
```

### 3. Reuse Font Manager

```rust
// ✅ Good - Load fonts once
let mut fonts = FontManager::new();
fonts.load("Arial", "arial.ttf")?;
fonts.load("Tajawal", "tajawal.ttf")?;
fonts.set_default("Arial")?;

// Use throughout application
// Fonts are cached internally
```

### 4. Layer Organization

```rust
// ✅ Good - Organized layers
canvas.create_layer("background")?;
canvas.create_layer("content")?;
canvas.create_layer("overlay")?;

// Later modify specific layers
canvas.layer("content")?
    .opacity(0.8)
    .apply_filter(Filter::blur(2.0));
```

### 5. Error Propagation

```rust
// ✅ Good - Use ? operator
fn render() -> Result<()> {
    let canvas = create_canvas()?;
    add_content(&mut canvas)?;
    canvas.save("output.png")?;
    Ok(())
}

// ❌ Avoid - Unwrap in library code
fn render() {
    let canvas = create_canvas().unwrap(); // Panic!
}
```

### 6. Type Hints for Colors

```rust
// ✅ Good - Clear intent
let primary = Color::hex("#667EEA")?;
let accent = Color::rgba(255, 100, 100, 200);
let background = Color::WHITE;

// Usage
rect.fill(primary)
```

---

## API Stability

### Version Guarantees

- **v0.x.x** - Breaking changes allowed between minor versions
- **v1.0.0+** - Semantic versioning
  - Major: Breaking changes
  - Minor: New features (backward compatible)
  - Patch: Bug fixes

### Deprecation Policy

```rust
// Deprecated APIs marked with #[deprecated]
#[deprecated(since = "0.2.0", note = "Use `draw_rect()` instead")]
pub fn rectangle(&mut self) -> RectBuilder {
    self.draw_rect()
}
```

---

## Related Documents

- [Architecture](./architecture.md)
- [Project Overview](./project-overview.md)
- [Conventions](./conventions.md)
- [v0.1.0 Overview](./roadmap/v0.1.0/overview.md)

---

**Last Updated:** 2025-11-27  
**Version:** 0.1.0 (Planning Phase)  
**Maintainer:** 1he03
