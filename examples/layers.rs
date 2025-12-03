use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    // Background layer
    canvas.create_layer("background")?
        .draw_rect()
            .position(0.0, 0.0)
            .size(800.0, 600.0)
            .fill(Color::hex("#2C3E50")?)
            .draw()?;
    
    // Shapes layer
    canvas.create_layer("shapes")?
        .draw_circle()
            .center(400.0, 300.0)
            .radius(150.0)
            .fill(Color::rgba(255, 100, 100, 180))
            .draw()?
        .blend_mode(BlendMode::Multiply)
        
        .opacity(0.8);
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    
    // Set font_manager to canvas
    canvas.set_font_manager(font_manager);

    // Text layer
    canvas.create_layer("text")?
        .draw_text("Layered Composition")
            .font_family("NotoSans")
            .font_size(36.0)
            .color(Color::named(NamedColor::White))
            .position(400.0, 50.0)
            .align(TextAlign::Center)
            .draw()?;
    
    canvas.save("saves/6-layers.png")?;
    
    Ok(())
}
