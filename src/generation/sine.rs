use crate::generation::Generator;

pub struct SineGenerator(f32);

#[allow(dead_code)]
impl SineGenerator {
    pub fn new() -> Self {
        Self(0.0)
    }
}

impl Generator<f32> for SineGenerator {
    fn next(&mut self) -> f32 {
        let value = self.0.sin() / 2. + 0.5;
        self.0 += 0.1;
        value
    }
}