use clove2d::prelude::*;
use clove2d::text::TextWidth;

fn main() -> Result<()> {
    let mut font_manager = FontManager::new();
    
    // Load all available fonts
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    font_manager.load("Cairo", "fonts/Cairo-Regular.ttf")?;
    font_manager.load("Rubik", "fonts/Rubik-Regular.ttf")?;
    font_manager.load("Almarai", "fonts/Almarai-Regular.ttf")?;
    font_manager.load("NotoSansArabic", "fonts/NotoSansArabic-Regular.ttf")?;
    font_manager.load("Inconsolata", "fonts/Inconsolata_Condensed-Bold.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1400, 1000)
        .background(Color::hex("#FFFFFF")?)
        .font_manager(font_manager.clone())
        .build()?;
    
    let mut y_pos = 50.0;
    let x_center = 700.0;
    
    // NotoSans
    canvas.create_layer("noto")?
        .draw_text("NotoSans Font - English Text Example")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#2C3E50")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Tajawal
    canvas.create_layer("tajawal")?
        .draw_text("خط Tajawal - مثال على النص العربي")
            .font_family("Tajawal")
            .font_size(32.0)
            .color(Color::hex("#E74C3C")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Cairo
    canvas.create_layer("cairo")?
        .draw_text("خط Cairo - نص تجريبي")
            .font_family("Cairo")
            .font_size(32.0)
            .color(Color::hex("#3498DB")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Rubik
    canvas.create_layer("rubik")?
        .draw_text("Rubik Font - Modern Sans Serif")
            .font_family("Rubik")
            .font_size(32.0)
            .color(Color::hex("#27AE60")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Almarai
    canvas.create_layer("almarai")?
        .draw_text("خط Almarai - خط عربي أنيق")
            .font_family("Almarai")
            .font_size(32.0)
            .color(Color::hex("#8E44AD")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // NotoSansArabic
    canvas.create_layer("noto_arabic")?
        .draw_text("خط Noto Sans Arabic - دعم شامل للعربية")
            .font_family("NotoSansArabic")
            .font_size(32.0)
            .color(Color::hex("#16A085")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Inconsolata
    canvas.create_layer("inconsolata")?
        .draw_text("Inconsolata - Monospace Code Font")
            .font_family("Inconsolata")
            .font_size(28.0)
            .color(Color::hex("#E67E22")?)
            .position(x_center, y_pos)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    y_pos += 60.0;
    
    // Font count info
    canvas.create_layer("info")?
        .draw_text(&format!("Total fonts loaded: {}", font_manager.font_count()))
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#7F8C8D")?)
            .position(x_center, y_pos + 40.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.save("saves/text-multiple-fonts.png")?;
    Ok(())
}

