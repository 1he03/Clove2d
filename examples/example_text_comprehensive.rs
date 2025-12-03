use clove2d::prelude::*;

fn main() -> Result<()> {
    println!("=== Example: Comprehensive Text Showcase ===");
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    font_manager.load("Cairo", "fonts/Cairo-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1600, 1400)
        .background(Color::hex("#FFFFFF")?)
        .font_manager(font_manager)
        .build()?;
    
    let layer = canvas.create_layer("text_comprehensive")?;
    
    // Section 1: English Text - Different Sizes
    println!("Drawing English Text - Different Sizes...");
    let english_texts = [
        ("Small Text", 12.0),
        ("Normal Text", 16.0),
        ("Large Text", 24.0),
        ("Extra Large", 36.0),
        ("Huge Text", 48.0),
    ];
    
    for (i, (text, size)) in english_texts.iter().enumerate() {
        layer.draw_text(text)
            .font_family("NotoSans")
            .font_size(*size)
            .color(Color::named(NamedColor::Black))
            .position(100.0, 50.0 + i as f32 * 80.0)
            .align(TextAlign::Left)
            .draw()?;
    }
    
    // Section 2: Arabic Text - Different Sizes
    println!("Drawing Arabic Text - Different Sizes...");
    let arabic_texts = [
        ("نص صغير", 12.0),
        ("نص عادي", 16.0),
        ("نص كبير", 24.0),
        ("نص كبير جداً", 36.0),
        ("نص ضخم", 48.0),
    ];
    
    for (i, (text, size)) in arabic_texts.iter().enumerate() {
        layer.draw_text(text)
            .font_family("Tajawal")
            .font_size(*size)
            .color(Color::hex("#2C3E50")?)
            .position(500.0, 50.0 + i as f32 * 80.0)
            .align(TextAlign::Right)
            .draw()?;
    }
    
    // Section 3: Text Alignments
    println!("Drawing Text Alignments...");
    let alignments = [
        (TextAlign::Left, "Left Aligned", 100.0),
        (TextAlign::Center, "Center Aligned", 600.0),
        (TextAlign::Right, "Right Aligned", 1100.0),
    ];
    
    for (i, (align, text, x)) in alignments.iter().enumerate() {
        layer.draw_rect()
            .position(*x - 150.0, 500.0 + i as f32 * 100.0)
            .size(300.0, 60.0)
            .stroke(Color::hex("#CCCCCC")?, 1.0)
            .draw()?;
        
        layer.draw_text(text)
            .font_family("NotoSans")
            .font_size(20.0)
            .color(Color::hex("#333333")?)
            .position(*x, 530.0 + i as f32 * 100.0)
            .align(*align)
            .width(TextWidth::Max(300.0))
            .draw()?;
    }
    
    // Section 4: Text Width Options
    println!("Drawing Text Width Options...");
    
    let test_text = "This is a long text that will demonstrate different width options in Clove2d text rendering system.";
    
    // TextWidth::None
    layer.draw_text("TextWidth::None")
        .font_family("NotoSans")
        .font_size(14.0)
        .color(Color::hex("#666666")?)
        .position(100.0, 850.0)
        .align(TextAlign::Left)
        .draw()?;
    
    layer.draw_text(test_text)
        .font_family("NotoSans")
        .font_size(16.0)
        .color(Color::named(NamedColor::Black))
        .position(100.0, 880.0)
        .align(TextAlign::Left)
        .width(TextWidth::None)
        .draw()?;
    
    // TextWidth::Max
    layer.draw_text("TextWidth::Max(400)")
        .font_family("NotoSans")
        .font_size(14.0)
        .color(Color::hex("#666666")?)
        .position(100.0, 950.0)
        .align(TextAlign::Left)
        .draw()?;
    
    layer.draw_rect()
        .position(100.0, 980.0)
        .size(400.0, 80.0)
        .stroke(Color::hex("#CCCCCC")?, 1.0)
        .draw()?;
    
    layer.draw_text(test_text)
        .font_family("NotoSans")
        .font_size(16.0)
        .color(Color::named(NamedColor::Black))
        .position(100.0, 1010.0)
        .align(TextAlign::Left)
        .width(TextWidth::Max(400.0))
        .draw()?;
    
    // TextWidth::FullPage
    layer.draw_text("TextWidth::FullPage")
        .font_family("NotoSans")
        .font_size(14.0)
        .color(Color::hex("#666666")?)
        .position(100.0, 1100.0)
        .align(TextAlign::Left)
        .draw()?;
    
    layer.draw_text(test_text)
        .font_family("NotoSans")
        .font_size(16.0)
        .color(Color::named(NamedColor::Black))
        .position(100.0, 1130.0)
        .align(TextAlign::Left)
        .width(TextWidth::FullPage)
        .draw()?;
    
    // Section 5: Multi-line Text
    println!("Drawing Multi-line Text...");
    let multiline_text = "This is a multi-line text example.\nIt demonstrates how text wraps\nacross multiple lines automatically.";
    
    layer.draw_rect()
        .position(600.0, 850.0)
        .size(400.0, 150.0)
        .stroke(Color::hex("#CCCCCC")?, 1.0)
        .draw()?;
    
    layer.draw_text(multiline_text)
        .font_family("NotoSans")
        .font_size(18.0)
        .color(Color::hex("#2C3E50")?)
        .position(600.0, 880.0)
        .align(TextAlign::Left)
        .width(TextWidth::Max(400.0))
        .draw()?;
    
    // Arabic multi-line
    let arabic_multiline = "هذا مثال على نص متعدد الأسطر.\nيظهر كيف يتم التفاف النص\nعبر عدة أسطر تلقائياً.";
    
    layer.draw_rect()
        .position(1100.0, 850.0)
        .size(400.0, 150.0)
        .stroke(Color::hex("#CCCCCC")?, 1.0)
        .draw()?;
    
    layer.draw_text(arabic_multiline)
        .font_family("Tajawal")
        .font_size(18.0)
        .color(Color::hex("#2C3E50")?)
        .position(1500.0, 880.0)
        .align(TextAlign::Right)
        .width(TextWidth::Max(400.0))
        .draw()?;
    
    // Section 6: Font Weights
    println!("Drawing Font Weights...");
    let weights = [
        (FontWeight::Light, "Light Weight"),
        (FontWeight::Normal, "Normal Weight"),
        (FontWeight::Bold, "Bold Weight"),
    ];
    
    for (i, (weight, label)) in weights.iter().enumerate() {
        layer.draw_text(label)
            .font_family("NotoSans")
            .font_size(24.0)
            .font_weight(*weight)
            .color(Color::named(NamedColor::Black))
            .position(600.0, 1050.0 + i as f32 * 50.0)
            .align(TextAlign::Left)
            .draw()?;
    }
    
    // Section 7: Mixed Arabic and English
    println!("Drawing Mixed Arabic and English...");
    let mixed_text = "Hello مرحباً World عالم";
    
    layer.draw_text(mixed_text)
        .font_family("NotoSans")
        .font_size(28.0)
        .color(Color::hex("#E74C3C")?)
        .position(800.0, 1250.0)
        .align(TextAlign::Center)
        .draw()?;
    
    // Section 8: Text Colors
    println!("Drawing Text Colors...");
    let color_texts = [
        (Color::hex("#E74C3C")?, "Red Text"),
        (Color::hex("#3498DB")?, "Blue Text"),
        (Color::hex("#2ECC71")?, "Green Text"),
        (Color::hex("#F39C12")?, "Orange Text"),
        (Color::hex("#9B59B6")?, "Purple Text"),
    ];
    
    for (i, (color, text)) in color_texts.iter().enumerate() {
        layer.draw_text(text)
            .font_family("NotoSans")
            .font_size(20.0)
            .color(color.clone())
            .position(1100.0, 1050.0 + i as f32 * 40.0)
            .align(TextAlign::Left)
            .draw()?;
    }
    
    println!("Saving image...");
    canvas.save("saves/example_text_comprehensive.png")?;
    println!("✓ Saved to saves/example_text_comprehensive.png");
    
    Ok(())
}

