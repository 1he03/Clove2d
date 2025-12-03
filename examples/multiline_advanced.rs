use clove2d::prelude::*;
use clove2d::text::TextWidth;
use std::time::Instant;

fn main() -> Result<()> {
    let start_time = Instant::now();
    println!("=== Advanced Multi-line Text Tests ===");
    
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
    let y_step = 120.0;
    
    // ========== VERY LONG TEXTS ==========
    println!("\n[Very Long Texts]");
    
    let long_english = "This is the first line of a very long text.\n\
        This is the second line with more content.\n\
        This is the third line continuing the story.\n\
        This is the fourth line adding more details.\n\
        This is the fifth line with additional information.\n\
        This is the sixth line expanding the narrative.\n\
        This is the seventh line providing more context.\n\
        This is the eighth line continuing the flow.";
    
    let long_arabic = "هذا هو السطر الأول من نص طويل جداً.\n\
        هذا هو السطر الثاني مع محتوى أكثر.\n\
        هذا هو السطر الثالث مستمراً في القصة.\n\
        هذا هو السطر الرابع مضيفاً المزيد من التفاصيل.\n\
        هذا هو السطر الخامس مع معلومات إضافية.\n\
        هذا هو السطر السادس موسعاً السرد.\n\
        هذا هو السطر السابع مقدم المزيد من السياق.\n\
        هذا هو السطر الثامن مستمراً في التدفق.";
    
    // Test 1: Very long English text
    let test_start = Instant::now();
    println!("[LONG-1] Very long English (8 lines), Fixed(600), Left");
    canvas.create_layer("long_en_fixed")?
        .draw_text(long_english)
            .font_family("NotoSans")
            .font_size(20.0)
            .color(Color::hex("#000000")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step * 2.5;
    
    // Test 2: Very long Arabic text
    let test_start = Instant::now();
    println!("[LONG-2] Very long Arabic (8 lines), Fixed(600), Left");
    canvas.create_layer("long_ar_fixed")?
        .draw_text(long_arabic)
            .font_family("Tajawal")
            .font_size(20.0)
            .color(Color::hex("#2C3E50")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step * 2.5;
    
    // ========== EDGE CASES ==========
    println!("\n[Edge Cases]");
    
    // Test 3: Single character per line
    let test_start = Instant::now();
    println!("[EDGE-1] Single character per line");
    canvas.create_layer("edge_single_char")?
        .draw_text("A\nB\nC\nD\nE")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#E74C3C")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(100.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 4: Empty lines
    let test_start = Instant::now();
    println!("[EDGE-2] Empty lines");
    canvas.create_layer("edge_empty")?
        .draw_text("Line 1\n\nLine 3\n\nLine 5")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#9B59B6")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(300.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 5: Very wide single line
    let test_start = Instant::now();
    println!("[EDGE-3] Very wide single line");
    canvas.create_layer("edge_wide")?
        .draw_text("This is a very wide line that should wrap properly when the width constraint is applied to ensure proper text layout and rendering.")
            .font_family("NotoSans")
            .font_size(20.0)
            .color(Color::hex("#2ECC71")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(400.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step * 2.0;
    
    // ========== DIFFERENT ALIGNMENTS ==========
    println!("\n[Different Alignments]");
    
    let alignment_text = "This is a multi-line text\n\
        With multiple lines\n\
        To test alignment\n\
        Across different settings.";
    
    // Test 6: Long text with Left alignment
    let test_start = Instant::now();
    println!("[ALIGN-1] Long text, Fixed(600), Left");
    canvas.create_layer("align_long_left")?
        .draw_text(alignment_text)
            .font_family("NotoSans")
            .font_size(22.0)
            .color(Color::hex("#E74C3C")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 7: Long text with Right alignment
    let test_start = Instant::now();
    println!("[ALIGN-2] Long text, Fixed(600), Right");
    canvas.create_layer("align_long_right")?
        .draw_text(alignment_text)
            .font_family("NotoSans")
            .font_size(22.0)
            .color(Color::hex("#3498DB")?)
            .position(50.0, y_pos)
            .align(TextAlign::Right)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 8: Long text with Center alignment
    let test_start = Instant::now();
    println!("[ALIGN-3] Long text, Fixed(600), Center");
    canvas.create_layer("align_long_center")?
        .draw_text(alignment_text)
            .font_family("NotoSans")
            .font_size(22.0)
            .color(Color::hex("#2ECC71")?)
            .position(50.0, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // Test 9: Long Arabic text with different alignments
    let alignment_arabic = "هذا نص متعدد الأسطر\n\
        مع عدة أسطر\n\
        لاختبار المحاذاة\n\
        عبر إعدادات مختلفة.";
    
    let test_start = Instant::now();
    println!("[ALIGN-4] Long Arabic text, Fixed(600), Left");
    canvas.create_layer("align_long_ar_left")?
        .draw_text(alignment_arabic)
            .font_family("Tajawal")
            .font_size(22.0)
            .color(Color::hex("#E67E22")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    let test_start = Instant::now();
    println!("[ALIGN-5] Long Arabic text, Fixed(600), Right");
    canvas.create_layer("align_long_ar_right")?
        .draw_text(alignment_arabic)
            .font_family("Tajawal")
            .font_size(22.0)
            .color(Color::hex("#16A085")?)
            .position(50.0, y_pos)
            .align(TextAlign::Right)
            .width(TextWidth::Max(600.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    y_pos += y_step;
    
    // ========== PERFORMANCE TEST ==========
    println!("\n[Performance Test]");
    
    // Test 10: Large text block
    let test_start = Instant::now();
    println!("[PERF-1] Large text block (15 lines)");
    let large_text = (1..=15)
        .map(|i| format!("Line {} with some content to make it longer", i))
        .collect::<Vec<_>>()
        .join("\n");
    
    canvas.create_layer("perf_large")?
        .draw_text(&large_text)
            .font_family("NotoSans")
            .font_size(18.0)
            .color(Color::hex("#34495E")?)
            .position(50.0, y_pos)
            .align(TextAlign::Left)
            .width(TextWidth::Max(800.0))
            .draw()?;
    eprintln!("  Time: {:?}", test_start.elapsed());
    
    println!("\n[Saving] Saving advanced multi-line image...");
    let save_start = Instant::now();
    canvas.save("saves/multiline-advanced.png")?;
    eprintln!("Save time: {:?}", save_start.elapsed());
    println!("✓ Saved to saves/multiline-advanced.png");
    
    let total_time = start_time.elapsed();
    println!("\n=== Test Summary ===");
    println!("Total tests: 10 (Long texts + Edge cases + Alignments + Performance)");
    println!("Canvas size: 1200x1500");
    println!("Total time: {:?}", total_time);
    
    Ok(())
}
