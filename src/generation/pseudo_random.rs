extern crate rand;
use rand::prelude::*;
use crate::generation::Generator;

pub struct PseudoRandomGenerator(ThreadRng);

#[allow(dead_code)]
impl PseudoRandomGenerator {
    pub fn new() -> Self {
        Self(rand::thread_rng())
    }
}

impl Generator<f32> for PseudoRandomGenerator {
    fn at(&mut self, _: isize) -> f32 {
        self.0.gen()
    }
}