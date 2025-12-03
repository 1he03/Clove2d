use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .background(Color::named(NamedColor::Silver))
        .build()?;
    
    canvas.create_layer("shapes")?
        .draw_rect()
            .position(50.0, 50.0)
            .size(200.0, 150.0)
            .fill(Color::hex("#FF5733")?)
            .stroke(Color::named(NamedColor::Black), 2.0)
            .draw()?
        .draw_circle()
            .center(400.0, 300.0)
            .radius(100.0)
            .fill(Color::rgba(0, 128, 255, 200))
            .stroke(Color::named(NamedColor::White), 3.0)
            .draw()?;
    
    canvas.save("saves/1-basic_shapes.png")?;
    Ok(())
}
