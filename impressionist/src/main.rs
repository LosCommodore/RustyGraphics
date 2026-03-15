mod img_helper;
mod painting;
mod shape;

use anyhow::{Ok, Result};
use image::RgbImage;
use painting::Painting;
use show_image::event;
use show_image::{ImageInfo, ImageView, create_window};

fn display_screen(disp_img: &RgbImage) -> Result<()> {
    let width = disp_img.width();
    let height = disp_img.height();

    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &disp_img);
    let window = create_window("image", Default::default())?;
    window.set_image("impressionist", image_view)?;

    // --- SHOW IMAGE
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
    Ok(())
}

#[show_image::main]
fn main() -> Result<()> {
    let mut painting = Painting::from_image(
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/landscape.jpeg",
        200,
        400,
        shape::ShapeType::Ellipse,
    )?;
    painting.paint(40000);
    display_screen(&painting.canvas)?;

    Ok(())
}
