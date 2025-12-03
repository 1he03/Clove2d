use clove2d::prelude::*;

fn main() -> Result<()> {
    println!("=== Example: Images & Filters Showcase ===");
    
    // First, create a test image
    println!("Creating test image...");
    let mut test_canvas = Canvas::builder()
        .size(400, 400)
        .background(Color::hex("#FFFFFF")?)
        .build()?;
    
    let mut font_manager = FontManager::new();
    font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
    test_canvas.set_font_manager(font_manager);
    
    test_canvas.create_layer("test_image")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(300.0, 300.0)
            .fill(Color::linear_gradient()
                .start(50.0, 50.0)
                .end(350.0, 350.0)
                .add_stop(0.0, Color::hex("#667EEA")?)
                .add_stop(1.0, Color::hex("#764BA2")?)
                .build())
            .draw()?
        .draw_circle()
            .center(200.0, 200.0)
            .radius(100.0)
            .fill(Color::rgba(255, 255, 255, 180))
            .draw()?
        .draw_text("Clove2d")
            .font_family("NotoSans")
            .font_size(48.0)
            .font_weight(FontWeight::Bold)
            .color(Color::hex("#2C3E50")?)
            .position(200.0, 190.0)
            .align(TextAlign::Center)
            .draw()?
        .draw_text("Image Test")
            .font_family("NotoSans")
            .font_size(24.0)
            .color(Color::hex("#34495E")?)
            .position(200.0, 240.0)
            .align(TextAlign::Center)
            .draw()?;
    
    test_canvas.save("saves/test_image_for_filters.png")?;
    
    // Load the test image
    let test_image = Image::from_path("saves/test_image_for_filters.png")?;
    
    // Main canvas
    let mut canvas = Canvas::builder()
        .size(1800, 1800)
        .background(Color::hex("#ECF0F1")?)
        .build()?;
    
    // Original image
    println!("Drawing original image...");
    {
        let layer = canvas.create_layer("images_filters")?;
        layer.draw_image(test_image.clone())
            .position(50.0, 50.0)
            .opacity(1.0)
            .draw()?;
    }
    
    // Blur filter
    println!("Applying Blur filter...");
    {
        let blur_layer = canvas.create_layer("blur")?;
        blur_layer.draw_image(test_image.clone())
            .position(500.0, 50.0)
            .opacity(1.0)
            .draw()?;
        blur_layer.apply_filter(Filter::Blur(10.0))?;
    }
    
    // Grayscale filter
    println!("Applying Grayscale filter...");
    {
        let grayscale_layer = canvas.create_layer("grayscale")?;
        grayscale_layer.draw_image(test_image.clone())
            .position(950.0, 50.0)
            .opacity(1.0)
            .draw()?;
        grayscale_layer.apply_filter(Filter::Grayscale)?;
    }
    
    // Sepia filter
    println!("Applying Sepia filter...");
    {
        let sepia_layer = canvas.create_layer("sepia")?;
        sepia_layer.draw_image(test_image.clone())
            .position(1400.0, 50.0)
            .opacity(1.0)
            .draw()?;
        sepia_layer.apply_filter(Filter::Sepia)?;
    }
    
    // Invert filter
    println!("Applying Invert filter...");
    {
        let invert_layer = canvas.create_layer("invert")?;
        invert_layer.draw_image(test_image.clone())
            .position(50.0, 500.0)
            .opacity(1.0)
            .draw()?;
        invert_layer.apply_filter(Filter::Invert)?;
    }
    
    // Brightness filter
    println!("Applying Brightness filter...");
    {
        let brightness_layer = canvas.create_layer("brightness")?;
        brightness_layer.draw_image(test_image.clone())
            .position(500.0, 500.0)
            .opacity(1.0)
            .draw()?;
        brightness_layer.apply_filter(Filter::Brightness(0.3))?;
    }
    
    // Contrast filter
    println!("Applying Contrast filter...");
    {
        let contrast_layer = canvas.create_layer("contrast")?;
        contrast_layer.draw_image(test_image.clone())
            .position(950.0, 500.0)
            .opacity(1.0)
            .draw()?;
        contrast_layer.apply_filter(Filter::Contrast(0.5))?;
    }
    
    // Saturation filter
    println!("Applying Saturation filter...");
    {
        let saturation_layer = canvas.create_layer("saturation")?;
        saturation_layer.draw_image(test_image.clone())
            .position(1400.0, 500.0)
            .opacity(1.0)
            .draw()?;
        saturation_layer.apply_filter(Filter::Saturation(0.8))?;
    }
    
    // Sharpen filter
    println!("Applying Sharpen filter...");
    {
        let sharpen_layer = canvas.create_layer("sharpen")?;
        sharpen_layer.draw_image(test_image.clone())
            .position(50.0, 950.0)
            .opacity(1.0)
            .draw()?;
        sharpen_layer.apply_filter(Filter::Sharpen(2.0))?;
    }
    
    // HueRotate filter
    println!("Applying HueRotate filter...");
    {
        let hue_layer = canvas.create_layer("hue")?;
        hue_layer.draw_image(test_image.clone())
            .position(500.0, 950.0)
            .opacity(1.0)
            .draw()?;
        hue_layer.apply_filter(Filter::HueRotate(90.0))?;
    }
    
    // Multiple filters combined
    println!("Applying multiple filters...");
    {
        let multi_layer = canvas.create_layer("multi_filter")?;
        multi_layer.draw_image(test_image.clone())
            .position(950.0, 950.0)
            .opacity(1.0)
            .draw()?;
        multi_layer.apply_filter(Filter::Grayscale)?;
        multi_layer.apply_filter(Filter::Brightness(0.2))?;
        multi_layer.apply_filter(Filter::Contrast(0.3))?;
    }
    
    // Image with opacity variations
    println!("Drawing images with different opacities...");
    {
        let layer = canvas.create_layer("opacity_variations")?;
        for i in 0..5 {
            let opacity = 0.2 + (i as f32 * 0.2);
            layer.draw_image(test_image.clone())
                .position(1400.0, 950.0 + i as f32 * 80.0)
                .opacity(opacity)
                .draw()?;
        }
    }
    
    // Image transformations (scaled images)
    println!("Drawing scaled images...");
    {
        let scaled_layer = canvas.create_layer("scaled")?;
        
        // Small
        let small_image = Image::from_path("saves/test_image_for_filters.png")?;
        scaled_layer.draw_image(small_image)
            .position(50.0, 1200.0)
            .opacity(1.0)
            .draw()?;
        
        // Medium (would need transform, but showing position)
        let medium_image = Image::from_path("saves/test_image_for_filters.png")?;
        scaled_layer.draw_image(medium_image)
            .position(500.0, 1200.0)
            .opacity(0.8)
            .draw()?;
    }
    
    // Labels
    {
        let mut font_manager = FontManager::new();
        font_manager.load("NotoSans", "fonts/NotoSans-Regular.ttf")?;
        canvas.set_font_manager(font_manager);
    }
    
    let labels = [
        ("Original", 250.0, 460.0),
        ("Blur", 650.0, 460.0),
        ("Grayscale", 1100.0, 460.0),
        ("Sepia", 1550.0, 460.0),
        ("Invert", 250.0, 910.0),
        ("Brightness", 650.0, 910.0),
        ("Contrast", 1100.0, 910.0),
        ("Saturation", 1550.0, 910.0),
        ("Sharpen", 250.0, 1360.0),
        ("HueRotate", 650.0, 1360.0),
        ("Multi Filter", 1100.0, 1360.0),
    ];
    
    {
        let layer = canvas.create_layer("labels")?;
        for (label, x, y) in labels.iter() {
            layer.draw_text(label)
                .font_family("NotoSans")
                .font_size(16.0)
                .color(Color::named(NamedColor::Black))
                .position(*x, *y)
                .align(TextAlign::Center)
                .draw()?;
        }
    }
    
    println!("Saving image...");
    canvas.save("saves/example_images_filters.png")?;
    println!("✓ Saved to saves/example_images_filters.png");
    
    Ok(())
}

