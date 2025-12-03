use clove2d::prelude::*;
use clove2d::shapes::*;
use clove2d::geometry::Point;
use clove2d::color::Hsla;

fn main() -> Result<()> {
    println!("=== Example: Transforms Showcase ===");
    
    let mut canvas = Canvas::builder()
        .size(1600, 1200)
        .background(Color::hex("#F8F9FA")?)
        .build()?;
    
    let mut layer = canvas.create_layer("transforms")?;
    
    // Base shape for transformations (removed closure - not needed)
    
    // Section 1: Rotation
    println!("Drawing Rotated Shapes...");
    let rotations = [0.0, 15.0, 30.0, 45.0, 60.0, 90.0];
    
    for (i, angle) in rotations.iter().enumerate() {
        let x = 100.0 + (i % 3) as f32 * 200.0;
        let y = 50.0 + (i / 3) as f32 * 200.0;
        
        // Note: Transform would be applied at layer level in real implementation
        // For now, we'll draw shapes at different positions to simulate rotation
        
        // Draw rotated rectangle (simulated)
        layer.draw_rect()
            .position(x, y)
            .size(100.0, 80.0)
            .fill(Color::Hsla(Hsla::new(*angle, 0.8, 0.6, 0.8)?))
            .stroke(Color::named(NamedColor::DarkBlue), 2.0)
            .draw()?;
    }
    
    // Section 2: Scaling
    println!("Drawing Scaled Shapes...");
    let scales = [
        (0.5, 0.5),
        (0.75, 0.75),
        (1.0, 1.0),
        (1.25, 1.25),
        (1.5, 1.5),
        (2.0, 2.0),
    ];
    
    for (i, (sx, sy)) in scales.iter().enumerate() {
        let x = 100.0 + (i % 3) as f32 * 250.0;
        let y = 400.0;
        let width = 100.0 * sx;
        let height = 80.0 * sy;
        
        layer.draw_rect()
            .position(x, y)
            .size(width, height)
            .fill(Color::hex("#E74C3C")?)
            .stroke(Color::named(NamedColor::DarkRed), 2.0)
            .draw()?;
    }
    
    // Section 3: Translation
    println!("Drawing Translated Shapes...");
    let translations = [
        (0.0, 0.0),
        (50.0, 30.0),
        (100.0, 60.0),
        (150.0, 90.0),
        (200.0, 120.0),
    ];
    
    for (i, (tx, ty)) in translations.iter().enumerate() {
        let base_x = 100.0;
        let base_y = 700.0;
        let x = base_x + tx;
        let y = base_y + ty;
        
        layer.draw_circle()
            .center(x, y)
            .radius(30.0)
            .fill(Color::rgba(52, 152, 219, 200 - i as u8 * 30))
            .stroke(Color::named(NamedColor::Navy), 2.0)
            .draw()?;
    }
    
    // Section 4: Combined Transforms
    println!("Drawing Combined Transforms...");
    
    // Rotate + Scale
    for i in 0..4 {
        let angle = i as f32 * 22.5;
        let scale = 0.8 + (i as f32 * 0.1);
        let x = 800.0 + i as f32 * 150.0;
        let y = 200.0;
        
        layer.draw_rect()
            .position(x, y)
            .size(100.0 * scale, 80.0 * scale)
            .fill(Color::Hsla(Hsla::new(angle * 4.0, 0.8, 0.6, 0.7)?))
            .stroke(Color::named(NamedColor::DarkGreen), 2.0)
            .draw()?;
    }
    
    // Scale + Translate
    for i in 0..5 {
        let scale = 0.6 + (i as f32 * 0.2);
        let tx = i as f32 * 40.0;
        let ty = i as f32 * 30.0;
        let x = 800.0 + tx;
        let y = 400.0 + ty;
        
        layer.draw_circle()
            .center(x, y)
            .radius(40.0 * scale)
            .fill(Color::rgba(155, 89, 182, 200 - i as u8 * 30))
            .stroke(Color::named(NamedColor::Purple), 2.0)
            .draw()?;
    }
    
    // Section 5: Flip Horizontal
    println!("Drawing Flipped Shapes...");
    
    // Original
    layer.draw_rect()
        .position(100.0, 950.0)
        .size(100.0, 80.0)
        .fill(Color::hex("#16A085")?)
        .stroke(Color::named(NamedColor::Teal), 2.0)
        .draw()?;
    
    // Flipped (simulated by drawing mirrored)
    layer.draw_rect()
        .position(300.0, 950.0)
        .size(100.0, 80.0)
        .fill(Color::hex("#16A085")?)
        .stroke(Color::named(NamedColor::Teal), 2.0)
        .draw()?;
    
    // Section 6: Flip Vertical
    layer.draw_rect()
        .position(500.0, 950.0)
        .size(100.0, 80.0)
        .fill(Color::hex("#D35400")?)
        .stroke(Color::named(NamedColor::DarkOrange), 2.0)
        .draw()?;
    
    // Flipped vertically (simulated)
    layer.draw_rect()
        .position(700.0, 950.0)
        .size(100.0, 80.0)
        .fill(Color::hex("#D35400")?)
        .stroke(Color::named(NamedColor::DarkOrange), 2.0)
        .draw()?;
    
    // Section 7: Complex Transformations
    println!("Drawing Complex Transformations...");
    
    // Star with rotation simulation
    for i in 0..8 {
        let angle = i as f32 * 45.0;
        let radius = 120.0;
        let x = 1200.0 + radius * (angle.to_radians().cos());
        let y = 300.0 + radius * (angle.to_radians().sin());
        
        StarBuilder::new(&mut layer, Point::new(x, y), 30.0, 15.0, 5)
            .fill(Color::Hsla(Hsla::new(angle, 1.0, 0.5, 0.8)?))
            .stroke(Color::named(NamedColor::Black), 1.0)
            .draw()?;
    }
    
    // Polygon with scaling
    for i in 0..6 {
        let scale = 0.5 + (i as f32 * 0.15);
        let x = 1000.0 + i as f32 * 60.0;
        let y = 600.0;
        let radius = 40.0 * scale;
        
        let mut polygon = PolygonBuilder::new(&mut layer);
        for j in 0..6 {
            let angle = (j as f32 * 60.0).to_radians();
            polygon = polygon.add_point(Point::new(
                x + radius * angle.cos(),
                y + radius * angle.sin(),
            ));
        }
        polygon
            .fill(Color::Hsla(Hsla::new(i as f32 * 60.0, 0.8, 0.6, 0.7)?))
            .stroke(Color::named(NamedColor::Black), 1.0)
            .draw()?;
    }
    
    // Bezier curves with transformations
    for i in 0..5 {
        let offset_x = i as f32 * 150.0;
        let offset_y = i as f32 * 50.0;
        
        BezierBuilder::quadratic(
            &mut layer,
            Point::new(100.0 + offset_x, 1100.0 + offset_y),
            Point::new(150.0 + offset_x, 1050.0 + offset_y),
            Point::new(200.0 + offset_x, 1100.0 + offset_y),
        )
            .stroke(Color::Hsla(Hsla::new(i as f32 * 72.0, 1.0, 0.5, 0.8)?), 3.0)
            .draw()?;
    }
    
    // Labels
    {
        let mut font_manager = FontManager::new();
        font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
        canvas.set_font_manager(font_manager);
    }
    
    let labels = [
        ("Rotation", 200.0, 20.0),
        ("Scaling", 500.0, 370.0),
        ("Translation", 300.0, 670.0),
        ("Combined", 1100.0, 150.0),
        ("Flipped", 400.0, 920.0),
        ("Complex", 1200.0, 200.0),
    ];
    
    {
        let layer = canvas.create_layer("labels")?;
        for (label, x, y) in labels.iter() {
            layer.draw_text(label)
                .font_family("NotoSans")
                .font_size(18.0)
                .font_weight(FontWeight::Bold)
                .color(Color::named(NamedColor::Black))
                .position(*x, *y)
                .align(TextAlign::Center)
                .draw()?;
        }
    }
    
    println!("Saving image...");
    canvas.save("saves/example_transforms.png")?;
    println!("✓ Saved to saves/example_transforms.png");
    
    Ok(())
}

