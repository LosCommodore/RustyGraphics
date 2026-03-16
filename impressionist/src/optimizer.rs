use crate::shape::Shape;
use image::Rgb;
use itertools::enumerate;

pub fn optimize_shape<F>(
    width: u32,
    height: u32,
    color: Rgb<u8>,
    initial_shape: &Shape,
    initial_score: u64,
    fitness_function: F,
) -> Option<(Shape, Rgb<u8>, u64)>
where
    F: Fn(&Shape, Rgb<u8>) -> u64,
{
    let mut best_shape = initial_shape.clone();
    let mut best_score = initial_score;

    for (i_point, point) in enumerate(&initial_shape.points) {
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
                        shape_type: initial_shape.shape_type,
                        points: new_points,
                    };
                    let score = fitness_function(&shape, color);
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
    Some((best_shape, color, best_score))
}
