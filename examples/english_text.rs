use clove2d::prelude::*;
use clove2d::text::TextWidth;

fn main() -> Result<()> {
    let mut font_manager = FontManager::new();
    
    // Load multiple English fonts
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    font_manager.load("Rubik", "fonts/Rubik-Regular.ttf")?;
    font_manager.load("Inconsolata", "fonts/Inconsolata_Condensed-Bold.ttf")?;
    font_manager.load("Roboto", "fonts/NotoSans-Regular.ttf")?;
    
    let mut canvas = Canvas::builder()
        .size(1000, 1500)
        .background(Color::hex("#F5F5F5")?)
        .font_manager(font_manager)
        .build()?;
    
    // Layer 1: TextWidth::None with Left/Right/Center/Justify alignment and LTR/RTL direction
    let layer1 = canvas.create_layer("text1")?;
    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 0.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 30.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 60.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 90.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 120.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 150.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 180.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::None)
        .draw()?;

    layer1.draw_text("Welcome to Clove2d!")
        .font_family("NotoSans")
        .font_size(20.0)
        .color(Color::hex("#2C3E50")?)
        .position(0.0, 210.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::None)
        .draw()?;

    // Layer 2: TextWidth::Max(400.0)
    let layer2 = canvas.create_layer("text2")?;
    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 240.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 270.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 300.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 330.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 360.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 390.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 420.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    layer2.draw_text("Welcome to Clove2d!")
        .font_family("Rubik")
        .font_size(20.0)
        .color(Color::hex("#E74C3C")?)
        .position(0.0, 450.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Max(400.0))
        .draw()?;

    // Layer 3: TextWidth::FullPage
    let layer3 = canvas.create_layer("text3")?;
    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 480.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 510.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 540.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 570.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 600.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 630.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 660.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::FullPage)
        .draw()?;

    layer3.draw_text("Welcome to Clove2d!")
        .font_family("Inconsolata")
        .font_size(20.0)
        .color(Color::hex("#27AE60")?)
        .position(0.0, 690.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::FullPage)
        .draw()?;

    // Layer 4: TextWidth::Layer
    let layer4 = canvas.create_layer("text4")?;
    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 720.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 750.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 780.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 810.0)
        .align(TextAlign::Right)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 840.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 870.0)
        .align(TextAlign::Center)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 900.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Layer)
        .draw()?;

    layer4.draw_text("Welcome to Clove2d!")
        .font_family("Roboto")
        .font_size(20.0)
        .color(Color::hex("#8E44AD")?)
        .position(0.0, 930.0)
        .align(TextAlign::Left)
        
        .width(TextWidth::Layer)
        .draw()?;
    
    canvas.save("saves/text-english.png")?;
    Ok(())
}
