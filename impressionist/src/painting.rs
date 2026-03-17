use crate::img_helper;
use crate::shape::{Shape, ShapeType};
use anyhow::Result;
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, RgbImage};
use std::path::Path;

/// Calculate the fitness value for a given shape. Lower is better.s
pub type FitnessFn<'a> = dyn Fn(&Shape) -> u64 + Sync + 'a;

/// Optimizer for optimizing a provied Shape based on the FitnessFunction.
pub type OptimizerFn = for<'a> fn(
    u32,           // screen: width
    u32,           // screen: height
    &Shape,        // inital shape
    u64,           // inital score
    &'a FitnessFn, // fitness function
) -> (Shape, u64);

pub struct Painting {
    shapes: Vec<Shape>,
    shape_type: ShapeType,
    pub original: RgbImage,
    pub canvas: RgbImage,
    score: u64,
    pub shape_optimizer: OptimizerFn,
}

impl Painting {
    pub fn from_image(
        file: impl AsRef<Path>,
        width: u32,
        height: u32,
        shape_type: ShapeType,
        shape_optimizer: OptimizerFn,
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

        let me = Painting {
            original,
            shapes,
            canvas,
            score,
            shape_type,
            shape_optimizer,
        };

        Ok(me)
    }

    fn get_avarage_color_from_shape_boundaries(&self, shape: &Shape) -> Rgb<u8> {
        let r: (u32, u32, u32, u32) = img_helper::bounding_box(&shape.points);
        let sub_image = self.original.view(r.0, r.1, r.2, r.3);
        img_helper::get_average_pixel(*sub_image)
    }

    fn calculate_score(&self, shape: &Shape) -> u64 {
        let mut temp_image = self.canvas.clone();
        shape.draw(&mut temp_image);
        img_helper::calculate_difference(&self.original, &temp_image)
    }

    pub fn execute_step(&mut self) -> bool {
        let width = self.canvas.width();
        let height = self.canvas.height();

        // -- Create a random initial shape
        let mut initial_shape =
            Shape::new_random_position(self.shape_type, width, height, Rgb([0, 0, 0]));
        initial_shape.color = self.get_avarage_color_from_shape_boundaries(&initial_shape);

        // -- Return if shape makes picture worse
        let initial_score = self.calculate_score(&initial_shape);
        if initial_score > self.score {
            return false;
        }

        // -- Optimize shape
        let fitness_fn = |s: &Shape| self.calculate_score(s);
        let (shape, score) =
            (self.shape_optimizer)(width, height, &initial_shape, initial_score, &fitness_fn);

        // -- Draw
        shape.draw(&mut self.canvas);

        // -- Remember shape and score
        self.score = score;
        self.shapes.push(shape);

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
