# Clove2d

A powerful 2D graphics library for Rust with support for shapes, text, images, filters, and layers.

## Features

- 🎨 **Rich Shape Support**: Rectangles, circles, ellipses, polygons, stars, arcs, bezier curves, and custom paths
- 📝 **Text Rendering**: Support for multiple fonts, bidirectional text (Arabic, Hebrew, etc.), and text styling
- 🖼️ **Image Support**: Load, manipulate, and apply filters to images
- 🎭 **Layers & Blend Modes**: Multiple layers with various blend modes (Normal, Multiply, Screen, Overlay, etc.)
- 🌈 **Gradients**: Linear and radial gradients with multiple color stops
- 🎨 **Filters**: Blur, grayscale, brightness, contrast, and more
- 🔧 **Transformations**: Scale, rotate, translate, and skew

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
clove2d = "0.1.1"
```

## Example

```rust
use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::hex("#1A1A1A")?)
        .build()?;
    
    canvas.create_layer("shapes")?
        .draw_rect()
            .position(100.0, 100.0)
            .size(200.0, 150.0)
            .fill(Color::hex("#FF6B6B")?)
            .stroke(Color::named(NamedColor::White), 2.0)
            .draw()?
        .draw_circle()
            .center(400.0, 200.0)
            .radius(75.0)
            .fill(Color::linear_gradient()
                .start(400.0, 125.0)
                .end(400.0, 275.0)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .draw()?;
    
    canvas.save("output.png")?;
    Ok(())
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

