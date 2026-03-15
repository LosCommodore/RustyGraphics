#![allow(unused)]
mod img_helper;
mod painting;
mod shape;

use anyhow::{Ok, Result};
use image::{ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
use itertools::izip;
use painting::Painting;
use shape::Shape;
use show_image::event;
use show_image::{ImageInfo, ImageView, create_window};
use std::path::Path;

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
        "../images/IMG_2638.JPG",
        600,
        300,
        shape::ShapeType::Triangle,
    )?;
    painting.paint(1);
    display_screen(&painting.canvas)?;

    Ok(())
}
