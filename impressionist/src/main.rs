use image::{Pixel, Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;
use show_image::event;
use show_image::{ImageInfo, ImageView, create_window};

use itertools::izip;

#[allow(unused)]
fn subtract_images(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = img1.dimensions();
    let mut out_img = RgbImage::new(width, height);

    izip!(img1.pixels(), img2.pixels(), out_img.pixels_mut()).for_each(|(p1, p2, p_out)| {
        *p_out = p1.map2(p2, |a, b| a.abs_diff(b));
    });
    out_img
}

#[allow(unused)]
fn calculate_difference(img1: &RgbImage, img2: &RgbImage) -> u64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0u64, |acc, (p1, p2)| {
            let p_out = p1.map2(p2, |a, b| a.abs_diff(b));
            let diff = p_out.0.iter().fold(0u64, |acc, x| acc + (*x as u64));
            acc + diff
        })
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Erstelle ein Bild mit dem image-Crate (z.B. 800x600, schwarz)
    let width = 800;
    let height = 600;
    let mut img = RgbImage::new(width, height);

    // 2. Ein paar Pixel manuell setzen (oder hier imageproc nutzen)
    for x in 0..width {
        for y in 0..height {
            let color = Rgb([(x % 255) as u8, (y % 255) as u8, 150]);
            img.put_pixel(x, y, color);
        }
    }

    let color = Rgb([100, 200, 240]);

    draw_filled_circle_mut(&mut img, (200, 200), 100, color);

    // 3. Das Bild für show-image vorbereiten
    // Wir sagen show-image, wie die Rohdaten (Bytes) zu interpretieren sind
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &img);

    // Create a window with default options and display the image.
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
