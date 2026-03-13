use anyhow::Result;
use image::GrayImage;
use image::ImageReader;
use retroimage::diffuse::Diffusion;
use retroimage::diffuse::diffuse_image;
use retroimage::mcpaint::compress_image;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<()> {
    let path = Path::new("../images/IMG_2638.JPG");
    let img = ImageReader::open(path)?.decode()?;

    let start = Instant::now();
    let th = img.thumbnail(576, 720); // .. stores a fixed 576x720 pixel image using
    let mut gray: GrayImage = th.grayscale().into();
    println!("Gray Thumbnail: {:?}", start.elapsed());

    let start = Instant::now();
    diffuse_image(&mut gray, Diffusion::FloydSteinberg, 128u8);
    println!("Diffuse: {:?}", start.elapsed());

    let start = Instant::now();
    let output = compress_image(&gray)?;
    println!("Compress: {:?}", start.elapsed());

    fs::write("../images/IMG_2638.mcpaint", output).expect("Fehler beim Schreiben");

    Ok(())
}
