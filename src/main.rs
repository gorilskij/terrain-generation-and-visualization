mod visualization;
mod generation;
mod fixed_size_pipe;

fn main() {
    let generator = generation::RunningAverageGenerator::new();
    let mut visual = visualization::Visual::new(200, 10, generator);

    for _ in 0..200 {
        visual.advance()
    }
    println!("{}", visual)
}