# 🎨 Clove2d - Project Overview

## @context: Project Overview
## @version: v0.1.0
## @status: Planning Phase
## @last-updated: 2025-11-27

---

## Table of Contents

- [Project Information](#project-information)
- [Vision & Goals](#vision--goals)
- [Key Features](#key-features)
- [Technology Stack](#technology-stack)
- [Comparison with Ghe2d](#comparison-with-ghe2d)
- [Success Metrics](#success-metrics)
- [Target Audience](#target-audience)
- [Roadmap Summary](#roadmap-summary)

---

## Project Information

| Property | Value |
|----------|-------|
| **Project Name** | Clove2d |
| **Type** | Pure Rust 2D Graphics & Canvas Library |
| **License** | MIT OR Apache-2.0 |
| **Current Version** | 0.0.0 (Pre-release) |
| **Target Version** | v0.1.0 (MVP) |
| **Rust Edition** | 2021 |
| **Minimum Rust Version** | 1.75+ |
| **Repository** | https://github.com/1he03/Clove2d |
| **Author** | He (@1he03) |
| **Created** | November 27, 2025 |

---

## Vision & Goals

### 🎯 **Primary Vision**

Build a **powerful, easy-to-use, Pure Rust 2D graphics library** that excels at:
- Professional-grade canvas operations
- Advanced Arabic text rendering with BiDi support
- High-performance image manipulation
- Intuitive Method Chaining API

### 🌟 **Core Goals**

1. **Developer Experience**
   - Intuitive API with Method Chaining pattern
   - Comprehensive documentation with examples
   - Clear error messages
   - Type-safe design

2. **Arabic Text Excellence**
   - Best-in-class Arabic text support in Rust ecosystem
   - Proper BiDi (Bidirectional) text handling
   - Advanced text shaping for complex scripts
   - Mixed language support (Arabic + Latin)

3. **Performance**
   - Pure Rust (no C dependencies by default)
   - Optimized rendering pipeline
   - Efficient memory usage
   - Parallel processing where possible

4. **Flexibility**
   - Rich color system (RGBA, Hex, HSL, Gradients, Patterns)
   - Comprehensive shape library (12+ shapes)
   - Advanced text layout options
   - Layer-based composition
   - Extensive filter library

5. **Production Ready**
   - Comprehensive testing (unit, integration, visual)
   - Stable API from v1.0.0
   - Long-term support commitment
   - Active maintenance

---

## Key Features

### ✅ **v0.1.0 MVP Features**

#### **Canvas System**
- Canvas creation with custom dimensions
- Background customization
- Multiple output formats (PNG, JPEG, WebP)
- State management (save/restore)
- Method chaining API

#### **Color System**
- **RGBA** - Standard red, green, blue, alpha
- **Hex** - Hexadecimal color codes (#RRGGBB, #RRGGBBAA)
- **HSL/HSLA** - Hue, saturation, lightness
- **Named Colors** - CSS-style color names
- **Linear Gradients** - Multi-stop color gradients
- **Radial Gradients** - Circular color gradients
- **Patterns** - Image-based fill patterns

#### **Shapes (12+)**
1. Rectangle (with fill, stroke, shadow)
2. Rounded Rectangle (customizable corner radius)
3. Circle (perfect circles)
4. Ellipse (oval shapes)
5. Line (straight lines with customizable caps)
6. Polyline (connected line segments)
7. Polygon (closed shapes)
8. Triangle (3-point shapes)
9. Arc (circular arcs)
10. Quadratic Bezier Curve
11. Cubic Bezier Curve
12. Star (customizable points)

**Shape Features:**
- Fill with solid colors or gradients
- Stroke with customizable width and style
- Shadow effects (offset, blur, color)
- Opacity control
- Transformations (rotate, scale, translate)

#### **Text System**

**Font Management:**
- Font Manager for centralized font handling
- Font family registration
- Font caching for performance
- Fallback font support

**Text Rendering:**
- Single-line text
- Multi-line text with automatic wrapping
- Manual line breaks (\n)
- Word wrapping with max width constraint

**Text Styling:**
- Font family, size, weight (normal, bold)
- Font style (normal, italic)
- Color (solid or gradient)
- Stroke (outline)
- Shadow effects
- Letter spacing
- Word spacing
- Line height
- Underline
- Strikethrough
- Background color

**Text Layout:**
- Horizontal alignment (left, center, right, justify)
- Vertical alignment (top, middle, bottom)
- Text direction (LTR, RTL, auto-detect)
- BiDi support for mixed text
- Arabic text shaping
- Text overflow handling (clip, ellipsis)
- Max width/height constraints

#### **Image System**

**Loading:**
- Local file loading (sync)
- URL loading (async)
- Buffer loading
- Supported formats: PNG, JPEG, WebP, GIF, BMP

**Operations:**
- Draw image on canvas
- Resize images
- Crop images
- Rotate images
- Flip (horizontal/vertical)
- Blend modes (normal, multiply, overlay, screen, etc.)

**Encoding:**
- PNG (customizable compression)
- JPEG (customizable quality)
- WebP (lossy/lossless)

#### **Layer System**
- Create multiple named layers
- Layer ordering (z-index)
- Show/hide layers
- Layer opacity
- Blend modes between layers
- Merge layers
- Clone layers

#### **Filter System (9 Filters)**
1. Blur (Gaussian)
2. Sharpen
3. Grayscale
4. Sepia
5. Invert
6. Brightness adjustment
7. Contrast adjustment
8. Saturation adjustment
9. Hue rotation

#### **Transform System**
- Rotate (by angle)
- Scale (uniform or non-uniform)
- Translate (move)
- Crop (rectangular region)
- Flip horizontal
- Flip vertical
- Transform chains

---

### 🚀 **Future Features (v0.2.0+)**

- **Advanced Text** (v0.2.0)
  - Text justification
  - Hyphenation
  - Text on path
  - Advanced text effects
  
- **Advanced Shapes** (v0.3.0)
  - Custom path builder
  - Complex curves
  - Advanced geometry
  
- **Performance** (v0.4.0)
  - Parallel rendering
  - Advanced caching
  - Memory optimization
  
- **Animation** (v0.5.0)
  - Frame-based animation
  - Timeline system
  - Keyframe animation
  - GIF export
  
- **Templates** (v0.6.0)
  - Template engine
  - Variable substitution
  - Conditional rendering
  
- **CLI Tool** (v0.7.0)
  - Command-line interface
  - Batch processing
  
- **WASM** (v0.8.0)
  - Browser support
  - JavaScript interop

---

## Technology Stack

### **Core Dependencies**

```toml
# Rendering Backend
tiny-skia = "0.11"           # Pure Rust 2D rendering
tiny-skia-path = "0.11"      # Path operations

# Text Rendering
cosmic-text = "0.12"         # Advanced text layout
rustybuzz = "0.18"           # Text shaping (HarfBuzz port)
unicode-bidi = "0.3"         # Bidirectional text algorithm
unicode-segmentation = "1.11" # Text segmentation

# Image Processing
image = "0.25"               # Image encoding/decoding
png = "0.17"                 # PNG support
jpeg-decoder = "0.3"         # JPEG support
webp = "0.3"                 # WebP support

# Async Support
tokio = "1.40"               # Async runtime (optional)
reqwest = "0.12"             # HTTP client (optional)

# Color Management
palette = "0.7"              # Color conversions

# Math & Geometry
euclid = "0.22"              # 2D geometry types
kurbo = "0.11"               # Bezier curves

# Utilities
thiserror = "1.0"            # Error handling
once_cell = "1.19"           # Lazy initialization
parking_lot = "0.12"         # Efficient synchronization
arrayvec = "0.7"             # Stack arrays
```

### **Why Pure Rust?**

✅ **Advantages:**
- No C dependencies (easier compilation)
- Cross-platform without system libraries
- Memory safe
- Better integration with Rust ecosystem
- Smaller binary size
- Easier to distribute

⚠️ **Trade-offs:**
- Slightly slower than Skia/Cairo (10-20%)
- Less battle-tested than C libraries

**Decision:** Pure Rust for v0.1.0, with optional C backend support in future versions

---

## Comparison with Ghe2d

| Feature | Ghe2d | Clove2d v0.1.0 | Improvement |
|---------|-------|----------------|-------------|
| **Shapes** | 2 (rect, circle) | 12+ shapes | **6x more** |
| **Colors** | RGBA only | RGBA, Hex, HSL, Named, Gradients, Patterns | **7x more options** |
| **Text Rendering** | Basic + Arabic | Advanced + BiDi + Multi-line + Layout | **Much more advanced** |
| **Font Management** | Manual load each time | Font Manager + Caching | **Automatic** |
| **Image Loading** | Basic (sync only) | Sync + Async + Multiple formats | **More flexible** |
| **Layers** | ❌ None | ✅ Full layer system | **New feature** |
| **Filters** | ❌ None | ✅ 9 filters | **New feature** |
| **Transformations** | ❌ None | ✅ Rotate, Scale, Translate, Crop | **New feature** |
| **API Style** | Imperative | Method Chaining | **More fluent** |
| **Error Handling** | Weak (errors hidden) | Strong (Result everywhere) | **Much better** |
| **Tests** | ❌ None | ✅ Comprehensive (unit, integration, visual) | **Production ready** |
| **Documentation** | Minimal (1 line README) | Extensive (guides, examples, API docs) | **Professional** |
| **Examples** | ❌ None | ✅ 11+ examples | **Learning friendly** |
| **Performance** | Not optimized | Optimized with benchmarks | **Faster** |
| **Backend** | image crate | tiny-skia (faster) | **Better quality** |
| **Arabic Support** | Custom algorithm | rustybuzz (HarfBuzz) | **More accurate** |
| **Binary Size** | ~2-3 MB | ~2-3 MB | **Similar** |

**Overall:** Clove2d is a **complete rewrite** with **professional-grade features**, not just an improvement.

---

## Success Metrics

### **Technical Metrics**

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Code Coverage** | > 80% | cargo-tarpaulin |
| **Documentation Coverage** | > 90% | cargo doc |
| **Clippy Warnings** | 0 | cargo clippy |
| **Compilation Time** | < 2 min | cargo build --release |
| **Binary Size** | < 5 MB | cargo build --release |

### **Performance Targets**

| Operation | Target | Benchmark |
|-----------|--------|----------|
| **Render 1000x1000 canvas** | < 50ms | criterion |
| **Draw 1000 shapes** | < 100ms | criterion |
| **Render Arabic text (100 chars)** | < 20ms | criterion |
| **Apply Gaussian blur** | < 30ms | criterion |
| **Load 1920x1080 image** | < 100ms | criterion |

### **Community Metrics**

| Metric | 6 Months | 1 Year |
|--------|----------|--------|
| **GitHub Stars** | 100+ | 500+ |
| **Contributors** | 5+ | 10+ |
| **Downloads (crates.io)** | 1,000+/month | 5,000+/month |
| **Issues Resolved** | 90%+ | 95%+ |
| **PR Response Time** | < 48h | < 24h |

---

## Target Audience

### **Primary Users**

1. **Arabic Content Creators**
   - Social media graphics
   - Islamic art
   - Arabic typography projects
   - Bilingual content

2. **Rust Developers**
   - Server-side image generation
   - CLI tools
   - Web services (watermarks, thumbnails)
   - Data visualization

3. **Game Developers**
   - 2D game assets
   - UI rendering
   - Sprite generation
   - Texture creation

4. **Data Scientists**
   - Chart generation
   - Report creation
   - Custom visualizations

### **Secondary Users**

5. **Web Developers**
   - OG image generation
   - Dynamic graphics
   - PDF generation (future)

6. **Educators**
   - Teaching materials
   - Diagram generation
   - Arabic education content

---

## Roadmap Summary

```
v0.1.0 (MVP)          ━━━━━━━━━━ 8-10 weeks ━━━━━━━━━━┓
├─ Phase 1: Foundation (2 weeks)                      ├─ CURRENT FOCUS
├─ Phase 2: Canvas Core (2 weeks)                     │
├─ Phase 3: Shapes (2 weeks)                          │
├─ Phase 4: Text System (2 weeks)                     │
├─ Phase 5: Images & Layers (1 week)                  │
└─ Phase 6: Filters & Polish (1 week)                 ┛

v0.2.0 (Advanced Text)    ━━━ 4-6 weeks ━━━
v0.3.0 (Shapes & Filters) ━━━ 4-6 weeks ━━━
v0.4.0 (Performance)      ━━━ 3-4 weeks ━━━
v0.5.0 (Animation)        ━━━ 6-8 weeks ━━━  ← Future
v0.6.0 (Templates)        ━━━ 3-4 weeks ━━━  ← Future
v0.7.0 (CLI)              ━━━ 3-4 weeks ━━━  ← Future
v0.8.0 (WASM)             ━━━ 4-6 weeks ━━━  ← Future
v0.9.0 (Polish)           ━━━ 4-6 weeks ━━━  ← Future
v1.0.0 (Stable)           ━━━ 2-3 weeks ━━━  ← Future
```

**Total estimated time to v1.0.0:** ~40-50 weeks (~10-12 months)

---

## Related Documents

- [Architecture](.cursor/architecture.md) - System architecture and design
- [API Design](.cursor/api-design.md) - API patterns and examples
- [Conventions](.cursor/conventions.md) - Coding standards
- [v0.1.0 Roadmap](.cursor/roadmap/v0.1.0/overview.md) - Detailed MVP plan

---

## Quick Links

- **Repository:** https://github.com/1he03/Clove2d
- **Documentation:** https://docs.rs/clove2d (future)
- **Crates.io:** https://crates.io/crates/clove2d (future)
- **Examples:** `examples/` directory
- **Issues:** https://github.com/1he03/Clove2d/issues

---

**Last Updated:** November 27, 2025  
**Next Review:** Start of v0.1.0 Phase 1  
**Document Owner:** @1he03
