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
    // valid ascii range is 0 - 127, therefore we can use the numeric values of the bytes that
    // represent each letter as the map to the frequencies
    let mut frequencies = [0u8; 128];
    let (mut twos, mut threes) = (0, 0);

    for line in input.lines() {
        // sanitize input to avoid panics
        if !line.is_ascii() {
            return Err(From::from("input must be ASCII"));
        }

        for f in frequencies.iter_mut() {
            *f = 0;
        }

        for b in line.bytes().map(|b| b as usize) {
            frequencies[b] += 1;
        }

        if frequencies.iter().any(|&f| f == 2) {
            twos += 1;
        }

        if frequencies.iter().any(|&f| f == 3) {
            threes += 1;
        }
    }

    println!("{}", twos * threes);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();

    for i in 0..ids.len() - 1 {
        for j in i + 1..ids.len() {
            if let Some(common) = common_letters(ids[i], ids[j]) {
                println!("{}", common);
            }
        }
    }

    Ok(())
}

fn common_letters(id1: &str, id2: &str) -> Option<String> {
    let mut one_wrong = false;

    for (c1, c2) in id1.bytes().zip(id2.bytes()) {
        if c1 != c2 {
            if one_wrong {
                return None;
            }
            one_wrong = true;
        }
    }

    Some(
        id1.chars()
            .zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}
