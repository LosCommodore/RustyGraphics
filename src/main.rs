use anyhow::Result;
use image::GrayImage;
use image::ImageFormat;
use image::ImageReader;
use image::Luma;
use image::Pixel;
use std::path::Path;
use std::time::Instant;

pub enum Diffusion {
    Atkinson,
    FloydSteinberg,
}

impl Diffusion {
    fn diffuse(&self, x: u32, y: u32, buffer: &mut GrayImage, error: isize) {
        const DIFFUSION_ATKINSON: [[isize; 2]; 6] =
            [[1, 1], [1, -1], [1, 0], [0, 1], [0, 2], [2, 0]];
        const DIFFUSION_STEINBERG: [[isize; 3]; 4] = [[1, 0, 7], [-1, 1, 3], [0, 1, 5], [1, 1, 1]];

        match self {
            Diffusion::Atkinson => {
                // Note: Atkinson does only diffuse 6/8th of the error.

                let transfer = error / 8;
                for [dx, dy] in DIFFUSION_ATKINSON {
                    let abs_x = (x as isize + dx).max(0) as u32;
                    let abs_y = (y as isize + dy).max(0) as u32;
                    let Some(mod_pix) = buffer.get_pixel_mut_checked(abs_x, abs_y) else {
                        continue;
                    };
                    mod_pix.map(|p| (p as isize).saturating_add(transfer).clamp(0, 255) as u8);
                }
            }
            Diffusion::FloydSteinberg => {
                let transfer = error / 16;

                for [dx, dy, factor] in DIFFUSION_STEINBERG {
                    let abs_x = (x as isize + dx).max(0) as u32;
                    let abs_y = (y as isize + dy).max(0) as u32;
                    let Some(mod_pix) = buffer.get_pixel_mut_checked(abs_x, abs_y) else {
                        continue;
                    };
                    mod_pix.map(|p| {
                        (p as isize).saturating_add(transfer * factor).clamp(0, 255) as u8
                    });
                }
            }
        };
    }
}

fn diffuse_image(gray: &mut GrayImage) {
    let boundary = 128u8;

    let diff = Diffusion::Atkinson;

    let black = Luma([0]);
    let white = Luma([255]);

    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let current_pixel = gray.get_pixel_mut(x, y);
            let current_value = current_pixel.0[0];

            let error = if current_value > boundary {
                current_value as isize - 255isize
            } else {
                current_value as isize
            };

            *current_pixel = if error > 0 { black } else { white };
            diff.diffuse(x, y, gray, error);
        }
    }
}

fn main() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let mut gray: GrayImage = img.grayscale().into();

    let start = Instant::now();
    diffuse_image(&mut gray);
    gray.save_with_format("../images/IMG_2638gray.png", ImageFormat::Png)?;
    println!("Dauer: {:?}", start.elapsed());

    Ok(())
}
