mod diffuse;
mod mcpaint;
use anyhow::Result;
use diffuse::diffuse_image;
use image::GrayImage;
use image::ImageFormat;
use image::ImageReader;
use std::path::Path;
use std::time::Instant;

use crate::diffuse::Diffusion;

fn main() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let mut gray: GrayImage = img.grayscale().into();

    let start = Instant::now();
    diffuse_image(&mut gray, Diffusion::FloydSteinberg, 128u8);
    gray.save_with_format("../images/IMG_2638gray_v2.png", ImageFormat::Png)?;
    println!("Dauer: {:?}", start.elapsed());

    Ok(())
}
