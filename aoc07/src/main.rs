use aoc07::{Adjacent, Graph, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut adjacencies: Vec<Adjacent> = Vec::new();

    for line in input.lines() {
        adjacencies.push(line.parse()?);
    }

    let graph = Graph::new(&adjacencies);
    let mut order: Vec<u8> = vec![];
    graph.step_order(&mut order);
    println!("{}", std::str::from_utf8(&order)?);
    order.clear();
    let duration = graph.with_workers(&mut order);
    println!("{} {}", std::str::from_utf8(&order)?, duration);

    Ok(())
}
