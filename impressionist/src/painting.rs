use crate::shape::Shape;
use anyhow::Result;
use image::{ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
use itertools::izip;

use std::path::Path;
pub struct Painting {
    shapes: Vec<Shape>,
    pub original: RgbImage,
    pub canvas: RgbImage,
    score: usize,
}

impl Painting {
    pub fn from_image(file: impl AsRef<Path>, width: u32, height: u32) -> Result<Self> {
        let file = file.as_ref();
        let img = ImageReader::open(file)?.decode()?;

        let th = img.thumbnail(width, height);
        let original = th.into_rgb8();
        let shapes = Vec::new();

        let pixel = get_average_pixel(&original);
        let mut canvas = RgbImage::from_pixel(width, height, pixel);

        Ok(Self {
            original,
            shapes,
            canvas,
            score: 0,
        })
    }
}

pub fn subtract_images(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = img1.dimensions();
    let mut out_img = RgbImage::new(width, height);

    izip!(img1.pixels(), img2.pixels(), out_img.pixels_mut()).for_each(|(p1, p2, p_out)| {
        *p_out = p1.map2(p2, |a, b| a.abs_diff(b));
    });
    out_img
}

pub fn calculate_difference(img1: &RgbImage, img2: &RgbImage) -> u64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0u64, |acc, (p1, p2)| {
            let p_out = p1.map2(p2, |a, b| a.abs_diff(b));
            let diff = p_out.0.iter().fold(0u64, |acc, x| acc + (*x as u64));
            acc + diff
        })
}

pub fn get_average_pixel(img1: &RgbImage) -> Rgb<u8> {
    let pixel_count = img1.width() + img1.height();
    let rgb_sum = img1.pixels().fold([0usize; 3], |acc, item| {
        [
            acc[0] + item[0] as usize,
            acc[1] + item[1] as usize,
            acc[2] + item[2] as usize,
        ]
    });
    let avg = rgb_sum
        .iter()
        .map(|x| (x / pixel_count as usize) as u8)
        .collect::<Vec<_>>();
    let arr: [u8; 3] = avg.try_into().unwrap();
    Rgb(arr)
}
