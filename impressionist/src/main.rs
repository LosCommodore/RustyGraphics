mod img_helper;
mod painting;
mod shape;

use crate::shape::ShapeType;
use anyhow::{Ok, Result};
use image::RgbImage;
use painting::Painting;
use show_image::{ImageInfo, ImageView, WindowProxy, create_window, event};
use std::path::Path;

fn display_screen(window: &WindowProxy, disp_img: &RgbImage) -> Result<()> {
    let width = disp_img.width();
    let height = disp_img.height();
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &disp_img);
    window.set_image("impressionist", image_view)?;
    Ok(())
}

fn exit_on_escape(window: &WindowProxy) -> Result<()> {
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

fn run(
    file: impl AsRef<Path>,
    width: u32,
    height: u32,
    shape_type: ShapeType,
    max_iter: usize,
    animate: bool,
) -> Result<()> {
    let mut painting = Painting::from_image(file, width, height, shape_type)?;

    let window = create_window("image", Default::default())?;
    if animate == true {
        display_screen(&window, &painting.canvas)?;
    }

    for i in 1..=max_iter {
        println!("Iteration {i} of {max_iter}");
        painting.next_shape();
        if animate == true {
            display_screen(&window, &painting.canvas)?;
        }
    }

    display_screen(&window, &painting.canvas)?;
    exit_on_escape(&window)?;
    Ok(())
}

#[show_image::main]
fn main() -> Result<()> {
    run(
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/landscape.jpeg",
        200,
        400,
        ShapeType::Ellipse,
        1000,
        true,
    )
}
