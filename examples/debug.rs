use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Bold.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1200, 800)
        .background(Color::hex("#F5F5F5")?)
        .font_manager(font_manager)
        .build()?;
    
    // Test 1: Linear Gradient
    canvas.create_layer("linear_gradient")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(300.0, 200.0)
            .fill(Color::linear_gradient()
                .start(50.0, 50.0)
                .end(350.0, 250.0)
                .add_stop(0.0, Color::hex("#FF0080")?)
                .add_stop(0.5, Color::hex("#7928CA")?)
                .add_stop(1.0, Color::hex("#0070F3")?)
                .build())
            .draw()?;
    
    // Test 2: Radial Gradient
    canvas.create_layer("radial_gradient")?
        .draw_circle()
            .center(500.0, 150.0)
            .radius(100.0)
            .fill(Color::radial_gradient()
                .center(500.0, 150.0)
                .radius(100.0)
                .add_stop(0.0, Color::hex("#FF0080")?)
                .add_stop(1.0, Color::hex("#0070F3")?)
                .build())
            .draw()?;
    
    // Test 3: Named Colors
    canvas.create_layer("named_colors")?
        .draw_rect()
            .position(50.0, 300.0)
            .size(100.0, 100.0)
            .fill(Color::named(NamedColor::Red))
            .draw()?
        .draw_rect()
            .position(170.0, 300.0)
            .size(100.0, 100.0)
            .fill(Color::named(NamedColor::Green))
            .draw()?
        .draw_rect()
            .position(290.0, 300.0)
            .size(100.0, 100.0)
            .fill(Color::named(NamedColor::Blue))
            .draw()?;
    
    // Test 4: RGBA Colors
    canvas.create_layer("rgba_colors")?
        .draw_rect()
            .position(50.0, 450.0)
            .size(100.0, 100.0)
            .fill(Color::rgba(255, 0, 0, 255))
            .draw()?
        .draw_rect()
            .position(170.0, 450.0)
            .size(100.0, 100.0)
            .fill(Color::rgba(0, 255, 0, 255))
            .draw()?
        .draw_rect()
            .position(290.0, 450.0)
            .size(100.0, 100.0)
            .fill(Color::rgba(0, 0, 255, 255))
            .draw()?;
    
    // Test 5: Text (English)
    canvas.create_layer("text_english")?
        .draw_text("Hello, Clove2d!")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#2C3E50")?)
            .position(500.0, 300.0)
            .align(TextAlign::Left)
            .draw()?;
    
    // Test 6: Text (Arabic)
    canvas.create_layer("text_arabic")?
        .draw_text("مرحبا بك في Clove2d")
            .font_family("Tajawal")
            .font_size(32.0)
            .color(Color::hex("#667EEA")?)
            .position(500.0, 350.0)
            .align(TextAlign::Left)
            .draw()?;
    
    // Test 7: Hex Colors
    canvas.create_layer("hex_colors")?
        .draw_rect()
            .position(50.0, 600.0)
            .size(100.0, 100.0)
            .fill(Color::hex("#FF5733")?)
            .draw()?
        .draw_rect()
            .position(170.0, 600.0)
            .size(100.0, 100.0)
            .fill(Color::hex("#33FF57")?)
            .draw()?
        .draw_rect()
            .position(290.0, 600.0)
            .size(100.0, 100.0)
            .fill(Color::hex("#3357FF")?)
            .draw()?;
    
    canvas.save("saves/debug.png")?;
    println!("Debug example saved to saves/debug.png");
    
    Ok(())
}

