extern crate rand;
use rand::prelude::*;
use crate::generation::Generator;
use crate::fixed_size_pipe::FixedSizePipe;

const BUFFER_LENGTH: usize = 5;

type Pipe = FixedSizePipe<f32>;

pub struct RunningAverageGenerator(Pipe, ThreadRng);

#[allow(dead_code)]
impl RunningAverageGenerator {
    pub fn new() -> Self {
        let mut pipe = Pipe::new(BUFFER_LENGTH);
        let mut rng = rand::thread_rng();
        for _ in 0..BUFFER_LENGTH {
            pipe.push(rng.gen())
        }
        Self(pipe, rng)
    }
}

impl Generator<f32> for RunningAverageGenerator {
    fn next(&mut self) -> f32 {
        let average = self.0.average();
        self.0.push(self.1.gen());
        average
    }
}