use crate::generation::Generator;

pub struct SineGenerator;

impl Generator<f32> for SineGenerator {
    fn at(&mut self, i: isize) -> f32 {
        (i as f32 / 10.).sin() / 2. + 0.5
    }
}