use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("{}", reduce_polymer(&input, None));

    // skip a through z as well as reducing to find the length of the shortest possible chain
    let shortest = (b'a'..=b'z')
        .map(|b| reduce_polymer(&input, Some(b)))
        .min()
        .unwrap();

    println!("{}", shortest);

    Ok(())
}

type Stack<T> = Vec<T>;

fn reacts(a: u8, b: u8) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

// reduces the polymer by "reacting" all opposite pairs aA bB etc and by optionally skipping a pair
fn reduce_polymer(p: &str, skip: Option<u8>) -> usize {
    p.trim_end()
        .bytes()
        .fold(Stack::<u8>::new(), |mut stack, byte| match stack.last() {
            _ if skip == Some(byte.to_ascii_lowercase()) => stack,
            Some(&last) if reacts(last, byte) => {
                stack.pop();
                stack
            }
            _ => {
                stack.push(byte);
                stack
            }
        })
        .len()
}
