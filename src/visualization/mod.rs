use std::fmt::{Display, Formatter, Error};
use crate::generation::Generator;
use crate::fixed_size_pipe::FixedSizePipe;

const DOT_CHARACTER: char = '|';
const DOT_WIDTH: u8 = 2;

pub struct Visual {
    pipe: FixedSizePipe<f32>,
    height: usize,
    generator: Box<Generator<f32>>
}

impl Visual {
    pub fn new<G: 'static>(width: usize, height: usize, mut generator: G) -> Self
    where G : Generator<f32>
    {
        Visual {
            pipe: FixedSizePipe::new(width),
            height,
            generator: Box::new(generator)
        }
    }

    pub fn advance(&mut self) {
        self.pipe.push(self.generator.next())
    }
}

impl Display for Visual {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for line in (0..self.height).rev() {
            for column in 0..self.pipe.len() {
                let value = self.pipe[column] * self.height as f32;
                for _ in 0..DOT_WIDTH {
                    if value >= line as f32 {
                        write!(f, "{}", DOT_CHARACTER)?
                    } else {
                        write!(f, " ")?
                    }
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}