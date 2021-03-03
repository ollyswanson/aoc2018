use aoc10::{Points, Result};
use std::io::{self, Read};

const MAX_HEIGHT: usize = 100;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut points = Points::new(&input)?;

    for _ in 0..1_000_000 {
        let bb = points.bounds();
        if bb.height() < MAX_HEIGHT {
            println!("{}", points.grid());
            println!("{}", points.second);
        }
        points.step();
    }
    Ok(())
}
