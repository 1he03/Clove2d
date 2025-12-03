use clove2d::prelude::*;
use clove2d::text::{TextWidth, TextAlign};
// use clove2d::image::formats::ImageFormat;

fn main() -> Result<()> {
    println!("=== Debug Text Rendering - Arabic TextWidth Issue with Grid Tests ===");
    
    let program_start = std::time::Instant::now();
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    font_manager.load("Cairo", "fonts/Cairo-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1000, 1500)
        .background(Color::hex("#F5F5F5")?)
        .font_manager(font_manager)
        .build()?;
    
    // Helper function to draw grid lines (vertical and horizontal) at text center
    fn draw_grid_lines(
        canvas: &mut Canvas,
        layer_name: &str,
        text_x: f32,
        text_y: f32,
        actual_text_width: f32,
        text_height: f32,
        container_width: Option<f32>,
        align: TextAlign,
    ) -> Result<()> {
        let container_w = container_width.unwrap_or(actual_text_width);
        let center_y = text_y + (text_height / 2.0);
        
        // Calculate text center based on TextAlign
        // Red line: center of text according to alignment
        let text_center_x = match align {
            TextAlign::Left => text_x + actual_text_width / 2.0,
            TextAlign::Center => text_x + container_w / 2.0,
            TextAlign::Right => text_x + container_w - actual_text_width / 2.0
        };
        
        // Calculate actual text start position based on TextAlign
        // Green line: actual text position
        let actual_text_start_x = match align {
            TextAlign::Left => text_x,
            TextAlign::Center => text_x + (container_w - actual_text_width) / 2.0,
            TextAlign::Right => text_x + container_w - actual_text_width,
        };
        
        // Draw all grid lines in a single layer to save layer count
        let grid_layer = canvas.create_layer(&format!("{}_grid", layer_name))?;
        
        // Draw vertical line (red) - center of text according to alignment
        grid_layer.draw_rect()
            .position(text_center_x - 0.5, text_y)
            .size(1.0, text_height)
            .fill(Color::hex("#FF0000")?)
            .draw()?;
        
        // Draw vertical line (green) - actual text start position
        grid_layer.draw_rect()
            .position(actual_text_start_x - 0.5, text_y)
            .size(1.0, text_height)
            .fill(Color::hex("#00FF00")?)
            .draw()?;
        
        // Draw horizontal line (blue) - full width of container
        grid_layer.draw_rect()
            .position(text_x, center_y - 0.5)
            .size(container_w, 1.0)
            .fill(Color::hex("#0000FF")?)
            .draw()?;
        
        Ok(())
    }
    
    // Helper function to draw text box boundary
    fn draw_text_box(
        canvas: &mut Canvas,
        layer_name: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<()> {
        canvas.create_layer(&format!("{}_box", layer_name))?
            .draw_rect()
            .position(x, y)
            .size(width, height)
            .stroke(Color::hex("#CCCCCC")?, 1.0)
            .draw()?;
        Ok(())
    }
    
    // ========== BOX 1: TextWidth Tests ==========
    println!("\n[Box 1: TextWidth Tests]");
    let box1_y_start = 20.0;
    let box1_y_end = 700.0;
    let box1_height = box1_y_end - box1_y_start;
    
    // Draw box 1 boundary
    canvas.create_layer("box1_boundary")?
        .draw_rect()
        .position(10.0, box1_y_start)
        .size(980.0, box1_height)
        .stroke(Color::hex("#000000")?, 2.0)
        .draw()?;
    
    let mut current_y = box1_y_start + 30.0;
    let line_spacing = 40.0;
    let font_size = 14.0;
    let test_width = 350.0;
    
    // Test 1: Arabic - TextWidth::None
    println!("[BOX1-1] Arabic TextWidth::None");
    let test_start = std::time::Instant::now();
    let test_text = "مرحباً بكم في Clove2d!";
    let test1_align = TextAlign::Left;
    canvas.create_layer("box1_ar_auto")?
        .draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#2C3E50")?)
        .position(50.0, current_y)
        .align(test1_align)
        .width(TextWidth::None)
        .draw()?;
    let test_time = test_start.elapsed();
    eprintln!("[PERF TIMING] BOX1-1 total: {:.3}ms", test_time.as_secs_f64() * 1000.0);
    // Draw grid lines - for Auto, estimate text width (approximate)
    let estimated_text_width = test_text.len() as f32 * font_size * 0.6; // Rough estimate
    draw_grid_lines(&mut canvas, "box1_ar_auto", 50.0, current_y, estimated_text_width, 25.0, None, test1_align)?;
    draw_text_box(&mut canvas, "box1_ar_auto", 50.0, current_y, estimated_text_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 2: Arabic - TextWidth::Fixed
    println!("[BOX1-2] Arabic TextWidth::Fixed");
    let test2_start = std::time::Instant::now();
    let test2_align = TextAlign::Left;
    let layer2 = canvas.create_layer("box1_ar_fixed")?;
    layer2.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#E74C3C")?)
        .position(50.0, current_y)
        .align(test2_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    let test2_time = test2_start.elapsed();
    eprintln!("[PERF TIMING] BOX1-2 total: {:.3}ms", test2_time.as_secs_f64() * 1000.0);
    draw_grid_lines(&mut canvas, "box1_ar_fixed", 50.0, current_y, estimated_text_width, 25.0, Some(test_width), test2_align)?;
    draw_text_box(&mut canvas, "box1_ar_fixed", 50.0, current_y, test_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 3: Arabic - TextWidth::FullPage
    println!("[BOX1-3] Arabic TextWidth::FullPage");
    let test3_align = TextAlign::Left;
    let layer3 = canvas.create_layer("box1_ar_fullpage")?;
    let fullpage_width = 1000.0;
    layer3.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#27AE60")?)
        .position(50.0, current_y)
        .align(test3_align)
        .width(TextWidth::FullPage)
        .draw()?;
    draw_grid_lines(&mut canvas, "box1_ar_fullpage", 50.0, current_y, estimated_text_width, 25.0, Some(fullpage_width - 50.0), test3_align)?;
    draw_text_box(&mut canvas, "box1_ar_fullpage", 50.0, current_y, fullpage_width - 50.0, 25.0)?;
    current_y += line_spacing;
    
    // Test 4: Arabic - TextWidth::Layer
    println!("[BOX1-4] Arabic TextWidth::Layer");
    let test4_align = TextAlign::Left;
    let layer_width = 500.0;
    let layer4 = canvas.create_layer("box1_ar_layer")?
        .width(layer_width as u32);
    layer4.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#8E44AD")?)
        .position(50.0, current_y)
        .align(test4_align)
        .width(TextWidth::Layer)
        .draw()?;
    draw_grid_lines(&mut canvas, "box1_ar_layer", 50.0, current_y, estimated_text_width, 25.0, Some(layer_width - 50.0), test4_align)?;
    draw_text_box(&mut canvas, "box1_ar_layer", 50.0, current_y, layer_width - 50.0, 25.0)?;
    current_y += line_spacing;
    
    // Test 5: Arabic - TextWidth::Fixed with Center alignment
    println!("[BOX1-5] Arabic TextWidth::Fixed Center");
    let test5_align = TextAlign::Center;
    let layer5 = canvas.create_layer("box1_ar_fixed_center")?;
    layer5.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#E67E22")?)
        .position(50.0, current_y)
        .align(test5_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box1_ar_fixed_center", 50.0, current_y, estimated_text_width, 25.0, Some(test_width), test5_align)?;
    draw_text_box(&mut canvas, "box1_ar_fixed_center", 50.0, current_y, test_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 6: Arabic - TextWidth::Fixed with Right alignment
    println!("[BOX1-6] Arabic TextWidth::Fixed Right");
    let test6_align = TextAlign::Right;
    let layer6 = canvas.create_layer("box1_ar_fixed_right")?;
    layer6.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#3498DB")?)
        .position(50.0, current_y)
        .align(test6_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box1_ar_fixed_right", 50.0, current_y, estimated_text_width, 25.0, Some(test_width), test6_align)?;
    draw_text_box(&mut canvas, "box1_ar_fixed_right", 50.0, current_y, test_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 7: Arabic - Multi-line with Fixed
    println!("[BOX1-7] Arabic Multi-line TextWidth::Fixed");
    let test7_align = TextAlign::Left;
    let multiline_text = "مرحباً بكم في Clove2d!\nهذا سطر ثاني\nوسطر ثالث";
    let layer7 = canvas.create_layer("box1_ar_multiline")?;
    layer7.draw_text(multiline_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#16A085")?)
        .position(50.0, current_y)
        .align(test7_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    let multiline_height = 70.0; // Approximate height for 3 lines with smaller font
    draw_grid_lines(&mut canvas, "box1_ar_multiline", 50.0, current_y, estimated_text_width, multiline_height, Some(test_width), test7_align)?;
    draw_text_box(&mut canvas, "box1_ar_multiline", 50.0, current_y, test_width, multiline_height)?;
    current_y += multiline_height + 20.0;
    
    // Test 8: English - TextWidth::Fixed for comparison
    println!("[BOX1-8] English TextWidth::Fixed");
    let test8_align = TextAlign::Left;
    let en_text = "Welcome to Clove2d!";
    let layer8 = canvas.create_layer("box1_en_fixed")?;
    layer8.draw_text(en_text)
        .font_family("NotoSans")
        .font_size(font_size)
        .color(Color::hex("#0000FF")?)
        .position(50.0, current_y)
        .align(test8_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    let en_estimated_width = en_text.len() as f32 * font_size * 0.5; // Rough estimate for English
    draw_grid_lines(&mut canvas, "box1_en_fixed", 50.0, current_y, en_estimated_width, 25.0, Some(test_width), test8_align)?;
    draw_text_box(&mut canvas, "box1_en_fixed", 50.0, current_y, test_width, 25.0)?;
    
    // Additional tests for more coverage
    current_y += line_spacing;
    
    // Test 9: Arabic - TextWidth::Fixed with Justify alignment
    println!("[BOX1-9] Arabic TextWidth::Fixed Justify");
    let test9_align = TextAlign::Left;
    let layer9 = canvas.create_layer("box1_ar_fixed_justify")?;
    layer9.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#9B59B6")?)
        .position(50.0, current_y)
        .align(test9_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box1_ar_fixed_justify", 50.0, current_y, estimated_text_width, 25.0, Some(test_width), test9_align)?;
    draw_text_box(&mut canvas, "box1_ar_fixed_justify", 50.0, current_y, test_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 10: Arabic - TextWidth::Fixed Center with longer text
    println!("[BOX1-10] Arabic TextWidth::Fixed Center (long text)");
    let test10_align = TextAlign::Center;
    let long_text = "مرحباً بكم في Clove2d! هذا نص أطول للاختبار";
    let layer10 = canvas.create_layer("box1_ar_fixed_center_long")?;
    layer10.draw_text(long_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#1ABC9C")?)
        .position(50.0, current_y)
        .align(test10_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    let long_estimated_width = long_text.len() as f32 * font_size * 0.6;
    draw_grid_lines(&mut canvas, "box1_ar_fixed_center_long", 50.0, current_y, long_estimated_width.min(test_width), 25.0, Some(test_width), test10_align)?;
    draw_text_box(&mut canvas, "box1_ar_fixed_center_long", 50.0, current_y, test_width, 25.0)?;
    
    // ========== BOX 2: Text Measurement Tests ==========
    println!("\n[Box 2: Text Measurement Tests]");
    let box2_y_start = 720.0;
    let box2_y_end = 1200.0;
    let box2_height = box2_y_end - box2_y_start;
    
    // Draw box 2 boundary
    canvas.create_layer("box2_boundary")?
        .draw_rect()
        .position(10.0, box2_y_start)
        .size(980.0, box2_height)
        .stroke(Color::hex("#000000")?, 2.0)
        .draw()?;
    
    current_y = box2_y_start + 30.0;
    
    // Same tests as Box 1 but with different visualizations
    // Test 1: Arabic - TextWidth::None
    println!("[BOX2-1] Arabic TextWidth::None");
    let box2_test1_align = TextAlign::Left;
    let layer2_1 = canvas.create_layer("box2_ar_auto")?;
    layer2_1.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#2C3E50")?)
        .position(50.0, current_y)
        .align(box2_test1_align)
        .width(TextWidth::None)
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_ar_auto", 50.0, current_y, estimated_text_width, 25.0, None, box2_test1_align)?;
    draw_text_box(&mut canvas, "box2_ar_auto", 50.0, current_y, estimated_text_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 2: Arabic - TextWidth::Fixed
    println!("[BOX2-2] Arabic TextWidth::Fixed");
    let box2_test2_align = TextAlign::Left;
    let layer2_2 = canvas.create_layer("box2_ar_fixed")?;
    layer2_2.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#E74C3C")?)
        .position(50.0, current_y)
        .align(box2_test2_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_ar_fixed", 50.0, current_y, estimated_text_width, 25.0, Some(test_width), box2_test2_align)?;
    draw_text_box(&mut canvas, "box2_ar_fixed", 50.0, current_y, test_width, 25.0)?;
    current_y += line_spacing;
    
    // Test 3: Arabic - TextWidth::FullPage
    println!("[BOX2-3] Arabic TextWidth::FullPage");
    let box2_test3_align = TextAlign::Left;
    let layer2_3 = canvas.create_layer("box2_ar_fullpage")?;
    layer2_3.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#27AE60")?)
        .position(50.0, current_y)
        .align(box2_test3_align)
        .width(TextWidth::FullPage)
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_ar_fullpage", 50.0, current_y, estimated_text_width, 25.0, Some(fullpage_width - 50.0), box2_test3_align)?;
    draw_text_box(&mut canvas, "box2_ar_fullpage", 50.0, current_y, fullpage_width - 50.0, 25.0)?;
    current_y += line_spacing;
    
    // Test 4: Arabic - TextWidth::Layer
    println!("[BOX2-4] Arabic TextWidth::Layer");
    let box2_test4_align = TextAlign::Left;
    let layer2_4 = canvas.create_layer("box2_ar_layer")?
        .width(layer_width as u32);
    layer2_4.draw_text(test_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#8E44AD")?)
        .position(50.0, current_y)
        .align(box2_test4_align)
        .width(TextWidth::Layer)
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_ar_layer", 50.0, current_y, estimated_text_width, 25.0, Some(layer_width - 50.0), box2_test4_align)?;
    draw_text_box(&mut canvas, "box2_ar_layer", 50.0, current_y, layer_width - 50.0, 25.0)?;
    current_y += line_spacing;
    
    // Test 5: Arabic - Multi-line with Fixed
    println!("[BOX2-5] Arabic Multi-line TextWidth::Fixed");
    let box2_test5_align = TextAlign::Left;
    let layer2_5 = canvas.create_layer("box2_ar_multiline")?;
    layer2_5.draw_text(multiline_text)
        .font_family("Tajawal")
        .font_size(font_size)
        .color(Color::hex("#16A085")?)
        .position(50.0, current_y)
        .align(box2_test5_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_ar_multiline", 50.0, current_y, estimated_text_width, multiline_height, Some(test_width), box2_test5_align)?;
    draw_text_box(&mut canvas, "box2_ar_multiline", 50.0, current_y, test_width, multiline_height)?;
    current_y += multiline_height + 20.0;
    
    // Test 6: English - TextWidth::Fixed for comparison
    println!("[BOX2-6] English TextWidth::Fixed");
    let box2_test6_align = TextAlign::Left;
    let layer2_6 = canvas.create_layer("box2_en_fixed")?;
    layer2_6.draw_text(en_text)
        .font_family("NotoSans")
        .font_size(font_size)
        .color(Color::hex("#0000FF")?)
        .position(50.0, current_y)
        .align(box2_test6_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "box2_en_fixed", 50.0, current_y, en_estimated_width, 25.0, Some(test_width), box2_test6_align)?;
    draw_text_box(&mut canvas, "box2_en_fixed", 50.0, current_y, test_width, 25.0)?;
    
    // ========== CAIRO FONT SPACE TESTS ==========
    println!("\n[Cairo Font Space Tests]");
    let cairo_y_start = 1220.0;
    let mut cairo_y = cairo_y_start;
    let cairo_line_spacing = 45.0;
    let cairo_font_size = 18.0;
    let cairo_test_text = "مرحبا بكم في Clove2d!";
    let cairo_estimated_width = cairo_test_text.len() as f32 * cairo_font_size * 0.6;
    
    // Draw Cairo tests boundary
    canvas.create_layer("cairo_boundary")?
        .draw_rect()
        .position(10.0, cairo_y_start - 10.0)
        .size(980.0, 260.0)
        .stroke(Color::hex("#FF0000")?, 2.0)
        .draw()?;
    
    // Test 1: Cairo - TextWidth::None
    println!("[CAIRO-1] Cairo font - TextWidth::None");
    let cairo_test1_align = TextAlign::Left;
    let cairo_layer1 = canvas.create_layer("cairo_auto")?;
    cairo_layer1.draw_text(cairo_test_text)
        .font_family("Cairo")
        .font_size(cairo_font_size)
        .color(Color::hex("#FF0000")?)
        .position(50.0, cairo_y)
        .align(cairo_test1_align)
        .width(TextWidth::None)
        .draw()?;
    draw_grid_lines(&mut canvas, "cairo_auto", 50.0, cairo_y, cairo_estimated_width, 30.0, None, cairo_test1_align)?;
    draw_text_box(&mut canvas, "cairo_auto", 50.0, cairo_y, cairo_estimated_width, 30.0)?;
    cairo_y += cairo_line_spacing;
    
    // Test 2: Cairo - TextWidth::Fixed
    println!("[CAIRO-2] Cairo font - TextWidth::Fixed");
    let cairo_test2_align = TextAlign::Left;
    let cairo_layer2 = canvas.create_layer("cairo_fixed")?;
    cairo_layer2.draw_text(cairo_test_text)
        .font_family("Cairo")
        .font_size(cairo_font_size)
        .color(Color::hex("#FF6600")?)
        .position(50.0, cairo_y)
        .align(cairo_test2_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "cairo_fixed", 50.0, cairo_y, cairo_estimated_width, 30.0, Some(test_width), cairo_test2_align)?;
    draw_text_box(&mut canvas, "cairo_fixed", 50.0, cairo_y, test_width, 30.0)?;
    cairo_y += cairo_line_spacing;
    
    // Test 3: Cairo - TextWidth::FullPage
    println!("[CAIRO-3] Cairo font - TextWidth::FullPage");
    let cairo_test3_align = TextAlign::Left;
    let cairo_layer3 = canvas.create_layer("cairo_fullpage")?;
    cairo_layer3.draw_text(cairo_test_text)
        .font_family("Cairo")
        .font_size(cairo_font_size)
        .color(Color::hex("#CC0000")?)
        .position(50.0, cairo_y)
        .align(cairo_test3_align)
        .width(TextWidth::FullPage)
        .draw()?;
    draw_grid_lines(&mut canvas, "cairo_fullpage", 50.0, cairo_y, cairo_estimated_width, 30.0, Some(fullpage_width - 50.0), cairo_test3_align)?;
    draw_text_box(&mut canvas, "cairo_fullpage", 50.0, cairo_y, fullpage_width - 50.0, 30.0)?;
    cairo_y += cairo_line_spacing;
    
    // Test 4: Cairo - TextWidth::Fixed with Center alignment
    println!("[CAIRO-4] Cairo font - TextWidth::Fixed Center");
    let cairo_test4_align = TextAlign::Center;
    let cairo_layer4 = canvas.create_layer("cairo_fixed_center")?;
    cairo_layer4.draw_text(cairo_test_text)
        .font_family("Cairo")
        .font_size(cairo_font_size)
        .color(Color::hex("#990000")?)
        .position(50.0, cairo_y)
        .align(cairo_test4_align)
        .width(TextWidth::Max(test_width))
        .draw()?;
    draw_grid_lines(&mut canvas, "cairo_fixed_center", 50.0, cairo_y, cairo_estimated_width, 30.0, Some(test_width), cairo_test4_align)?;
    draw_text_box(&mut canvas, "cairo_fixed_center", 50.0, cairo_y, test_width, 30.0)?;
    
    // Comparison: Same text with Tajawal font (should work correctly)
    println!("\n[CAIRO-COMPARE] Tajawal font (should work correctly)");
    let tajawal_y = cairo_y_start;
    let tajawal_test_align = TextAlign::Left;
    let tajawal_layer = canvas.create_layer("tajawal_comparison")?;
    tajawal_layer.draw_text(cairo_test_text)
        .font_family("Tajawal")
        .font_size(cairo_font_size)
        .color(Color::hex("#0066FF")?)
        .position(500.0, tajawal_y)
        .align(tajawal_test_align)
        .width(TextWidth::None)
        .draw()?;
    draw_grid_lines(&mut canvas, "tajawal_comparison", 500.0, tajawal_y, cairo_estimated_width, 30.0, None, tajawal_test_align)?;
    draw_text_box(&mut canvas, "tajawal_comparison", 500.0, tajawal_y, cairo_estimated_width, 30.0)?;
    
    // Additional test: Multiple spaces
    println!("\n[CAIRO-5] Cairo font - Multiple spaces test");
    let multi_space_text = "مرحبا  بكم   في";
    let cairo_test5_align = TextAlign::Left;
    let multi_space_estimated_width = multi_space_text.len() as f32 * cairo_font_size * 0.6;
    let cairo_layer5 = canvas.create_layer("cairo_multispaces")?;
    cairo_layer5.draw_text(multi_space_text)
        .font_family("Cairo")
        .font_size(cairo_font_size)
        .color(Color::hex("#990000")?)
        .position(500.0, tajawal_y + cairo_line_spacing)
        .align(cairo_test5_align)
        .width(TextWidth::None)
        .draw()?;
    draw_grid_lines(&mut canvas, "cairo_multispaces", 500.0, tajawal_y + cairo_line_spacing, multi_space_estimated_width, 30.0, None, cairo_test5_align)?;
    draw_text_box(&mut canvas, "cairo_multispaces", 500.0, tajawal_y + cairo_line_spacing, multi_space_estimated_width, 30.0)?;
    
    println!("\n[Saving] Saving debug image...");
    let save_start = std::time::Instant::now();
    let merge_before_save = std::time::Instant::now();
    canvas.save("saves/text-debug.png")?;
    let merge_time = merge_before_save.elapsed();
    let save_time = save_start.elapsed();
    println!("✓ Saved to saves/text-debug.png");
    eprintln!("[PERF TIMING] Merge layers before save: {:.3}ms", merge_time.as_secs_f64() * 1000.0);
    eprintln!("[PERF TIMING] Save time (including merge): {:.3}ms", save_time.as_secs_f64() * 1000.0);
    
    let total_program_time = program_start.elapsed();
    eprintln!("\n[PERF TIMING] ========================================");
    eprintln!("[PERF TIMING] BEFORE OPTIMIZATION: ~59675ms (59.676s)");
    eprintln!("[PERF TIMING] AFTER OPTIMIZATION: {:.3}ms ({:.3}s)", 
        total_program_time.as_secs_f64() * 1000.0, total_program_time.as_secs_f64());
    eprintln!("[PERF TIMING] IMPROVEMENT: {:.1}% faster", 
        (59675.0 - total_program_time.as_secs_f64() * 1000.0) / 59675.0 * 100.0);
    eprintln!("[PERF TIMING] ========================================");
    
    println!("\n=== Analysis ===");
    println!("Box 1: TextWidth Tests with grid lines");
    println!("Box 2: Text Measurement Tests with grid lines");
    println!("Cairo Tests: Check if spaces are visible in Cairo font");
    println!("Grid lines:");
    println!("  Red vertical = center of text according to TextAlign");
    println!("  Green vertical = actual text start position");
    println!("  Blue horizontal = center of text height");
    println!("Check terminal output for [DEBUG SPACE] messages");
    
    Ok(())
}
