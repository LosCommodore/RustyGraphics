mod img_helper;
mod optimizers;
mod painting;
mod shape;

use crate::shape::ShapeType;
use anyhow::{Ok, Result};
use image::RgbImage;
use painting::OptimizerFn;
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
    thumpnail_file: impl AsRef<Path>,
    save_file: impl AsRef<Path>,
    width: u32,
    height: u32,
    shape_type: ShapeType,
    max_iter: usize,
    animate: Option<usize>,
    shape_optimizer: OptimizerFn,
) -> Result<()> {
    let mut painting = Painting::from_image(file, width, height, shape_type, shape_optimizer)?;
    painting.original.save(thumpnail_file)?;

    let window = create_window("image", Default::default())?;
    if let Some(_) = animate {
        display_screen(&window, &painting.canvas)?;
    }

    let mut anim_counter = 1;
    for i in 1..=max_iter {
        println!("Iteration {i} of {max_iter}");
        painting.execute_step();
        if let Some(anim_step) = animate {
            if anim_counter >= anim_step {
                display_screen(&window, &painting.canvas)?;
                anim_counter = 1;
            } else {
                anim_counter += 1;
            }
        }
    }

    painting.canvas.save(save_file)?;
    display_screen(&window, &painting.canvas)?;
    exit_on_escape(&window)?;
    Ok(())
}

#[show_image::main]
fn main() -> Result<()> {
    /*
    run(
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/landscape.jpeg",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/landscape_thumpnail.jpeg",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/landscape_impression.jpeg",
        200,
        300,
        ShapeType::Ellipse,
        100000,
        Some(10),
    )
    */
    /*
    run(
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/face_painting.jpg",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/face_painting_th.jpg",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/face_painting_impression.jpg",
        400,
        600,
        ShapeType::Quadrinial,
        80000,
        Some(10),
    )
    */
    /*
    run(
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/IMG_2627.JPG",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/IMG_2627.JPG_th.jpg",
        "/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/IMG_2627.JPG_impression.jpg",
        200,
        300,
        ShapeType::Triangle,
        50000,
        Some(10),
    )
    */
    let image = "IMG-20251120-WA0001";
    let ending = ".jpg";

    run(
        format!("/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/{image}{ending}"),
        format!("/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/{image}_th.jpg"),
        format!("/home/clangen/Proj/cs_from_scatch/RustyGraphics/images/{image}_impression.jpg"),
        200,
        300,
        ShapeType::Quadrinial,
        500000,
        Some(10),
        (optimizers::OptimizerType::Cross).get_fn(),
    )
}
