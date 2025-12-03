use clove2d::prelude::*;
use clove2d::text::TextWidth;

fn main() -> Result<()> {
    println!("=== Debug TextWidth::Layer - Arabic Text ===");
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1200, 1000)
        .background(Color::hex("#F5F5F5")?)
        .font_manager(font_manager)
        .build()?;
    
    // ========== ARABIC TextWidth::Layer TESTS ==========
    println!("\n[Arabic TextWidth::Layer Tests]");
    
    // Test 1: Layer without explicit width - Left alignment
    println!("[TEST-1] Layer without width, Left alignment");
    canvas.create_layer("layer1")?
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#E74C3C")?)
            .position(50.0, 50.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 2: Layer without explicit width - Right alignment
    println!("[TEST-2] Layer without width, Right alignment");
    canvas.create_layer("layer2")?
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#3498DB")?)
            .position(50.0, 100.0)
            .align(TextAlign::Right)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 3: Layer without explicit width - Center alignment
    println!("[TEST-3] Layer without width, Center alignment");
    canvas.create_layer("layer3")?
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#2ECC71")?)
            .position(50.0, 150.0)
            .align(TextAlign::Center)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 4: Layer without explicit width - Justify alignment
    println!("[TEST-4] Layer without width, Justify alignment");
    canvas.create_layer("layer4")?
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#9B59B6")?)
            .position(50.0, 200.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 5: Layer with explicit width(600) - Left alignment
    println!("[TEST-5] Layer with width(600), Left alignment");
    canvas.create_layer("layer5")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#E67E22")?)
            .position(50.0, 250.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 6: Layer with explicit width(600) - Right alignment
    println!("[TEST-6] Layer with width(600), Right alignment");
    canvas.create_layer("layer6")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#16A085")?)
            .position(50.0, 300.0)
            .align(TextAlign::Right)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 7: Layer with explicit width(600) - Center alignment
    println!("[TEST-7] Layer with width(600), Center alignment");
    canvas.create_layer("layer7")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#D35400")?)
            .position(50.0, 350.0)
            .align(TextAlign::Center)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 8: Layer with explicit width(600) - Justify alignment
    println!("[TEST-8] Layer with width(600), Justify alignment");
    canvas.create_layer("layer8")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#8E44AD")?)
            .position(50.0, 400.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 9: Layer with explicit width(800) - Left alignment, x=0
    println!("[TEST-9] Layer with width(800), Left alignment, x=0");
    canvas.create_layer("layer9")?
        .width(800)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#C0392B")?)
            .position(0.0, 450.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 10: Layer with explicit width(800) - Right alignment, x=0
    println!("[TEST-10] Layer with width(800), Right alignment, x=0");
    canvas.create_layer("layer10")?
        .width(800)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#27AE60")?)
            .position(0.0, 500.0)
            .align(TextAlign::Right)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 11: Layer with explicit width(800) - Center alignment, x=0
    println!("[TEST-11] Layer with width(800), Center alignment, x=0");
    canvas.create_layer("layer11")?
        .width(800)
        .draw_text("مرحباً بكم في Clove2d!")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#2980B9")?)
            .position(0.0, 550.0)
            .align(TextAlign::Center)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 12: Multi-line Arabic with Layer width(600) - Left alignment
    println!("[TEST-12] Multi-line Arabic with Layer width(600), Left alignment");
    canvas.create_layer("layer12")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!\nهذا سطر ثاني\nوسطر ثالث")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#1ABC9C")?)
            .position(50.0, 600.0)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 13: Multi-line Arabic with Layer width(600) - Right alignment
    println!("[TEST-13] Multi-line Arabic with Layer width(600), Right alignment");
    canvas.create_layer("layer13")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!\nهذا سطر ثاني\nوسطر ثالث")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#F39C12")?)
            .position(50.0, 750.0)
            .align(TextAlign::Right)
            .width(TextWidth::Layer)
            .draw()?;
    
    // Test 14: Multi-line Arabic with Layer width(600) - Center alignment
    println!("[TEST-14] Multi-line Arabic with Layer width(600), Center alignment");
    canvas.create_layer("layer14")?
        .width(600)
        .draw_text("مرحباً بكم في Clove2d!\nهذا سطر ثاني\nوسطر ثالث")
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#E74C3C")?)
            .position(50.0, 900.0)
            .align(TextAlign::Center)
            .width(TextWidth::Layer)
            .draw()?;
    
    // ========== VISUAL MARKERS ==========
    println!("\n[Drawing Visual Markers]");
    
    // Draw layer boundaries
    canvas.create_layer("boundaries")?
        .draw_rect()
            .position(50.0, 250.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 300.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 350.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 400.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(0.0, 450.0)
            .size(800.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(0.0, 500.0)
            .size(800.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(0.0, 550.0)
            .size(800.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 600.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 750.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?
        .draw_rect()
            .position(50.0, 900.0)
            .size(600.0, 1.0)
            .fill(Color::hex("#FF0000")?)
            .draw()?;
    
    // Draw expected text area boxes (yellow) to show where text should be
    canvas.create_layer("expected_areas")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(1200.0, 30.0) // Full layer width (no explicit width)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 100.0)
            .size(1200.0, 30.0) // Full layer width (no explicit width)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 150.0)
            .size(1200.0, 30.0) // Full layer width (no explicit width)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 200.0)
            .size(1200.0, 30.0) // Full layer width (no explicit width)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 250.0)
            .size(600.0, 30.0) // Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 300.0)
            .size(600.0, 30.0) // Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 350.0)
            .size(600.0, 30.0) // Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 400.0)
            .size(600.0, 30.0) // Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(0.0, 450.0)
            .size(800.0, 30.0) // Layer width(800)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(0.0, 500.0)
            .size(800.0, 30.0) // Layer width(800)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(0.0, 550.0)
            .size(800.0, 30.0) // Layer width(800)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 600.0)
            .size(600.0, 90.0) // Multi-line, Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 750.0)
            .size(600.0, 90.0) // Multi-line, Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?
        .draw_rect()
            .position(50.0, 900.0)
            .size(600.0, 90.0) // Multi-line, Layer width(600)
            .stroke(Color::hex("#FFFF00")?, 2.0)
            .draw()?;
    
    // Draw position markers (green dots)
    canvas.create_layer("markers")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 100.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 150.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 200.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 250.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 300.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 350.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 400.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(0.0, 450.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(0.0, 500.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(0.0, 550.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 600.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 750.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?
        .draw_rect()
            .position(50.0, 900.0)
            .size(3.0, 3.0)
            .fill(Color::hex("#00FF00")?)
            .draw()?;
    
    println!("\n[Saving] Saving debug layer image...");
    canvas.save("saves/text-layer-debug.png")?;
    println!("✓ Saved to saves/text-layer-debug.png");
    println!("\n=== Analysis ===");
    println!("Check all Arabic TextWidth::Layer tests - they should all display correctly");
    println!("Left alignment should be at position.x");
    println!("Right alignment should be at position.x + layer_width - text_width");
    println!("Center alignment should be centered within layer_width starting at position.x");
    
    Ok(())
}

