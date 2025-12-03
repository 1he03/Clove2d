use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Bold.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1200, 800)
        .background(Color::hex("#ECEFF1")?)
        .font_manager(font_manager)
        .build()?;
    
    // Gradient background
    canvas.create_layer("background")?
        .draw_rect()
            .position(0.0, 0.0)
            .size(1200.0, 800.0)
            .fill(Color::linear_gradient()
                .start(0.0, 0.0)
                .end(1200.0, 800.0)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .draw()?;
    
    // Card
    canvas.create_layer("card")?
        .draw_rect()
            .position(300.0, 200.0)
            .size(600.0, 400.0)
            .fill(Color::named(NamedColor::White))
            //.corner_radius(20) // Not implemented yet
            //.shadow(Color::rgba(0, 0, 0, 30), 0, 10, 40) // Not implemented yet
            .draw()?
        .draw_text("Clove2d")
            .font_family("NotoSans")
            .font_size(64.0)
            .font_weight(FontWeight::Bold)
            .color(Color::hex("#2C3E50")?)
            .position(600.0, 320.0)
            .align(TextAlign::Center)
            .draw()?
        .draw_text("مكتبة رسومات احترافية")
            .font_family("Tajawal")
            .font_size(32.0)
            .color(Color::hex("#667EEA")?)
            .position(600.0, 400.0)
            .align(TextAlign::Center)
            
            .draw()?
        .draw_circle()
            .center(600.0, 480.0)
            .radius(30.0)
            .fill(Color::hex("#4CAF50")?)
            .draw()?;
    
    canvas.save("saves/7-complex_scene.png")?;
    
    Ok(())
}
