use clove2d::prelude::*;
use clove2d::text::TextWidth;

fn main() -> Result<()> {
    let mut font_manager = FontManager::new();
    
    // Load fonts for mixed content
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Tajawal", "fonts/Tajawal-Regular.ttf")?;
    font_manager.load("Rubik", "fonts/Rubik-Regular.ttf")?;
    font_manager.load("Cairo", "fonts/Cairo-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1200, 900)
        .background(Color::hex("#ECF0F1")?)
        .font_manager(font_manager)
        .build()?;
    
    // Title - English
    canvas.create_layer("title")?
        .draw_text("Mixed Language Text Demo")
            .font_family("Rubik")
            .font_size(48.0)
            .color(Color::hex("#2C3E50")?)
            .position(600.0, 50.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    // Arabic text
    canvas.create_layer("arabic1")?
        .draw_text("مرحباً! هذا مثال على النص المختلط")
            .font_family("Tajawal")
            .font_size(36.0)
            .color(Color::hex("#E74C3C")?)
            .position(600.0, 150.0)
            .align(TextAlign::Center)
            
            .width(TextWidth::None)
            .draw()?;
    
    // English text
    canvas.create_layer("english1")?
        .draw_text("Welcome! This is a mixed text example")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#3498DB")?)
            .position(600.0, 250.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    // Mixed Arabic and English
    canvas.create_layer("mixed1")?
        .draw_text("Hello مرحباً - Mixed content")
            .font_family("Cairo")
            .font_size(28.0)
            .color(Color::hex("#27AE60")?)
            .position(600.0, 350.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    // Different font sizes
    canvas.create_layer("size1")?
        .draw_text("Size: 16px")
            .font_family("NotoSans")
            .font_size(16.0)
            .color(Color::hex("#34495E")?)
            .position(200.0, 450.0)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("size2")?
        .draw_text("Size: 24px")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#34495E")?)
            .position(200.0, 500.0)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("size3")?
        .draw_text("Size: 32px")
            .font_family("NotoSans")
            .font_size(32.0)
            .color(Color::hex("#34495E")?)
            .position(200.0, 560.0)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("size4")?
        .draw_text("Size: 48px")
            .font_family("NotoSans")
            .font_size(48.0)
            .color(Color::hex("#34495E")?)
            .position(200.0, 640.0)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    
    // Different colors
    canvas.create_layer("color1")?
        .draw_text("Red Text")
            .font_family("Rubik")
            .font_size(28.0)
            .color(Color::hex("#E74C3C")?)
            .position(600.0, 450.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("color2")?
        .draw_text("Blue Text")
            .font_family("Rubik")
            .font_size(28.0)
            .color(Color::hex("#3498DB")?)
            .position(600.0, 500.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("color3")?
        .draw_text("Green Text")
            .font_family("Rubik")
            .font_size(28.0)
            .color(Color::hex("#27AE60")?)
            .position(600.0, 550.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("color4")?
        .draw_text("Purple Text")
            .font_family("Rubik")
            .font_size(28.0)
            .color(Color::hex("#8E44AD")?)
            .position(600.0, 600.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    // Different alignments
    canvas.create_layer("align1")?
        .draw_text("Left")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#34495E")?)
            .position(100.0, 750.0)
            .align(TextAlign::Left)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("align2")?
        .draw_text("Center")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#34495E")?)
            .position(600.0, 750.0)
            .align(TextAlign::Center)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.create_layer("align3")?
        .draw_text("Right")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#34495E")?)
            .position(1100.0, 750.0)
            .align(TextAlign::Right)
            .width(TextWidth::None)
            .draw()?;
    
    canvas.save("saves/text-mixed.png")?;
    Ok(())
}

