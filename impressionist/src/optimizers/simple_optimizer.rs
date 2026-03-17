use crate::{
    painting::{FitnessFn, OptimizerFn},
    shape::Shape,
};
use imageproc::point::Point;
use itertools::{enumerate, izip};
use rayon::prelude::*;
use std::iter::repeat;

fn get_cross_directions_for_point(
    point: &Point<i32>,
    width: u32,
    height: u32,
) -> Vec<Vec<(i32, i32)>> {
    // Gib fertige Datenpakete zurück
    let px = point.x;
    let py = point.y;

    vec![
        izip!((0..px).rev().step_by(2), repeat(py)).collect(),
        izip!((px + 1..width as i32).step_by(2), repeat(py)).collect(),
        izip!(repeat(px), (0..py).rev().step_by(2)).collect(),
        izip!(repeat(px), (py + 1..height as i32).step_by(2)).collect(),
    ]
}

pub fn walk_in_direction<I>(
    i_point: usize,
    direction: I,
    initial_shape: &Shape,
    initial_score: u64,
    fitness_function: &FitnessFn,
) -> (Shape, u64)
where
    I: IntoIterator<Item = (i32, i32)>,
{
    let mut points = initial_shape.points.clone();
    let mut best_score = initial_score;
    let mut best_shape = initial_shape.clone();

    let mut count_steps: i32 = 0;
    println!("  Initial score {initial_score}");

    for (x, y) in direction {
        count_steps += 1;
        points[i_point].x = x;
        points[i_point].y = y;

        let shape = Shape {
            points: points.clone(),
            ..*initial_shape
        };
        let score = fitness_function(&shape);
        if score > best_score {
            break;
        }
        best_score = score;
        best_shape = shape;
    }
    println!("Step count: {count_steps} / score: {best_score}");
    (best_shape, best_score)
}

pub fn cross_optimizer<'a>(
    width: u32,
    height: u32,
    initial_shape: &Shape,
    initial_score: u64,
    fitness_function: &FitnessFn<'a>,
) -> (Shape, u64) {
    let mut best_shape = initial_shape.clone();
    let mut best_score = initial_score;

    for (i_point, point) in enumerate(&initial_shape.points) {
        let directions = get_cross_directions_for_point(point, width, height);

        let best = directions
            .into_par_iter()
            .map(|direction: Vec<(i32, i32)>| {
                walk_in_direction(
                    i_point,
                    direction,
                    initial_shape,
                    initial_score,
                    fitness_function,
                )
            })
            .min_by_key(|res| res.1);

        if let Some((shape, score)) = best {
            best_shape = shape;
            best_score = score;
        }
    }
    (best_shape, best_score)
}

// Dieser "Check" stellt sicher, dass die Signatur exakt passt:
const _: OptimizerFn = cross_optimizer;
