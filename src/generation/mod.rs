mod pseudo_random;
mod sine;
mod running_average;

pub use pseudo_random::PseudoRandomGenerator;
pub use sine::SineGenerator;
pub use running_average::RunningAverageGenerator;

// expects to be normalized [0,1]
pub trait Generator<T> {
    fn at(&mut self, i: isize) -> T;
}