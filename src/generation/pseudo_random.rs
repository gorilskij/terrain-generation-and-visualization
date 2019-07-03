extern crate rand;
use rand::prelude::*;
use crate::generation::Generator;

pub struct PseudoRandomGenerator(ThreadRng);

impl PseudoRandomGenerator {
    pub fn new() -> Self {
        Self(rand::thread_rng())
    }
}

impl Generator<f32> for PseudoRandomGenerator {
    fn next(&mut self) -> f32 {
        self.0.gen::<f32>()
    }
}