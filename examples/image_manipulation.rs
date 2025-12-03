use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::hex("#1A1A1A")?)
        .build()?;
    
    // Create a dummy image
    let mut image_canvas = Canvas::new(100, 100)?;
    image_canvas.clear(Color::named(NamedColor::Red));
    image_canvas.save("saves/dummy_image.png")?;

    let image = Image::from_path("saves/dummy_image.png")?;

    canvas.create_layer("image")?
        .draw_image(image)
            .position(100.0, 100.0)
            .opacity(0.9)
            .draw()?
        .apply_filter(Filter::Blur(5.0))?;
    
    canvas.save("saves/5-image_manipulation.png")?;
    Ok(())
}
