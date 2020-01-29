extern crate rand;
use rand::prelude::*;
use crate::generation::Generator;
use crate::fixed_size_pipe::FixedSizePipe;

const BUFFER_LENGTH: usize = 5;

type Pipe = FixedSizePipe<f32>;

pub struct RAGCache {
    zero_at: isize,
    values: FixedSizePipe<f32>,
}

impl RAGCache {
    fn get(&self, i: isize) -> Option<f32> {
        if self.zero_at <= i && i < self.zero_at + self.values.len() as isize {
            Some(self.values[(self.zero_at + i) as usize])
        } else {
            None
        }
    }

    fn recalculate_and_get(&mut self, i: isize, rng: &mut ThreadRng) -> f32 {
        self.zero_at = i - BUFFER_LENGTH as isize / 2;
        for _ in self.zero_at .. self.zero_at + BUFFER_LENGTH as isize {
            self.values.push_right(rng.gen())
        }
        self.values[0]
    }
}

pub struct RunningAverageGenerator {
    cache: Option<RAGCache>,
    rng: ThreadRng,
}

#[allow(dead_code)]
impl RunningAverageGenerator {
    pub fn new() -> Self {
        let mut pipe = Pipe::new(BUFFER_LENGTH);
        let mut rng = rand::thread_rng();
        for _ in 0..BUFFER_LENGTH {
            pipe.push_right(rng.gen())
        }

        Self {
            cache: None,
            rng: thread_rng(),
        }
    }
}

impl Generator<f32> for RunningAverageGenerator {
    fn at(&mut self, i: isize) -> f32 {
        if self.cache.is_none() {
            self.cache = Some(RAGCache {
                zero_at: 0,
                values: FixedSizePipe::new(BUFFER_LENGTH),
            })
        }

        let got = self.cache.as_ref().unwrap().get(i);
        got.unwrap_or_else(||
            self.cache.as_mut().unwrap().recalculate_and_get(i, &mut self.rng))
    }
}