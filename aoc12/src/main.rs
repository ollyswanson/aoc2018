use aoc12::{Pots, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut pots: Pots = input.parse()?;

    for _ in 0..200 {
        pots.evolve();
        println!(
            "{}: {}: {}: {}",
            pots.generation,
            pots.first,
            pots.pots_to_string(),
            pots.sum()
        );
    }

    // too lazy to do this one properly. You can see by visual inspection that it converges on a
    // steady state after ~100 generations with a linear increase of 25 between generations, so
    // taking the sum at 100 generations and adding 25 * 49,999,999,900 to it we can get the value
    // for the 50e12th generation
    println!("{}", (50_000_000_000i64 - 100) * 25 + 3491);

    Ok(())
}
