use clove2d::prelude::*;
use clove2d::text::TextWidth;
use std::time::Instant;

fn main() -> Result<()> {
    let start_time = Instant::now();
    println!("=== Comprehensive Multi-line Text Tests ===");
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    
    // Reduced canvas size: 1200x1500
    let mut canvas = Canvas::builder()
        .size(1200, 1500)
        .background(Color::hex("#F5F5F5")?)
        .font_manager(font_manager)
        .build()?;
    
    let mut y_pos = 50.0;
    let y_step = 100.0;
    
    // ========== ENGLISH MULTI-LINE TESTS ==========
    println!("\n[English Multi-line Tests]");
    
    let english_texts = vec![
        "First line\nSecond line\nThird line",
        "Line one with some content\nLine two with more content\nLine three with even more content",
    ];
    
    // Test 1: English - TextWidth::None with multi-line
    let test_start = Instant::now();
    println!("[EN-ML-1] TextWidth::None, Multi-line, Left");
    canvas.create_layer("en_ml_auto_left")?
        .draw_text(english_texts[0])
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#000000")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 2: English - TextWidth::Fixed with multi-line
    let test_start = Instant::now();
    println!("[EN-ML-2] TextWidth::Max(500), Multi-line, Left");
    canvas.create_layer("en_ml_fixed_left")?
        .draw_text(english_texts[1])
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#0000FF")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(500.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 3: English - TextWidth::FullPage with multi-line
    let test_start = Instant::now();
    println!("[EN-ML-3] TextWidth::FullPage, Multi-line, Center");
    canvas.create_layer("en_ml_fullpage_center")?
        .draw_text(english_texts[1])
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#0066CC")?)
            .position(0.0, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::FullPage)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 4: English - TextWidth::Layer with multi-line
    let test_start = Instant::now();
    println!("[EN-ML-4] TextWidth::Layer, Multi-line, Left");
    canvas.create_layer("en_ml_layer_left")?
        .width(600)
        .draw_text(english_texts[0])
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#00CC66")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step * 2.0;
    
    // ========== ARABIC MULTI-LINE TESTS ==========
    println!("\n[Arabic Multi-line Tests]");
    
    let arabic_texts = vec![
        "السطر الأول\nالسطر الثاني\nالسطر الثالث",
        "السطر الأول مع بعض المحتوى\nالسطر الثاني مع محتوى أكثر\nالسطر الثالث مع محتوى أكثر بكثير",
    ];
    
    // Test 5: Arabic - TextWidth::None with multi-line
    let test_start = Instant::now();
    println!("[AR-ML-1] TextWidth::None, Multi-line, Left");
    canvas.create_layer("ar_ml_auto_left")?
        .draw_text(arabic_texts[0])
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#2C3E50")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 6: Arabic - TextWidth::Fixed with multi-line
    let test_start = Instant::now();
    println!("[AR-ML-2] TextWidth::Max(500), Multi-line, Left");
    canvas.create_layer("ar_ml_fixed_left")?
        .draw_text(arabic_texts[1])
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#E74C3C")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(500.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 7: Arabic - TextWidth::FullPage with multi-line
    let test_start = Instant::now();
    println!("[AR-ML-3] TextWidth::FullPage, Multi-line, Center");
    canvas.create_layer("ar_ml_fullpage_center")?
        .draw_text(arabic_texts[1])
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#16A085")?)
            .position(0.0, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::FullPage)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 8: Arabic - TextWidth::Layer with multi-line
    let test_start = Instant::now();
    println!("[AR-ML-4] TextWidth::Layer, Multi-line, Left");
    canvas.create_layer("ar_ml_layer_left")?
        .width(600)
        .draw_text(arabic_texts[0])
            .font_family("Tajawal")
            .font_size(24.0)
            .color(Color::hex("#8E44AD")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Layer)
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step * 2.0;
    
    // ========== DIFFERENT FONT SIZES ==========
    println!("\n[Different Font Sizes]");
    
    let font_sizes = vec![20.0, 24.0, 28.0];
    let mut x_pos = 50.0;
    
    for (i, size) in font_sizes.iter().enumerate() {
        let test_start = Instant::now();
        println!("[FONT-SIZE-{}] Font size: {}", i + 1, size);
        canvas.create_layer(&format!("font_size_{}", i))?
            .draw_text(&format!("Size {}\nLine two\nLine three", size))
                .font_family("NotoSans")
                .font_size(*size)
                .color(Color::hex("#000000")?)
                .position(x_pos, y_pos)
                .align(TextAlign::Left)
                .width(TextWidth::Max(300.0))
                .draw()?;
        eprintln!("  Time: {:?}", test_start.elapsed());
        x_pos += 350.0;
        if x_pos > 1000.0 {
            x_pos = 50.0;
            y_pos += y_step * 1.5;
        }
    }
    y_pos += y_step * 2.0;
    
    // ========== DIFFERENT POSITIONS ==========
    println!("\n[Different Positions]");
    
    let positions = vec![0.0, 50.0, 200.0];
    
    for (i, pos) in positions.iter().enumerate() {
        let test_start = Instant::now();
        println!("[POS-{}] Position x: {}", i + 1, pos);
        canvas.create_layer(&format!("pos_{}", i))?
            .draw_text("Multi-line text\nSecond line\nThird line")
                .font_family("NotoSans")
                .font_size(20.0)
                .color(Color::hex("#333333")?)
                .position(*pos, y_pos)
                .align(TextAlign::Left)
                .width(TextWidth::Max(300.0))
                .draw()?;
        eprintln!("  Time: {:?}", test_start.elapsed());
        y_pos += y_step;
    }
    
    println!("\n[Saving] Saving comprehensive multi-line image...");
    let save_start = Instant::now();
    canvas.save("saves/multiline-comprehensive.png")?;
    eprintln!("Save time: {:?}", save_start.elapsed());
    println!("✓ Saved to saves/multiline-comprehensive.png");
    
    let total_time = start_time.elapsed();
    println!("\n=== Test Summary ===");
    println!("Total tests: 12 (English + Arabic + Font sizes + Positions)");
    println!("Canvas size: 1200x1500");
    println!("Total time: {:?}", total_time);
    
    Ok(())
}
