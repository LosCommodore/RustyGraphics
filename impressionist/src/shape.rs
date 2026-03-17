use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_line_segment_mut, draw_polygon_mut};
use imageproc::point::Point;
use rand::RngExt;
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum ShapeType {
    Ellipse,
    Triangle,
    Quadrinial,
    Line,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub points: Vec<Point<i32>>,
    pub color: Rgb<u8>,
}

#[allow(unused)]
impl Shape {
    pub fn new_random_position(
        shape_type: ShapeType,
        screen_width: u32,
        screen_height: u32,
        color: Rgb<u8>,
    ) -> Self {
        let num_points = match shape_type {
            ShapeType::Line => 2,
            ShapeType::Ellipse => 2,
            ShapeType::Triangle => 3,
            ShapeType::Quadrinial => 4,
        };

        let points = (0..num_points)
            .map(|_| random_point(screen_width, screen_height))
            .collect();
        Self {
            shape_type,
            points,
            color,
        }
    }

    pub fn new_random_type(screen_width: u32, screen_height: u32, color: Rgb<u8>) -> Self {
        let mut rng = rand::rng();
        let choices = [
            ShapeType::Ellipse,
            ShapeType::Line,
            ShapeType::Triangle,
            ShapeType::Quadrinial,
        ];

        let shape_type = choices.choose(&mut rng).unwrap();
        Shape::new_random_position(*shape_type, screen_width, screen_height, color)
    }

    pub fn draw(&self, canvas: &mut RgbImage) {
        match self.shape_type {
            ShapeType::Ellipse => {
                // convert bounding-box points to -> center & radius
                let delta = self.points[1] - self.points[0];
                let dx = delta.x / 2;
                let dy = delta.y / 2;
                let center = (self.points[0].x + dx, self.points[0].y + dy);
                draw_filled_ellipse_mut(canvas, center, dx.abs(), dy.abs(), self.color);
            }

            ShapeType::Line => {
                let start = (self.points[0].x as f32, self.points[0].y as f32);
                let end = (self.points[1].x as f32, self.points[1].y as f32);
                draw_line_segment_mut(canvas, start, end, self.color);
            }
            ShapeType::Triangle => {
                draw_polygon_mut(canvas, &self.points, self.color);
            }
            ShapeType::Quadrinial => {
                let mut sort_points = self.points.clone();
                sort_points_clockwise(&mut sort_points);
                draw_polygon_mut(canvas, &sort_points, self.color);
            }
        }
    }
}

fn random_point(screen_width: u32, screen_height: u32) -> Point<i32> {
    let mut rng = rand::rng();
    let x = rng.random_range(0..screen_width) as i32;
    let y = rng.random_range(0..screen_height) as i32;
    Point { x, y }
}

fn sort_points_clockwise(points: &mut [Point<i32>]) {
    // 1. Mittelpunkt berechnen
    let m_x: i32 = points.iter().map(|p| p.x).sum::<i32>() / points.len() as i32;
    let m_y: i32 = points.iter().map(|p| p.y).sum::<i32>() / points.len() as i32;

    // 2. Nach Polarwinkel sortieren
    points.sort_by(|a, b| {
        let angle_a = ((a.y - m_y) as f64).atan2((a.x - m_x) as f64);
        let angle_b = ((b.y - m_y) as f64).atan2((b.x - m_x) as f64);

        angle_a.partial_cmp(&angle_b).unwrap()
    });
}

#[allow(unused)]
fn random_color() -> Rgb<u8> {
    let mut rng = rand::rng();
    let colors: [u8; 3] = std::array::from_fn(|_| rng.random_range(0..=255));
    Rgb(colors)
}
