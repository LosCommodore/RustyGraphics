mod no_optimizer;
mod simple_optimizer;
use crate::painting::OptimizerFn;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum OptimizerType {
    None,
    Cross,
}

impl OptimizerType {
    pub fn get_fn(&self) -> OptimizerFn {
        match self {
            OptimizerType::None => no_optimizer::no_optimizer,
            OptimizerType::Cross => simple_optimizer::cross_optimizer,
        }
    }
}
