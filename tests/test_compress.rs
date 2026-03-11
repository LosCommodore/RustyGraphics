use anyhow::{Ok, Result};
use image::{GrayImage, ImageReader};
use retroimage::diffuse::{Diffusion, diffuse_image};
use retroimage::mcpaint::compress_image;
use std::fs;
use std::path::Path;

#[test]
fn test_image_compress() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let th = img.thumbnail(576, 720); // .. stores a fixed 576x720 pixel image using
    let mut gray: GrayImage = th.grayscale().into();
    diffuse_image(&mut gray, Diffusion::FloydSteinberg, 128u8);

    let output = compress_image(&gray)?;
    fs::write("../images/IMG_2638.mcpaint", output).expect("Fehler beim Schreiben");
    Ok(())
}
