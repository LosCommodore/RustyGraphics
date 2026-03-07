use anyhow::Result;
use image::GrayImage;
use image::ImageFormat;
use image::ImageReader;
use image::Luma;
use image::Pixel;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let mut gray: GrayImage = img.grayscale().into();

    let boundary = 128u8;

    let black = Luma([0]);
    let white = Luma([255]);

    let offset: [[isize; 2]; _] = [[1, 1], [1, -1], [1, 0], [0, 1], [0, 2], [2, 0]];

    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let pixel = gray.get_pixel_mut(x, y);

            let this_pixel = pixel.0[0];

            let delta = if pixel.0[0] > boundary {
                *pixel = white;
                this_pixel as isize - 255isize
            } else {
                *pixel = black;
                this_pixel as isize
            };

            let transfer = delta / 8;

            for [dx, dy] in offset {
                let abs_x = (x as isize + dx).max(0) as u32;
                let abs_y = (y as isize + dy).max(0) as u32;
                let Some(mod_pix) = gray.get_pixel_mut_checked(abs_x, abs_y) else {
                    continue;
                };
                mod_pix.map(|p| (p as isize).saturating_add(transfer).clamp(0, 255) as u8);
            }
        }
    }

    gray.save_with_format("../images/IMG_2638gray", ImageFormat::Png)?;
    Ok(())
}
