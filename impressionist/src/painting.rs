use crate::img_helper;
use crate::shape::{Shape, ShapeType};
use anyhow::Result;
use image::{ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
use imageproc::point::Point;
use itertools::izip;

use std::path::Path;
pub struct Painting {
    shapes: Vec<Shape>,
    shape_type: ShapeType,
    pub original: RgbImage,
    pub canvas: RgbImage,
    score: u64,
}

impl Painting {
    pub fn from_image(
        file: impl AsRef<Path>,
        width: u32,
        height: u32,
        shape_type: ShapeType,
    ) -> Result<Self> {
        let file = file.as_ref();
        let img = ImageReader::open(file)?.decode()?;

        let th = img.thumbnail(width, height);
        let original = th.into_rgb8();
        let shapes = Vec::new();

        let pixel = img_helper::get_average_pixel(&original);
        let mut canvas = RgbImage::from_pixel(width, height, pixel);
        let score = img_helper::calculate_difference(&original, &canvas);

        Ok(Self {
            original,
            shapes,
            canvas,
            score,
            shape_type,
        })
    }

    pub fn step(&mut self) {
        todo!()
    }
}
