use anyhow::Result;
use image::GrayImage;
use image::ImageFormat;
use image::ImageReader;
use retroimage::diffuse::Diffusion;
use retroimage::diffuse::diffuse_image;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let th = img.thumbnail(576, 720); // .. stores a fixed 576x720 pixel image using
    let mut gray: GrayImage = th.grayscale().into();
    let start = Instant::now();
    diffuse_image(&mut gray, Diffusion::FloydSteinberg, 128u8);
    gray.save_with_format("../images/IMG_2638gray_v2.png", ImageFormat::Png)?;
    println!("Dauer: {:?}", start.elapsed());

    Ok(())
}
