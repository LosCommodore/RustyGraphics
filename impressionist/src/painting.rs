use crate::shape::{Shape, ShapeType};
use crate::{img_helper, shape};
use anyhow::Result;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
use imageproc::point::Point;
use itertools::{enumerate, izip};

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
        let mut canvas: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::from_pixel(width, height, pixel);
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
        let sub_image = self.canvas.view(r.0, r.1, r.2, r.3);
        img_helper::get_average_pixel(*sub_image)
    }

    fn calculate_score(&self, shape: &Shape, color: Rgb<u8>) -> u64 {
        let mut temp_image = self.canvas.clone();
        shape.draw(&mut temp_image, color);
        img_helper::calculate_difference(&self.canvas, &temp_image)
    }

    pub fn next_shape(&mut self) -> (Shape, Rgb<u8>, u64) {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let shape = Shape::new_random_position(self.shape_type, width, height);
        let color = self.get_avarage_color_from_shape_boundaries(&shape);

        let inital_shape = Shape::new_random_position(self.shape_type, width, height);
        let mut initial_score = self.calculate_score(&inital_shape, color);

        let mut best_shape = inital_shape.clone();
        let mut best_score = initial_score;

        for (i_point, point) in enumerate(&inital_shape.points) {
            let directions_cross = [
                (0..point.x as i32, 0..1),
                ((point.x + 1)..width as i32, 0..1),
                (0..1, 0..point.y as i32),
                ((point.y + 1)..height as i32, 0..1),
            ];

            for (iter_x, iter_y) in &directions_cross {
                for x in iter_x.clone() {
                    for y in iter_y.clone() {
                        let mut new_points = best_shape.points.clone();
                        new_points[i_point].x = x;
                        new_points[i_point].y = y;

                        let shape = Shape {
                            shape_type: self.shape_type,
                            points: new_points,
                        };
                        let score = self.calculate_score(&inital_shape, color);
                        if score > initial_score {
                            break;
                        }
                        if score < initial_score {
                            best_shape = shape;
                            best_score = score;
                        }
                    }
                }
            }
        }

        (best_shape, color, best_score)
    }

    pub fn paint(&mut self, runs: usize) {
        for i in 0..runs {
            println!("run: {i} of {runs}");
            let (shape, color, score) = self.next_shape();
            if score < self.score {
                shape.draw(&mut self.canvas, color);
                self.shapes.push((shape, color));
            }
        }
    }
}
