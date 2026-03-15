use crate::img_helper;
use crate::shape::{Shape, ShapeType};
use anyhow::Result;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
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
        let original_view = original.view(0, 0, original.width(), original.height());
        let pixel = img_helper::get_average_pixel(*original_view);
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
        let shape = Shape::new_random(self.canvas.width(), self.canvas.height());
        let r = img_helper::bounding_box(&shape.points);
        let sub_image = self.canvas.view(r.0, r.1, r.2, r.3);
        let color = img_helper::get_average_pixel(*sub_image);

        let mut temp_image = self.canvas.clone();
        let shape = Shape::new(self.shape_type, temp_image.width(), temp_image.height());
        shape.draw(&mut temp_image);

        let new_score = img_helper::calculate_difference(&self.canvas, &temp_image);
        if new_score < self.score {
            self.canvas = temp_image;
        }

        todo!()
    }
}
