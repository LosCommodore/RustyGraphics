use crate::{
    painting::{FitnessFn, OptimizerFn},
    shape::Shape,
};

#[allow(unused)]
pub fn no_optimizer<'a>(
    width: u32,
    height: u32,
    initial_shape: &Shape,
    initial_score: u64,
    fitness_function: &FitnessFn<'a>,
) -> (Shape, u64) {
    (initial_shape.clone(), initial_score)
}

// Dieser "Check" stellt sicher, dass die Signatur exakt passt:
const _: OptimizerFn = no_optimizer;
