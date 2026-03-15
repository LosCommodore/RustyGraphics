#![allow(unused)]
mod shape;
use image::{Pixel, Rgb, RgbImage};
use itertools::izip;
use shape::Shape;
use show_image::event;
use show_image::{ImageInfo, ImageView, create_window};

fn subtract_images(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = img1.dimensions();
    let mut out_img = RgbImage::new(width, height);

    izip!(img1.pixels(), img2.pixels(), out_img.pixels_mut()).for_each(|(p1, p2, p_out)| {
        *p_out = p1.map2(p2, |a, b| a.abs_diff(b));
    });
    out_img
}

fn calculate_difference(img1: &RgbImage, img2: &RgbImage) -> u64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0u64, |acc, (p1, p2)| {
            let p_out = p1.map2(p2, |a, b| a.abs_diff(b));
            let diff = p_out.0.iter().fold(0u64, |acc, x| acc + (*x as u64));
            acc + diff
        })
}

fn snippets() {
    /*
    // 2. Ein paar Pixel manuell setzen (oder hier imageproc nutzen)
    for x in 0..width {
        for y in 0..height {
            let color = Rgb([(x % 255) as u8, (y % 255) as u8, 150]);
            img.put_pixel(x, y, color);
        }
    }
    */
    todo!()
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Erstelle ein Bild mit dem image-Crate (z.B. 800x600, schwarz)
    let width = 800;
    let height = 600;
    let mut img = RgbImage::new(width, height);

    // --- print to image
    for _ in 0..100 {
        let shape = Shape::new_random(width, height);
        shape.draw(&mut img);
    }
    //draw_filled_circle_mut(&mut img, (100, 100), 200, Rgb([100, 200, 200]));
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &img);
    let window = create_window("image", Default::default())?;
    window.set_image("bild-001", image_view)?;

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn flatten_pixels(input: &[[u8; 3]]) -> Vec<u8> {
        input.into_iter().copied().flatten().collect::<Vec<u8>>()
    }

    #[test]
    fn test_add() {
        let pixel = flatten_pixels(&[[5, 6, 5], [5, 5, 5]]);
        let pixel2 = flatten_pixels(&[[5, 5, 7], [5, 5, 0]]);

        let img1 = RgbImage::from_raw(2, 1, pixel).unwrap();
        let img2 = RgbImage::from_raw(2, 1, pixel2).unwrap();
        let d = calculate_difference(&img1, &img2);
        assert_eq!(d, 8);
    }
}
