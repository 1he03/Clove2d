use clove2d::prelude::*;
use clove2d::shapes::*;
use clove2d::geometry::Point;

fn main() -> Result<()> {
    println!("=== Example: All Shapes Showcase ===");
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1600, 1200)
        .background(Color::hex("#F5F7FA")?)
        .font_manager(font_manager)
        .build()?;
    
    let mut layer = canvas.create_layer("all_shapes")?;
    
    // 1. Rectangle
    println!("Drawing Rectangle...");
    layer.draw_rect()
        .position(50.0, 50.0)
        .size(150.0, 100.0)
        .fill(Color::hex("#FF6B6B")?)
        .stroke(Color::named(NamedColor::Black), 2.0)
        .draw()?;
    
    // 2. Rounded Rectangle
    println!("Drawing Rounded Rectangle...");
    RoundedRectBuilder::new(&mut layer, 250.0, 50.0, 150.0, 100.0)
        .corner_radius(20.0)
        .fill(Color::hex("#4CAF50")?)
        .stroke(Color::named(NamedColor::DarkGreen), 2.0)
        .draw()?;
    
    // 3. Circle
    println!("Drawing Circle...");
    layer.draw_circle()
        .center(500.0, 100.0)
        .radius(50.0)
        .fill(Color::hex("#2196F3")?)
        .stroke(Color::named(NamedColor::Navy), 3.0)
        .draw()?;
    
    // 4. Ellipse
    println!("Drawing Ellipse...");
    EllipseBuilder::new(&mut layer, 650.0, 100.0, 80.0, 50.0)
        .fill(Color::hex("#FF9800")?)
        .stroke(Color::named(NamedColor::DarkOrange), 2.0)
        .draw()?;
    
    // 5. Line
    println!("Drawing Line...");
    LineBuilder::new(&mut layer, 50.0, 200.0, 200.0, 250.0)
        .stroke(Color::hex("#9C27B0")?, 4.0)
        .draw()?;
    
    // 6. Triangle
    println!("Drawing Triangle...");
    TriangleBuilder::new(
        &mut layer,
        Point::new(300.0, 200.0),
        Point::new(400.0, 300.0),
        Point::new(200.0, 300.0),
    )
        .fill(Color::hex("#E91E63")?)
        .stroke(Color::named(NamedColor::DarkRed), 2.0)
        .draw()?;
    
    // 7. Polygon (Hexagon)
    println!("Drawing Polygon (Hexagon)...");
    let hex_center_x = 550.0;
    let hex_center_y = 250.0;
    let hex_radius = 60.0;
    let mut hex_points = Vec::new();
    for i in 0..6 {
        let angle = (i as f32 * 60.0).to_radians();
        hex_points.push(Point::new(
            hex_center_x + hex_radius * angle.cos(),
            hex_center_y + hex_radius * angle.sin(),
        ));
    }
    let mut polygon = PolygonBuilder::new(&mut layer);
    for point in hex_points {
        polygon = polygon.add_point(point);
    }
    polygon
        .fill(Color::hex("#00BCD4")?)
        .stroke(Color::named(NamedColor::Teal), 2.0)
        .draw()?;
    
    // 8. Polyline (Arrow)
    println!("Drawing Polyline (Arrow)...");
    PolylineBuilder::new(&mut layer)
        .add_point(Point::new(700.0, 200.0))
        .add_point(Point::new(850.0, 250.0))
        .add_point(Point::new(800.0, 250.0))
        .add_point(Point::new(800.0, 300.0))
        .add_point(Point::new(900.0, 300.0))
        .add_point(Point::new(900.0, 200.0))
        .add_point(Point::new(800.0, 200.0))
        .add_point(Point::new(800.0, 250.0))
        .add_point(Point::new(850.0, 250.0))
        .stroke(Color::hex("#795548")?, 3.0)
        .draw()?;
    
    // 9. Star
    println!("Drawing Star...");
    StarBuilder::new(&mut layer, Point::new(1050.0, 250.0), 60.0, 30.0, 5)
        .fill(Color::hex("#FFC107")?)
        .stroke(Color::named(NamedColor::Orange), 2.0)
        .draw()?;
    
    // 10. Arc
    println!("Drawing Arc...");
    ArcBuilder::new(&mut layer, Point::new(1200.0, 250.0), 50.0, 0.0, 270.0)
        .stroke(Color::hex("#607D8B")?, 5.0)
        .draw()?;
    
    // 11. Bezier Curve (Quadratic)
    println!("Drawing Quadratic Bezier...");
    BezierBuilder::quadratic(
        &mut layer,
        Point::new(50.0, 400.0),
        Point::new(200.0, 300.0),
        Point::new(350.0, 400.0),
    )
        .stroke(Color::hex("#8BC34A")?, 4.0)
        .draw()?;
    
    // 12. Bezier Curve (Cubic)
    println!("Drawing Cubic Bezier...");
    BezierBuilder::cubic(
        &mut layer,
        Point::new(400.0, 400.0),
        Point::new(500.0, 300.0),
        Point::new(600.0, 500.0),
        Point::new(700.0, 400.0),
    )
        .stroke(Color::hex("#FF5722")?, 4.0)
        .draw()?;
    
    // 13. Path (Custom shape)
    println!("Drawing Custom Path...");
    PathBuilder::new(&mut layer)
        .move_to(800.0, 400.0)
        .line_to(900.0, 350.0)
        .line_to(1000.0, 400.0)
        .line_to(950.0, 450.0)
        .close_path()
        .fill(Color::hex("#673AB7")?)
        .stroke(Color::named(NamedColor::Purple), 2.0)
        .draw()?;
    
    // Shapes with gradients
    println!("Drawing Shapes with Gradients...");
    
    // Rectangle with Linear Gradient
    layer.draw_rect()
        .position(50.0, 550.0)
        .size(200.0, 150.0)
        .fill(Color::linear_gradient()
            .start(50.0, 550.0)
            .end(250.0, 700.0)
            .add_stop(0.0, Color::hex("#667EEA")?)
            .add_stop(1.0, Color::hex("#764BA2")?)
            .build())
        .draw()?;
    
    // Circle with Radial Gradient
    layer.draw_circle()
        .center(400.0, 625.0)
        .radius(75.0)
        .fill(Color::radial_gradient()
            .center(400.0, 625.0)
            .radius(75.0)
            .add_stop(0.0, Color::hex("#F093FB")?)
            .add_stop(1.0, Color::hex("#F5576C")?)
            .build())
        .draw()?;
    
    // Complex composition
    println!("Drawing Complex Composition...");
    
    // Overlapping shapes with opacity
    layer.draw_circle()
        .center(600.0, 625.0)
        .radius(60.0)
        .fill(Color::rgba(255, 0, 0, 150))
        .draw()?;
    
    layer.draw_circle()
        .center(650.0, 625.0)
        .radius(60.0)
        .fill(Color::rgba(0, 255, 0, 150))
        .draw()?;
    
    layer.draw_circle()
        .center(625.0, 600.0)
        .radius(60.0)
        .fill(Color::rgba(0, 0, 255, 150))
        .draw()?;
    
    // Grid pattern with lines
    println!("Drawing Grid Pattern...");
    for i in 0..10 {
        LineBuilder::new(&mut layer, 800.0 + i as f32 * 20.0, 550.0, 800.0 + i as f32 * 20.0, 700.0)
            .stroke(Color::rgba(0, 0, 0, 30), 1.0)
            .draw()?;
    }
    for i in 0..8 {
        LineBuilder::new(&mut layer, 800.0, 550.0 + i as f32 * 20.0, 1000.0, 550.0 + i as f32 * 20.0)
            .stroke(Color::rgba(0, 0, 0, 30), 1.0)
            .draw()?;
    }
    
    // Text labels
    
    let labels = [
        ("Rectangle", 125.0, 160.0),
        ("Rounded", 325.0, 160.0),
        ("Circle", 500.0, 160.0),
        ("Ellipse", 650.0, 160.0),
        ("Line", 125.0, 320.0),
        ("Triangle", 300.0, 320.0),
        ("Polygon", 550.0, 320.0),
        ("Polyline", 800.0, 320.0),
        ("Star", 1050.0, 320.0),
        ("Arc", 1200.0, 320.0),
        ("Bezier Q", 200.0, 420.0),
        ("Bezier C", 550.0, 420.0),
        ("Path", 900.0, 420.0),
    ];
    
    for (label, x, y) in labels.iter() {
        layer.draw_text(label)
            .font_family("NotoSans")
            .font_size(14.0)
            .color(Color::named(NamedColor::Black))
            .position(*x, *y)
            .align(TextAlign::Center)
            .draw()?;
    }
    
    println!("Saving image...");
    canvas.save("saves/example_all_shapes.png")?;
    println!("✓ Saved to saves/example_all_shapes.png");
    
    Ok(())
}

