use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let freq = input
        .lines()
        .try_fold(0, |acc, cur| match cur.parse::<i32>() {
            Ok(cur) => Ok(acc + cur),
            Err(e) => return Err(e),
        })?;

    println!("{}", freq);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let delta: i32 = line.parse()?;
            freq += delta;

            if seen.contains(&freq) {
                println!("{}", freq);
                return Ok(());
            } else {
                seen.insert(freq);
            }
        }
    }
}
