use clove2d::prelude::*;
use clove2d::color::Hsla;

fn main() -> Result<()> {
    println!("=== Example: Colors & Gradients Showcase ===");
    
    let mut canvas = Canvas::builder()
        .size(1600, 1200)
        .background(Color::hex("#1A1A1A")?)
        .build()?;
    
    let mut layer = canvas.create_layer("colors_gradients")?;
    
    // Section 1: Named Colors
    println!("Drawing Named Colors...");
    let named_colors = [
        NamedColor::Red, NamedColor::Green, NamedColor::Blue,
        NamedColor::Yellow, NamedColor::Cyan, NamedColor::Magenta,
        NamedColor::Orange, NamedColor::Purple, NamedColor::Pink,
        NamedColor::Brown, NamedColor::Gray, NamedColor::Black,
    ];
    
    for (i, color) in named_colors.iter().enumerate() {
        let x = 50.0 + (i % 4) as f32 * 200.0;
        let y = 50.0 + (i / 4) as f32 * 150.0;
        layer.draw_rect()
            .position(x, y)
            .size(150.0, 100.0)
            .fill(Color::named(*color))
            .draw()?;
    }
    
    // Section 2: RGBA Colors
    println!("Drawing RGBA Colors...");
    let rgba_colors = [
        (255, 0, 0, 255),   // Red
        (0, 255, 0, 255),   // Green
        (0, 0, 255, 255),   // Blue
        (255, 255, 0, 255), // Yellow
        (255, 0, 255, 255), // Magenta
        (0, 255, 255, 255), // Cyan
        (255, 128, 0, 255), // Orange
        (128, 0, 128, 255), // Purple
    ];
    
    for (i, (r, g, b, a)) in rgba_colors.iter().enumerate() {
        let x = 50.0 + (i % 4) as f32 * 200.0;
        let y = 400.0 + (i / 4) as f32 * 150.0;
        layer.draw_rect()
            .position(x, y)
            .size(150.0, 100.0)
            .fill(Color::rgba(*r, *g, *b, *a))
            .draw()?;
    }
    
    // Section 3: Hex Colors
    println!("Drawing Hex Colors...");
    let hex_colors = [
        "#FF6B6B", "#4ECDC4", "#45B7D1", "#FFA07A",
        "#98D8C8", "#F7DC6F", "#BB8FCE", "#85C1E2",
    ];
    
    for (i, hex) in hex_colors.iter().enumerate() {
        let x = 50.0 + (i % 4) as f32 * 200.0;
        let y = 700.0 + (i / 4) as f32 * 150.0;
        layer.draw_rect()
            .position(x, y)
            .size(150.0, 100.0)
            .fill(Color::hex(hex)?)
            .draw()?;
    }
    
    // Section 4: HSLA Colors
    println!("Drawing HSLA Colors...");
    let hsla_colors = [
        (0.0, 1.0, 0.5, 1.0),   // Red
        (120.0, 1.0, 0.5, 1.0), // Green
        (240.0, 1.0, 0.5, 1.0), // Blue
        (60.0, 1.0, 0.5, 1.0),  // Yellow
    ];
    
    for (i, (h, s, l, a)) in hsla_colors.iter().enumerate() {
        let x = 900.0 + (i % 2) as f32 * 200.0;
        let y = 50.0 + (i / 2) as f32 * 150.0;
        let hsla = Hsla::new(*h, *s, *l, *a)?;
        layer.draw_rect()
            .position(x, y)
            .size(150.0, 100.0)
            .fill(Color::Hsla(hsla))
            .draw()?;
    }
    
    // Section 5: Linear Gradients
    println!("Drawing Linear Gradients...");
    
    // Horizontal gradient
    layer.draw_rect()
        .position(900.0, 400.0)
        .size(300.0, 150.0)
        .fill(Color::linear_gradient()
            .start(900.0, 400.0)
            .end(1200.0, 400.0)
            .add_stop(0.0, Color::hex("#FF6B6B")?)
            .add_stop(0.5, Color::hex("#4ECDC4")?)
            .add_stop(1.0, Color::hex("#45B7D1")?)
            .build())
        .draw()?;
    
    // Vertical gradient
    layer.draw_rect()
        .position(1300.0, 400.0)
        .size(150.0, 300.0)
        .fill(Color::linear_gradient()
            .start(1300.0, 400.0)
            .end(1300.0, 700.0)
            .add_stop(0.0, Color::hex("#667EEA")?)
            .add_stop(1.0, Color::hex("#764BA2")?)
            .build())
        .draw()?;
    
    // Diagonal gradient
    layer.draw_rect()
        .position(900.0, 600.0)
        .size(300.0, 150.0)
        .fill(Color::linear_gradient()
            .start(900.0, 600.0)
            .end(1200.0, 750.0)
            .add_stop(0.0, Color::hex("#F093FB")?)
            .add_stop(1.0, Color::hex("#F5576C")?)
            .build())
        .draw()?;
    
    // Multi-stop gradient
    layer.draw_rect()
        .position(1300.0, 750.0)
        .size(250.0, 100.0)
        .fill(Color::linear_gradient()
            .start(1300.0, 750.0)
            .end(1550.0, 750.0)
            .add_stop(0.0, Color::hex("#FF6B6B")?)
            .add_stop(0.25, Color::hex("#FFE66D")?)
            .add_stop(0.5, Color::hex("#4ECDC4")?)
            .add_stop(0.75, Color::hex("#45B7D1")?)
            .add_stop(1.0, Color::hex("#A8E6CF")?)
            .build())
        .draw()?;
    
    // Section 6: Radial Gradients
    println!("Drawing Radial Gradients...");
    
    // Simple radial gradient
    layer.draw_circle()
        .center(1050.0, 950.0)
        .radius(100.0)
        .fill(Color::radial_gradient()
            .center(1050.0, 950.0)
            .radius(100.0)
            .add_stop(0.0, Color::hex("#FF6B6B")?)
            .add_stop(1.0, Color::hex("#4ECDC4")?)
            .build())
        .draw()?;
    
    // Multi-stop radial gradient
    layer.draw_circle()
        .center(1350.0, 950.0)
        .radius(100.0)
        .fill(Color::radial_gradient()
            .center(1350.0, 950.0)
            .radius(100.0)
            .add_stop(0.0, Color::hex("#FF6B6B")?)
            .add_stop(0.3, Color::hex("#FFE66D")?)
            .add_stop(0.6, Color::hex("#4ECDC4")?)
            .add_stop(1.0, Color::hex("#45B7D1")?)
            .build())
        .draw()?;
    
    // Gradient on shapes
    println!("Drawing Gradients on Different Shapes...");
    
    // Ellipse with gradient
    use clove2d::shapes::EllipseBuilder;
    EllipseBuilder::new(&mut layer, 150.0, 1000.0, 80.0, 60.0)
        .fill(Color::linear_gradient()
            .start(70.0, 1000.0)
            .end(230.0, 1000.0)
            .add_stop(0.0, Color::hex("#667EEA")?)
            .add_stop(1.0, Color::hex("#764BA2")?)
            .build())
        .draw()?;
    
    // Triangle with gradient
    use clove2d::shapes::TriangleBuilder;
    use clove2d::geometry::Point;
    TriangleBuilder::new(
        &mut layer,
        Point::new(400.0, 950.0),
        Point::new(500.0, 1050.0),
        Point::new(300.0, 1050.0),
    )
        .fill(Color::radial_gradient()
            .center(400.0, 1000.0)
            .radius(60.0)
            .add_stop(0.0, Color::hex("#F093FB")?)
            .add_stop(1.0, Color::hex("#F5576C")?)
            .build())
        .draw()?;
    
    // Colors with opacity
    println!("Drawing Colors with Opacity...");
    for i in 0..5 {
        let opacity = (i + 1) as f32 * 0.2;
        let alpha = (opacity * 255.0) as u8;
        layer.draw_circle()
            .center(600.0 + i as f32 * 80.0, 1000.0)
            .radius(50.0)
            .fill(Color::rgba(255, 100, 100, alpha))
            .draw()?;
    }
    
    // Gradient combinations
    println!("Drawing Gradient Combinations...");
    
    // Overlapping gradients
    layer.draw_rect()
        .position(50.0, 1100.0)
        .size(200.0, 80.0)
        .fill(Color::linear_gradient()
            .start(50.0, 1100.0)
            .end(250.0, 1100.0)
            .add_stop(0.0, Color::rgba(255, 0, 0, 200))
            .add_stop(1.0, Color::rgba(0, 0, 255, 200))
            .build())
        .draw()?;
    
    layer.draw_rect()
        .position(150.0, 1100.0)
        .size(200.0, 80.0)
        .fill(Color::linear_gradient()
            .start(150.0, 1100.0)
            .end(350.0, 1100.0)
            .add_stop(0.0, Color::rgba(0, 255, 0, 150))
            .add_stop(1.0, Color::rgba(255, 0, 255, 150))
            .build())
        .draw()?;
    
    println!("Saving image...");
    canvas.save("saves/example_colors_gradients.png")?;
    println!("✓ Saved to saves/example_colors_gradients.png");
    
    Ok(())
}

