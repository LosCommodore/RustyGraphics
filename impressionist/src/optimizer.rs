use crate::shape::Shape;
use image::Rgb;
use imageproc::point::Point;
use itertools::{enumerate, izip};
use std::iter::repeat;

fn get_cross_directions_for_point(
    point: &Point<i32>,
    width: u32,
    height: u32,
) -> [Box<dyn Iterator<Item = (i32, i32)>>; 4] {
    let left = izip!((0..point.x as i32).rev(), repeat(point.y));
    let right = izip!((point.x + 1)..width as i32, repeat(point.y));
    let up = izip!(repeat(point.x), (0..point.y as i32).rev());
    let down = izip!(repeat(point.x), (point.y + 1)..height as i32);

    let directions: [Box<dyn Iterator<Item = (i32, i32)>>; 4] = [
        Box::new(left),
        Box::new(right),
        Box::new(up),
        Box::new(down),
    ];
    directions
}

pub fn walk_in_direction<I>(
    i_point: usize,
    direction: I,
    initial_shape: &Shape,
    color: Rgb<u8>,
    initial_score: u64,
    fitness_function: &dyn Fn(&Shape, Rgb<u8>) -> u64,
) -> (Shape, u64)
where
    I: IntoIterator<Item = (i32, i32)>,
{
    let mut points = initial_shape.points.clone();
    let mut best_score = initial_score;
    let mut best_shape = initial_shape.clone();

    for (x, y) in direction {
        points[i_point].x = x;
        points[i_point].y = y;

        let shape = Shape {
            shape_type: initial_shape.shape_type,
            points: points.clone(),
        };
        let score = fitness_function(&shape, color);
        if score > best_score {
            break;
        }
        best_score = score;
        best_shape = shape;
    }
    (best_shape, best_score)
}

pub fn cross_optimizer(
    width: u32,
    height: u32,
    color: Rgb<u8>,
    initial_shape: &Shape,
    initial_score: u64,
    fitness_function: &dyn Fn(&Shape, Rgb<u8>) -> u64,
) -> Option<(Shape, Rgb<u8>, u64)> {
    let mut best_shape = initial_shape.clone();
    let mut best_score = initial_score;

    for (i_point, point) in enumerate(&initial_shape.points) {
        let directions = get_cross_directions_for_point(point, width, height);

        for direction in directions {
            let (new_shape, new_score) = walk_in_direction(
                i_point,
                direction,
                initial_shape,
                color,
                initial_score,
                fitness_function,
            );
            if new_score < best_score {
                best_score = new_score;
                best_shape = new_shape;
            }
        }
    }
    Some((best_shape, color, best_score))
}
