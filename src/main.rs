mod visual;
mod generation;
mod fixed_size_pipe;

fn main() {
    let generator = {
        use generation::*;
//        RunningAverageGenerator::new()
//        PseudoRandomGenerator::new()
        SineGenerator::new()
    };

//    let mut visual = visual::TermVisual::new(200, 10, generator);
//        for _ in 0..200 { visual.advance() }
//    println!("{}", visual)

    let mut visual = visual::RendVisual::new(generator, 60, 90);
    visual.run();
}