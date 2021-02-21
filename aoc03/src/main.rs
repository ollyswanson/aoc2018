use aoc03::{Claim, Fabric, Result};
use std::convert::TryInto;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut claims: Vec<Claim> = Vec::new();

    for line in input.lines() {
        // this should probably be turned into FromStr and line.parse()
        let claim: Claim = line.try_into()?;
        claims.push(claim);
    }

    let mut fabric = Fabric::with_side(1000);

    for claim in claims.iter() {
        fabric.make_claim(claim);
    }

    println!("{}", fabric.count_overlapping());

    for claim in claims.iter() {
        if fabric.claim_uncontested(claim) {
            println!("{}", claim.id);
        }
    }

    Ok(())
}
