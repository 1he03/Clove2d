# Clove2d Coding Conventions

## @context: Development Standards
## @version: v0.1.0
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Code Style](#code-style)
2. [Naming Conventions](#naming-conventions)
3. [Documentation Standards](#documentation-standards)
4. [Testing Conventions](#testing-conventions)
5. [Git Workflow](#git-workflow)
6. [Pull Request Guidelines](#pull-request-guidelines)
7. [Error Messages](#error-messages)
8. [Performance Guidelines](#performance-guidelines)

---

## Code Style

### Rust Edition & Formatting

```toml
# Cargo.toml
[package]
edition = "2021"

# Use rustfmt for consistent formatting
```

**Run before commit:**
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

### Code Organization

```rust
// File structure
// 1. Module documentation
// 2. Imports (grouped)
// 3. Constants
// 4. Type definitions
// 5. Implementations
// 6. Private helpers
// 7. Tests

//! Module documentation here

// Standard library
use std::collections::HashMap;
use std::path::Path;

// External crates
use tiny_skia::Pixmap;
use thiserror::Error;

// Internal modules
use crate::color::Color;
use crate::error::Result;

// Constants
const DEFAULT_SIZE: u32 = 800;
const MAX_LAYERS: usize = 100;

// Types and implementations...
```

### Line Length

- **Maximum 100 characters** per line
- Break long function signatures:

```rust
// ✅ Good
pub fn create_gradient(
    start: Point,
    end: Point,
    stops: Vec<(f32, Color)>,
) -> Result<Gradient> {
    // ...
}

// ❌ Too long
pub fn create_gradient(start: Point, end: Point, stops: Vec<(f32, Color)>) -> Result<Gradient> { /* ... */ }
```

### Indentation

- **4 spaces** (configured in rustfmt.toml)
- No tabs

---

## Naming Conventions

### General Rules

| Item | Convention | Example |
|------|------------|--------|
| Crates | snake_case | `clove2d` |
| Modules | snake_case | `font_manager` |
| Types | PascalCase | `Canvas`, `FontManager` |
| Traits | PascalCase | `Drawable`, `Renderable` |
| Enums | PascalCase | `Color`, `BlendMode` |
| Variants | PascalCase | `Color::Red`, `BlendMode::Normal` |
| Functions | snake_case | `create_layer`, `draw_rect` |
| Methods | snake_case | `set_opacity`, `get_bounds` |
| Constants | SCREAMING_SNAKE | `MAX_LAYERS`, `DEFAULT_FONT` |
| Statics | SCREAMING_SNAKE | `GLOBAL_CACHE` |
| Variables | snake_case | `layer_count`, `pixel_data` |
| Lifetimes | lowercase, descriptive | `'a`, `'canvas` |
| Type parameters | Single uppercase | `T`, `E`, or PascalCase | `Item` |

### Specific Conventions

**Builder types:**
```rust
pub struct CanvasBuilder { /* ... */ }
pub struct RectBuilder<'a> { /* ... */ }
pub struct TextBuilder<'a> { /* ... */ }
```

**Error types:**
```rust
pub enum CloveError { /* ... */ }
pub type Result<T> = std::result::Result<T, CloveError>;
```

**Trait naming:**
```rust
// Adjectives for capabilities
trait Drawable { }
trait Fillable { }
trait Strokeable { }

// "-able" suffix for behaviors
trait Transformable { }
trait Composable { }
```

**Method prefixes:**
```rust
// Getters - no prefix
fn width(&self) -> u32 { }
fn height(&self) -> u32 { }

// Boolean queries - is_/has_/can_
fn is_visible(&self) -> bool { }
fn has_alpha(&self) -> bool { }
fn can_blend(&self) -> bool { }

// Setters - avoid set_ prefix, use builder pattern
fn opacity(mut self, value: f32) -> Self { } // ✅
fn set_opacity(&mut self, value: f32) { }    // ❌

// Conversions
fn to_rgba(&self) -> Rgba { }    // Consumes or clones
fn as_bytes(&self) -> &[u8] { }  // Borrows
fn into_buffer(self) -> Vec<u8> { } // Takes ownership
```

---

## Documentation Standards

### Module Documentation

```rust
//! Canvas system for Clove2d.
//!
//! This module provides the main `Canvas` type and related builders
//! for creating and manipulating 2D graphics.
//!
//! # Examples
//!
//! ```
//! use clove2d::prelude::*;
//!
//! let canvas = Canvas::builder()
//!     .size(800, 600)
//!     .build()?;
//! ```

mod canvas;
```

### Type Documentation

```rust
/// A 2D canvas for drawing graphics.
///
/// The canvas is the main entry point for all drawing operations.
/// It manages layers, coordinates, and final image output.
///
/// # Examples
///
/// ```
/// use clove2d::prelude::*;
///
/// let mut canvas = Canvas::builder()
///     .size(800, 600)
///     .background(Color::WHITE)
///     .build()?;
///
/// canvas.create_layer("shapes")?;
/// canvas.save("output.png")?;
/// # Ok::<(), clove2d::CloveError>(())
/// ```
pub struct Canvas {
    // fields...
}
```

### Function Documentation

```rust
/// Creates a new layer with the given name.
///
/// # Arguments
///
/// * `name` - A unique name for the layer
///
/// # Returns
///
/// A mutable reference to the newly created layer.
///
/// # Errors
///
/// Returns `CloveError::InvalidLayerName` if:
/// - The name is empty
/// - A layer with this name already exists
///
/// Returns `CloveError::LayerLimitExceeded` if the maximum
/// number of layers (100) has been reached.
///
/// # Examples
///
/// ```
/// use clove2d::prelude::*;
///
/// let mut canvas = Canvas::new(800, 600)?;
/// let layer = canvas.create_layer("background")?;
/// # Ok::<(), clove2d::CloveError>(())
/// ```
pub fn create_layer(&mut self, name: &str) -> Result<&mut Layer> {
    // implementation
}
```

### Error Documentation

```rust
/// Errors that can occur in Clove2d.
#[derive(Debug, thiserror::Error)]
pub enum CloveError {
    /// Invalid canvas dimensions.
    ///
    /// Both width and height must be greater than 0 and
    /// less than 16384 pixels.
    #[error("Invalid canvas dimensions: {width}x{height}")]
    InvalidDimensions {
        width: u32,
        height: u32,
    },
    
    /// Font file not found or failed to load.
    #[error("Font not found: {0}")]
    FontNotFound(String),
}
```

### Documentation Coverage

**Required documentation:**
- All public types, traits, functions, methods
- All public modules
- Complex algorithms (even if private)

**Documentation checklist:**
- [ ] Summary sentence
- [ ] Detailed description (if needed)
- [ ] Examples (for complex APIs)
- [ ] Arguments (with descriptions)
- [ ] Return value
- [ ] Errors (what can fail and why)
- [ ] Panics (if applicable)
- [ ] Safety (for unsafe code)

---

## Testing Conventions

### Test Organization

```rust
// In src/canvas/mod.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(800, 600);
        assert!(canvas.is_ok());
    }
    
    #[test]
    fn test_invalid_dimensions() {
        let canvas = Canvas::new(0, 0);
        assert!(canvas.is_err());
    }
}
```

### Integration Tests

```rust
// In tests/integration/canvas_tests.rs

use clove2d::prelude::*;

#[test]
fn test_complete_workflow() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    canvas.create_layer("layer1")?;
    canvas.save("/tmp/test_output.png")?;
    
    Ok(())
}
```

### Test Naming

```rust
// Pattern: test_<what>_<condition>_<expected>

#[test]
fn test_canvas_new_valid_dimensions_succeeds() { }

#[test]
fn test_canvas_new_zero_width_fails() { }

#[test]
fn test_layer_create_duplicate_name_returns_error() { }
```

### Property Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_color_conversion_roundtrip(
        r in 0u8..=255,
        g in 0u8..=255,
        b in 0u8..=255,
        a in 0u8..=255
    ) {
        let color = Color::rgba(r, g, b, a);
        let rgba = color.to_rgba();
        assert_eq!(rgba, (r, g, b, a));
    }
}
```

### Visual Tests

```rust
use insta::assert_snapshot;

#[test]
fn test_gradient_rendering() {
    let mut canvas = Canvas::new(100, 100).unwrap();
    // ... draw gradient ...
    
    let image = canvas.to_buffer(ImageFormat::Png).unwrap();
    assert_snapshot!("gradient", image);
}
```

---

## Git Workflow

### Branch Naming

```
main                    # Stable releases
develop                 # Integration branch
feature/<name>          # New features
fix/<name>              # Bug fixes
docs/<name>             # Documentation
refactor/<name>         # Code refactoring
test/<name>             # Test improvements
```

**Examples:**
```
feature/text-shadows
fix/gradient-rendering
docs/api-examples
refactor/layer-system
```

### Commit Messages

**Format:**
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting, missing semi-colons, etc.
- `refactor` - Code restructuring
- `perf` - Performance improvements
- `test` - Adding tests
- `chore` - Maintenance tasks

**Examples:**
```
feat(canvas): add layer opacity control

Implement opacity method for Layer to control
transparency of entire layer during composition.

Closes #42

---

fix(text): handle RTL text direction correctly

Fix BiDi algorithm application for mixed LTR/RTL text.
Adds unicode-bidi dependency for proper handling.

Fixes #58

---

docs(api): add gradient examples

Add comprehensive examples for linear and radial
gradients in API documentation.
```

### Commit Frequency

- Commit **logical units of work**
- Don't commit broken code to develop/main
- Use feature branches for work in progress
- Squash commits before merging to main

---

## Pull Request Guidelines

### PR Template

```markdown
## Description
[Describe the changes]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No new warnings

## Related Issues
Closes #XX
```

### PR Requirements

**Before submitting:**
1. Run `cargo fmt --all`
2. Run `cargo clippy --all-targets -- -D warnings`
3. Run `cargo test --all`
4. Update documentation
5. Add examples if new feature
6. Update CHANGELOG.md

**PR size:**
- Keep PRs focused and small
- Prefer multiple small PRs over one large PR
- Separate refactoring from feature additions

**Code review:**
- At least 1 approval required
- Address all comments
- Update PR based on feedback

---

## Error Messages

### Writing Good Error Messages

```rust
// ✅ Good - Specific and actionable
return Err(CloveError::InvalidDimensions {
    width,
    height,
});
// Message: "Invalid canvas dimensions: 0x600"
//          User knows exactly what's wrong

// ❌ Bad - Vague
return Err(CloveError::InvalidInput);
// Message: "Invalid input"
//          User doesn't know what to fix
```

### Error Message Guidelines

1. **Be specific** - Say what's wrong
2. **Be actionable** - Suggest how to fix
3. **Include context** - Show relevant values
4. **Use consistent format** - Match project style

**Examples:**

```rust
// Canvas errors
InvalidDimensions { width: 0, height: 600 }
// "Invalid canvas dimensions: 0x600"
// Improvement: "Canvas width must be > 0, got 0"

// Font errors
FontNotFound("Arial")
// "Font not found: Arial"
// Improvement: "Font 'Arial' not found. Load it with FontManager::load()"

// Layer errors
LayerNotFound("background")
// "Layer not found: background"
// Improvement: "Layer 'background' does not exist. Create it with create_layer()"
```

---

## Performance Guidelines

### Memory Allocation

```rust
// ✅ Good - Pre-allocate when size is known
let mut pixels = Vec::with_capacity(width * height * 4);

// ❌ Avoid - Multiple reallocations
let mut pixels = Vec::new();
for _ in 0..(width * height * 4) {
    pixels.push(0);
}
```

### Cloning

```rust
// ✅ Good - Borrow when possible
fn process_image(image: &RgbaImage) { }

// ❌ Avoid unnecessary clones
fn process_image(image: RgbaImage) { } // Takes ownership
```

### Iterators

```rust
// ✅ Good - Use iterator chains
let sum: u32 = pixels
    .iter()
    .map(|p| p.r as u32)
    .sum();

// ❌ Avoid - Manual loops when iterator works
let mut sum = 0;
for pixel in &pixels {
    sum += pixel.r as u32;
}
```

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_canvas_creation(c: &mut Criterion) {
    c.bench_function("canvas_new_800x600", |b| {
        b.iter(|| {
            Canvas::new(black_box(800), black_box(600))
        });
    });
}

criterion_group!(benches, bench_canvas_creation);
criterion_main!(benches);
```

---

## Code Review Checklist

### For Authors

- [ ] Code is self-explanatory
- [ ] Complex logic has comments
- [ ] No TODO comments
- [ ] No commented-out code
- [ ] Error handling is appropriate
- [ ] Tests cover new code
- [ ] Documentation is updated
- [ ] Examples work as shown

### For Reviewers

- [ ] Code follows conventions
- [ ] Logic is correct
- [ ] Error handling is complete
- [ ] Tests are sufficient
- [ ] Documentation is clear
- [ ] No performance regressions
- [ ] No security issues

---

## Unsafe Code Guidelines

### When to Use Unsafe

**Only when necessary:**
- FFI with C libraries
- Performance-critical code (after benchmarking)
- Implementing safe abstractions

### Unsafe Code Requirements

```rust
// SAFETY comment is MANDATORY
unsafe {
    // SAFETY: ptr is valid because it was just allocated
    // and hasn't been freed yet. The size is correct as
    // it matches the allocation size.
    std::ptr::copy_nonoverlapping(src, dst, len);
}
```

**Checklist:**
- [ ] SAFETY comment explains why it's safe
- [ ] All invariants are documented
- [ ] Thoroughly tested
- [ ] Alternatives considered
- [ ] Reviewed by maintainer

---

## Versioning

### Version Format

**Semantic Versioning:** `MAJOR.MINOR.PATCH`

- **MAJOR** - Breaking changes
- **MINOR** - New features (backward compatible)
- **PATCH** - Bug fixes

### Pre-1.0 Versions

- `0.x.x` - Breaking changes allowed in MINOR
- API not stable
- No backward compatibility guarantees

### Post-1.0 Versions

- Strict semantic versioning
- Breaking changes only in MAJOR
- Deprecation warnings before removal

---

## Related Documents

- [Architecture](./architecture.md)
- [API Design](./api-design.md)
- [Project Overview](./project-overview.md)
- [Contributing Guide](../CONTRIBUTING.md)

---

**Last Updated:** 2025-11-27  
**Version:** 0.1.0 (Planning Phase)  
**Maintainer:** 1he03
