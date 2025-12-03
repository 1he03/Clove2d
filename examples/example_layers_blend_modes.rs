use clove2d::prelude::*;
use clove2d::color::Hsla;

fn main() -> Result<()> {
    println!("=== Example: Layers & Blend Modes Showcase ===");
    
    let mut canvas = Canvas::builder()
        .size(1600, 1200)
        .background(Color::hex("#2C3E50")?)
        .build()?;
    
    // Base layer - background pattern
    println!("Creating base layer...");
    canvas.create_layer("base")?
        .draw_rect()
            .position(0.0, 0.0)
            .size(1600.0, 1200.0)
            .fill(Color::linear_gradient()
                .start(0.0, 0.0)
                .end(1600.0, 1200.0)
                .add_stop(0.0, Color::hex("#34495E")?)
                .add_stop(1.0, Color::hex("#2C3E50")?)
                .build())
            .draw()?;
    
    // Layer 1: Normal blend mode
    println!("Creating layer with Normal blend mode...");
    canvas.create_layer("layer_normal")?
        .draw_circle()
            .center(200.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(255, 100, 100, 200))
            .draw()?
        .blend_mode(BlendMode::Normal)
        .opacity(0.8);
    
    // Layer 2: Multiply blend mode
    println!("Creating layer with Multiply blend mode...");
    canvas.create_layer("layer_multiply")?
        .draw_circle()
            .center(400.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(100, 255, 100, 200))
            .draw()?
        .blend_mode(BlendMode::Multiply)
        .opacity(0.8);
    
    // Layer 3: Screen blend mode
    println!("Creating layer with Screen blend mode...");
    canvas.create_layer("layer_screen")?
        .draw_circle()
            .center(600.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(100, 100, 255, 200))
            .draw()?
        .blend_mode(BlendMode::Screen)
        .opacity(0.8);
    
    // Layer 4: Overlay blend mode
    println!("Creating layer with Overlay blend mode...");
    canvas.create_layer("layer_overlay")?
        .draw_circle()
            .center(800.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(255, 255, 100, 200))
            .draw()?
        .blend_mode(BlendMode::Overlay)
        .opacity(0.8);
    
    // Layer 5: Darken blend mode
    println!("Creating layer with Darken blend mode...");
    canvas.create_layer("layer_darken")?
        .draw_circle()
            .center(1000.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(255, 150, 50, 200))
            .draw()?
        .blend_mode(BlendMode::Darken)
        .opacity(0.8);
    
    // Layer 6: Lighten blend mode
    println!("Creating layer with Lighten blend mode...");
    canvas.create_layer("layer_lighten")?
        .draw_circle()
            .center(1200.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(150, 50, 255, 200))
            .draw()?
        .blend_mode(BlendMode::Lighten)
        .opacity(0.8);
    
    // Layer 7: ColorDodge blend mode
    println!("Creating layer with ColorDodge blend mode...");
    canvas.create_layer("layer_colordodge")?
        .draw_circle()
            .center(200.0, 400.0)
            .radius(100.0)
            .fill(Color::rgba(255, 200, 100, 200))
            .draw()?
        .blend_mode(BlendMode::ColorDodge)
        .opacity(0.8);
    
    // Layer 8: ColorBurn blend mode
    println!("Creating layer with ColorBurn blend mode...");
    canvas.create_layer("layer_colorburn")?
        .draw_circle()
            .center(400.0, 400.0)
            .radius(100.0)
            .fill(Color::rgba(100, 200, 255, 200))
            .draw()?
        .blend_mode(BlendMode::ColorBurn)
        .opacity(0.8);
    
    // Layer 9: HardLight blend mode
    println!("Creating layer with HardLight blend mode...");
    canvas.create_layer("layer_hardlight")?
        .draw_circle()
            .center(600.0, 400.0)
            .radius(100.0)
            .fill(Color::rgba(255, 100, 200, 200))
            .draw()?
        .blend_mode(BlendMode::HardLight)
        .opacity(0.8);
    
    // Layer 10: SoftLight blend mode
    println!("Creating layer with SoftLight blend mode...");
    canvas.create_layer("layer_softlight")?
        .draw_circle()
            .center(800.0, 400.0)
            .radius(100.0)
            .fill(Color::rgba(200, 255, 100, 200))
            .draw()?
        .blend_mode(BlendMode::SoftLight)
        .opacity(0.8);
    
    // Overlapping layers with different opacities
    println!("Creating overlapping layers with different opacities...");
    for i in 0..5 {
        let opacity = 0.3 + (i as f32 * 0.15);
        canvas.create_layer(&format!("opacity_layer_{}", i))?
            .draw_rect()
                .position(1000.0 + i as f32 * 30.0, 400.0 + i as f32 * 30.0)
                .size(200.0, 150.0)
                .fill(Color::rgba(100 + i as u8 * 30, 150 + i as u8 * 20, 200 + i as u8 * 10, 180))
                .draw()?
            .opacity(opacity);
    }
    
    // Layer composition with gradients
    println!("Creating gradient layers...");
    canvas.create_layer("gradient_layer_1")?
        .draw_rect()
            .position(200.0, 600.0)
            .size(300.0, 200.0)
            .fill(Color::linear_gradient()
                .start(200.0, 600.0)
                .end(500.0, 800.0)
                .add_stop(0.0, Color::rgba(255, 0, 0, 150))
                .add_stop(1.0, Color::rgba(0, 0, 255, 150))
                .build())
            .draw()?
        .blend_mode(BlendMode::Multiply)
        .opacity(0.7);
    
    canvas.create_layer("gradient_layer_2")?
        .draw_rect()
            .position(300.0, 650.0)
            .size(300.0, 200.0)
            .fill(Color::radial_gradient()
                .center(450.0, 750.0)
                .radius(150.0)
                .add_stop(0.0, Color::rgba(0, 255, 0, 150))
                .add_stop(1.0, Color::rgba(255, 0, 255, 150))
                .build())
            .draw()?
        .blend_mode(BlendMode::Screen)
        .opacity(0.7);
    
    // Text layer
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    canvas.set_font_manager(font_manager);
    
    println!("Creating text layer...");
    canvas.create_layer("text_layer")?
        .draw_text("Blend Modes Showcase")
            .font_family("NotoSans")
            .font_size(48.0)
            .font_weight(FontWeight::Bold)
            .color(Color::named(NamedColor::White))
            .position(800.0, 600.0)
            .align(TextAlign::Center)
            .draw()?
        .blend_mode(BlendMode::Overlay)
        .opacity(0.9);
    
    // Layer with shapes
    println!("Creating shapes layer...");
    
    canvas.create_layer("shapes_layer")?
        .draw_circle()
            .center(1200.0, 700.0)
            .radius(80.0)
            .fill(Color::rgba(255, 200, 0, 180))
            .draw()?
        .draw_rect()
            .position(1100.0, 800.0)
            .size(200.0, 100.0)
            .fill(Color::rgba(0, 200, 255, 180))
            .draw()?
        .blend_mode(BlendMode::HardLight)
        .opacity(0.8);
    
    // Complex layer composition
    println!("Creating complex composition...");
    for i in 0..8 {
        let angle = i as f32 * 45.0;
        let radius = 150.0;
        let x = 1400.0 + radius * (angle.to_radians().cos());
        let y = 1000.0 + radius * (angle.to_radians().sin());
        
        canvas.create_layer(&format!("rotated_layer_{}", i))?
            .draw_circle()
                .center(x, y)
                .radius(40.0)
                .fill(Color::Hsla(Hsla::new(angle, 1.0, 0.5, 0.7)?))
                .draw()?
            .opacity(0.6);
    }
    
    println!("Saving image...");
    canvas.save("saves/example_layers_blend_modes.png")?;
    println!("✓ Saved to saves/example_layers_blend_modes.png");
    
    Ok(())
}

