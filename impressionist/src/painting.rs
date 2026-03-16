use crate::img_helper;
use crate::optimizer::optimize_shape;
use crate::shape::{Shape, ShapeType};
use anyhow::Result;
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, RgbImage};
use std::path::Path;

pub struct Painting {
    shapes: Vec<(Shape, Rgb<u8>)>,
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
        let canvas: ImageBuffer<Rgb<u8>, Vec<u8>> =
            RgbImage::from_pixel(original.width(), original.height(), pixel);
        let score = img_helper::calculate_difference(&original, &canvas);

        Ok(Self {
            original,
            shapes,
            canvas,
            score,
            shape_type,
        })
    }

    fn get_avarage_color_from_shape_boundaries(&self, shape: &Shape) -> Rgb<u8> {
        let r: (u32, u32, u32, u32) = img_helper::bounding_box(&shape.points);
        let sub_image = self.original.view(r.0, r.1, r.2, r.3);
        img_helper::get_average_pixel(*sub_image)
    }

    fn calculate_score(&self, shape: &Shape, color: Rgb<u8>) -> u64 {
        let mut temp_image = self.canvas.clone();
        shape.draw(&mut temp_image, color);
        img_helper::calculate_difference(&self.original, &temp_image)
    }

    pub fn execute_step(&mut self) -> bool {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let initial_shape = Shape::new_random_position(self.shape_type, width, height);
        let color = self.get_avarage_color_from_shape_boundaries(&initial_shape);
        let initial_score = self.calculate_score(&initial_shape, color);
        if initial_score > self.score {
            return false;
        }

        let Some((shape, color, score)) = optimize_shape(
            width,
            height,
            color,
            &initial_shape,
            initial_score,
            |shape, color| self.calculate_score(shape, color),
        ) else {
            return false;
        };

        shape.draw(&mut self.canvas, color);
        self.score = score;
        self.shapes.push((shape, color));
        true
    }

    #[allow(unused)]
    pub fn paint(&mut self, runs: usize) {
        for i in 0..runs {
            println!("run: {i} of {runs}");
            match self.execute_step() {
                true => {
                    println!(" -> sucess, new shaped added");
                }
                false => {
                    println!(" -> fail, shaped discarded");
                }
            };
        }
    }
}
