use anyhow::{Ok, Result};
use image::{GrayImage, ImageReader};
use retroimage::diffuse::{Diffusion, diffuse_image};
use retroimage::mcpaint::pack_compress;
use std::path::Path;

#[test]
fn test_image_compress() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;
    let th = img.thumbnail(576, 720); // .. stores a fixed 576x720 pixel image using
    let mut gray: GrayImage = th.grayscale().into();
    diffuse_image(&mut gray, Diffusion::FloydSteinberg, 128u8);

    let _array: Vec<u8> = gray
        .as_raw()
        .chunks_exact(576)
        .map(|row| pack_compress(row))
        .collect::<Result<Vec<_>, _>>()? // Hier wird der Fehler abgefangen
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    Ok(())
}
