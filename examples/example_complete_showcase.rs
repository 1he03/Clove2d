use clove2d::prelude::*;
use clove2d::shapes::*;
use clove2d::geometry::Point;
use clove2d::color::Hsla;

fn main() -> Result<()> {
    println!("=== Example: Complete Showcase - All Features ===");
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    font_manager.load("Cairo", "fonts/Cairo-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(2000, 1600)
        .background(Color::hex("#1A1A2E")?)
        .font_manager(font_manager)
        .build()?;
    
    // Background layer with gradient
    println!("Creating background...");
    canvas.create_layer("background")?
        .draw_rect()
            .position(0.0, 0.0)
            .size(2000.0, 1600.0)
            .fill(Color::linear_gradient()
                .start(0.0, 0.0)
                .end(2000.0, 1600.0)
                .add_stop(0.0, Color::hex("#0F3460")?)
                .add_stop(0.5, Color::hex("#16213E")?)
                .add_stop(1.0, Color::hex("#1A1A2E")?)
                .build())
            .draw()?;
    
    // Create test image first (before using it)
    println!("Creating test image...");
    let mut img_canvas = Canvas::builder()
        .size(300, 300)
        .background(Color::hex("#FFFFFF")?)
        .build()?;
    
    let mut img_font_manager = FontManager::new();
    img_font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    img_canvas.set_font_manager(img_font_manager);
    
    img_canvas.create_layer("img_content")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(200.0, 200.0)
            .fill(Color::radial_gradient()
                .center(150.0, 150.0)
                .radius(100.0)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .draw()?
        .draw_text("Image")
            .font_family("NotoSans")
            .font_size(48.0)
            .font_weight(FontWeight::Bold)
            .color(Color::named(NamedColor::White))
            .position(150.0, 140.0)
            .align(TextAlign::Center)
            .draw()?;
    
    img_canvas.save("saves/showcase_image.png")?;
    
    // Main content layer
    {
        let mut main_layer = canvas.create_layer("main_content")?;
        
        // Section 1: Shapes showcase
        println!("Drawing shapes showcase...");
        
        // Rectangle with gradient
        main_layer.draw_rect()
            .position(100.0, 100.0)
            .size(250.0, 180.0)
            .fill(Color::linear_gradient()
                .start(100.0, 100.0)
                .end(350.0, 280.0)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .stroke(Color::named(NamedColor::White), 3.0)
            .draw()?;
        
        // Circle with radial gradient
        main_layer.draw_circle()
            .center(500.0, 190.0)
            .radius(90.0)
            .fill(Color::radial_gradient()
                .center(500.0, 190.0)
                .radius(90.0)
                .add_stop(0.0, Color::hex("#F093FB")?)
                .add_stop(1.0, Color::hex("#F5576C")?)
                .build())
            .stroke(Color::named(NamedColor::White), 3.0)
            .draw()?;
        
        // Star
        StarBuilder::new(&mut main_layer, Point::new(750.0, 190.0), 80.0, 40.0, 8)
            .fill(Color::hex("#FFD700")?)
            .stroke(Color::named(NamedColor::Orange), 2.0)
            .draw()?;
        
        // Polygon (Hexagon)
        let hex_center = Point::new(1000.0, 190.0);
        let hex_radius = 70.0;
        let mut hex = PolygonBuilder::new(&mut main_layer);
        for i in 0..6 {
            let angle = (i as f32 * 60.0).to_radians();
            hex = hex.add_point(Point::new(
                hex_center.x + hex_radius * angle.cos(),
                hex_center.y + hex_radius * angle.sin(),
            ));
        }
        hex.fill(Color::hex("#4ECDC4")?)
            .stroke(Color::named(NamedColor::Teal), 2.0)
            .draw()?;
        
        // Section 2: Text showcase
        println!("Drawing text showcase...");
        
        // English text
        main_layer.draw_text("Clove2d")
            .font_family("NotoSans")
            .font_size(72.0)
            .font_weight(FontWeight::Bold)
            .color(Color::named(NamedColor::White))
            .position(100.0, 350.0)
            .align(TextAlign::Left)
            .draw()?;
        
        // Arabic text
        main_layer.draw_text("مكتبة رسومات احترافية")
            .font_family("Tajawal")
            .font_size(48.0)
            .color(Color::hex("#FFD700")?)
            .position(1900.0, 350.0)
            .align(TextAlign::Right)
            .draw()?;
        
        // Mixed text
        main_layer.draw_text("Hello مرحباً World عالم")
            .font_family("NotoSans")
            .font_size(36.0)
            .color(Color::hex("#4ECDC4")?)
            .position(1000.0, 450.0)
            .align(TextAlign::Center)
            .draw()?;
        
        // Section 3: Complex shapes
        println!("Drawing complex shapes...");
        
        // Bezier curves
        BezierBuilder::cubic(
            &mut main_layer,
            Point::new(100.0, 600.0),
            Point::new(300.0, 500.0),
            Point::new(500.0, 700.0),
            Point::new(700.0, 600.0),
        )
            .stroke(Color::hex("#FF6B6B")?, 5.0)
            .draw()?;
        
        // Path
        PathBuilder::new(&mut main_layer)
            .move_to(800.0, 600.0)
            .line_to(1000.0, 550.0)
            .line_to(1200.0, 600.0)
            .line_to(1100.0, 650.0)
            .close_path()
            .fill(Color::hex("#9B59B6")?)
            .stroke(Color::named(NamedColor::Purple), 2.0)
            .draw()?;
        
        // Section 6: Gradient patterns
        println!("Drawing gradient patterns...");
        
        // Multiple gradients
        for i in 0..5 {
            let x = 100.0 + i as f32 * 200.0;
            let y = 1200.0;
            
            main_layer.draw_rect()
                .position(x, y)
                .size(150.0, 150.0)
                .fill(Color::linear_gradient()
                    .start(x, y)
                    .end(x + 150.0, y + 150.0)
                    .add_stop(0.0, Color::Hsla(Hsla::new(i as f32 * 72.0, 1.0, 0.5, 1.0)?))
                    .add_stop(1.0, Color::Hsla(Hsla::new((i as f32 * 72.0 + 60.0) % 360.0, 1.0, 0.5, 1.0)?))
                    .build())
                .draw()?;
        }
        
        // Section 7: Complex composition
        println!("Creating complex composition...");
        
        // Overlapping shapes with different opacities
        for i in 0..6 {
            let angle = i as f32 * 60.0;
            let radius = 200.0;
            let x = 1500.0 + radius * (angle.to_radians().cos());
            let y = 800.0 + radius * (angle.to_radians().sin());
            let opacity = 0.3 + (i as f32 * 0.1);
            let alpha = (opacity * 255.0) as u8;
            
            main_layer.draw_circle()
                .center(x, y)
                .radius(80.0)
                .fill(Color::rgba(100 + i as u8 * 25, 150 + i as u8 * 15, 200 + i as u8 * 8, alpha))
                .draw()?;
        }
        
        // Text overlay
        main_layer.draw_text("Complete Showcase")
            .font_family("NotoSans")
            .font_size(64.0)
            .font_weight(FontWeight::Bold)
            .color(Color::named(NamedColor::White))
            .position(1000.0, 1400.0)
            .align(TextAlign::Center)
            .draw()?;
        
        main_layer.draw_text("All Features Combined")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#FFD700")?)
            .position(1000.0, 1480.0)
            .align(TextAlign::Center)
            .draw()?;
        
        // Decorative elements
        for i in 0..20 {
            let angle = i as f32 * 18.0;
            let radius = 250.0;
            let x = 1000.0 + radius * (angle.to_radians().cos());
            let y = 800.0 + radius * (angle.to_radians().sin());
            
            StarBuilder::new(&mut main_layer, Point::new(x, y), 15.0, 7.0, 5)
                .fill(Color::Hsla(Hsla::new(angle, 1.0, 0.6, 0.6)?))
                .draw()?;
        }
        
        // Draw image on main layer (after creating the image)
        let showcase_image = Image::from_path("saves/showcase_image.png")?;
        main_layer.draw_image(showcase_image.clone())
            .position(100.0, 800.0)
            .opacity(1.0)
            .draw()?;
    }
    
    // Section 4: Layers with blend modes
    println!("Creating layers with blend modes...");
    
    {
        let blend_layer_1 = canvas.create_layer("blend_layer_1")?;
        blend_layer_1.draw_circle()
            .center(1400.0, 400.0)
            .radius(120.0)
            .fill(Color::rgba(255, 100, 100, 180))
            .draw()?;
        blend_layer_1.blend_mode(BlendMode::Multiply);
        blend_layer_1.opacity(0.8);
    }
    
    {
        let blend_layer_2 = canvas.create_layer("blend_layer_2")?;
        blend_layer_2.draw_circle()
            .center(1500.0, 400.0)
            .radius(120.0)
            .fill(Color::rgba(100, 255, 100, 180))
            .draw()?;
        blend_layer_2.blend_mode(BlendMode::Screen);
        blend_layer_2.opacity(0.8);
    }
    
    {
        let blend_layer_3 = canvas.create_layer("blend_layer_3")?;
        blend_layer_3.draw_circle()
            .center(1450.0, 350.0)
            .radius(120.0)
            .fill(Color::rgba(100, 100, 255, 180))
            .draw()?;
        blend_layer_3.blend_mode(BlendMode::Overlay);
        blend_layer_3.opacity(0.8);
    }
    
    // Section 5: Image showcase
    println!("Drawing images with filters...");
    
    // Image with filter
    {
        let showcase_image = Image::from_path("saves/showcase_image.png")?;
        let filtered_layer = canvas.create_layer("filtered_image")?;
        filtered_layer.draw_image(showcase_image)
            .position(500.0, 800.0)
            .opacity(1.0)
            .draw()?;
        filtered_layer.apply_filter(Filter::Grayscale)?;
    }
    
    println!("Saving image...");
    canvas.save("saves/example_complete_showcase.png")?;
    println!("✓ Saved to saves/example_complete_showcase.png");
    
    Ok(())
}

