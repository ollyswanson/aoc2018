use aoc08::{Node, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut flat: Vec<i32> = vec![];

    for n in input.split_ascii_whitespace() {
        flat.push(n.parse()?);
    }

    let node = Node::new(&flat)?;

    println!("{}", node.sum_metadata());
    println!("{}", node.sum_metadata_complex());

    Ok(())
}
