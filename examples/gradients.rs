use clove2d::prelude::*;

fn main() -> Result<()> {
    let mut canvas = Canvas::builder()
        .size(800, 600)
        .build()?;
    
    let gradient = Color::linear_gradient()
        .start(0.0, 0.0)
        .end(800.0, 600.0)
        .add_stop(0.0, Color::hex("#FF0080")?)
        .add_stop(0.5, Color::hex("#7928CA")?)
        .add_stop(1.0, Color::hex("#0070F3")?)
        .build();
    
    canvas.create_layer("background")?
        .draw_rect()
            .position(0.0, 0.0)
            .size(800.0, 600.0)
            .fill(gradient)
            .draw()?;
    
    canvas.save("saves/2-gradients.png")?;
    Ok(())
}
