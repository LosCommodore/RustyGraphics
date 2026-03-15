use image::{Pixel, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_line_segment_mut, draw_polygon_mut};
use imageproc::point::Point;
use itertools::izip;
use rand::RngExt;
use rand::prelude::*;
use show_image::event;
use show_image::{ImageInfo, ImageView, create_window};

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
enum ShapeType {
    Ellipse,
    Triangle,
    Quadrinial,
    Line,
}

struct Shape {
    shape_type: ShapeType,
    points: Vec<Point<i32>>,
}

fn random_point(screen_width: u32, screen_height: u32) -> Point<i32> {
    let mut rng = rand::rng();
    let x = rng.random_range(0..screen_width) as i32;
    let y = rng.random_range(0..screen_height) as i32;
    Point { x, y }
}

fn random_color() -> Rgb<u8> {
    let mut rng = rand::rng();
    let colors: [u8; 3] = std::array::from_fn(|_| rng.random_range(0..=255));
    Rgb(colors)
}

#[allow(unused)]
impl Shape {
    fn new(shape_type: ShapeType, screen_width: u32, screen_height: u32) -> Self {
        let num_points = match shape_type {
            ShapeType::Line => 2,
            ShapeType::Ellipse => 2,
            ShapeType::Triangle => 3,
            ShapeType::Quadrinial => 4,
        };

        let points = (0..num_points)
            .map(|_| random_point(screen_width, screen_height))
            .collect();
        Self { shape_type, points }
    }

    pub fn new_random(screen_width: u32, screen_height: u32) -> Self {
        let mut rng = rand::rng();
        let choices = [
            ShapeType::Ellipse,
            ShapeType::Line,
            ShapeType::Triangle,
            ShapeType::Quadrinial,
        ];

        let shape_type = choices.choose(&mut rng).unwrap();
        Shape::new(*shape_type, screen_width, screen_height)
    }

    fn draw(&self, canvas: &mut RgbImage) {
        let color = random_color();

        match self.shape_type {
            ShapeType::Ellipse => {
                // convert bounding-box points to -> center & radius
                let delta = self.points[1] - self.points[0];
                let dx = delta.x / 2;
                let dy = delta.y / 2;
                let center = (self.points[0].x + dx, self.points[0].y + dy);
                draw_filled_ellipse_mut(canvas, center, dx.abs(), dy.abs(), color);
            }

            ShapeType::Line => {
                let start = (self.points[0].x as f32, self.points[0].y as f32);
                let end = (self.points[1].x as f32, self.points[1].y as f32);
                draw_line_segment_mut(canvas, start, end, color);
            }
            ShapeType::Triangle | ShapeType::Quadrinial => {
                draw_polygon_mut(canvas, &self.points, color);
            }
        }
    }
}

#[allow(unused)]
fn subtract_images(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = img1.dimensions();
    let mut out_img = RgbImage::new(width, height);

    izip!(img1.pixels(), img2.pixels(), out_img.pixels_mut()).for_each(|(p1, p2, p_out)| {
        *p_out = p1.map2(p2, |a, b| a.abs_diff(b));
    });
    out_img
}

#[allow(unused)]
fn calculate_difference(img1: &RgbImage, img2: &RgbImage) -> u64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0u64, |acc, (p1, p2)| {
            let p_out = p1.map2(p2, |a, b| a.abs_diff(b));
            let diff = p_out.0.iter().fold(0u64, |acc, x| acc + (*x as u64));
            acc + diff
        })
}

#[allow(unused)]
fn snippets() {
    /*
    // 2. Ein paar Pixel manuell setzen (oder hier imageproc nutzen)
    for x in 0..width {
        for y in 0..height {
            let color = Rgb([(x % 255) as u8, (y % 255) as u8, 150]);
            img.put_pixel(x, y, color);
        }
    }
    */
    todo!()
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Erstelle ein Bild mit dem image-Crate (z.B. 800x600, schwarz)
    let width = 800;
    let height = 600;
    let mut img = RgbImage::new(width, height);

    // --- print to image
    for _ in 0..100 {
        let shape = Shape::new_random(width, height);
        shape.draw(&mut img);
    }
    //draw_filled_circle_mut(&mut img, (100, 100), 200, Rgb([100, 200, 200]));
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &img);
    let window = create_window("image", Default::default())?;
    window.set_image("bild-001", image_view)?;

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flatten_pixels(input: &[[u8; 3]]) -> Vec<u8> {
        input.into_iter().copied().flatten().collect::<Vec<u8>>()
    }

    #[test]
    fn test_add() {
        let pixel = flatten_pixels(&[[5, 6, 5], [5, 5, 5]]);
        let pixel2 = flatten_pixels(&[[5, 5, 7], [5, 5, 0]]);

        let img1 = RgbImage::from_raw(2, 1, pixel).unwrap();
        let img2 = RgbImage::from_raw(2, 1, pixel2).unwrap();
        let d = calculate_difference(&img1, &img2);
        assert_eq!(d, 8);
    }
}
