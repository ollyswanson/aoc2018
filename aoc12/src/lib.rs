use lazy_static::lazy_static;
use regex::Regex;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Pots {
    transitions: Vec<Transition>,
    pots: Vec<Pot>,
    pub generation: u32,
    pub first: i32,
}

impl FromStr for Pots {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let prefix = "initial state: ";
        let mut lines = s.lines();

        let first_line = match lines.next() {
            Some(line) => line,
            None => {
                return Err(From::from("Empty input"));
            }
        };

        let mut pots: Vec<Pot> = vec![Pot::Empty; 4];
        let mut initial = first_line[prefix.len()..]
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Pot>>>()?;
        pots.append(&mut initial);
        pots.push(Pot::Empty);
        pots.push(Pot::Empty);
        pots.push(Pot::Empty);
        pots.push(Pot::Empty);

        lines.next();
        let transitions = lines
            .map(|l| l.parse())
            .collect::<Result<Vec<Transition>>>()?;

        Ok(Pots {
            first: -4,
            transitions,
            pots,
            generation: 0,
        })
    }
}

impl Pots {
    pub fn evolve(&mut self) {
        // assume that we always have a plant

        let first_plant = self.pots.iter().position(|&p| p == Pot::Plant).unwrap();
        let last_plant = self.pots.len()
            - 1
            - self
                .pots
                .iter()
                .rev()
                .position(|&p| p == Pot::Plant)
                .unwrap();
        // offset -4 for the 4 empty pots, first_plant - 2 as that's where we're starting the
        // iteration
        self.first += first_plant as i32 - 6;
        let mut next_gen: Vec<Pot> = Vec::with_capacity(self.pots.len());

        for _ in 0..4 {
            next_gen.push(Pot::Empty);
        }

        for i in first_plant - 2..last_plant + 2 {
            let to = self
                .transitions
                .iter()
                .find(|t| t.is_match(&self.pots[i - 2..i + 3]))
                .unwrap()
                .to;

            next_gen.push(to);
        }

        for _ in 0..4 {
            next_gen.push(Pot::Empty);
        }

        self.pots = next_gen;
        self.generation += 1;
    }

    pub fn sum(&self) -> i32 {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, &p)| p == Pot::Plant)
            .map(|(i, _)| i as i32 + self.first)
            .sum()
    }

    pub fn pots_to_string(&self) -> String {
        String::from_utf8(
            self.pots
                .iter()
                .map(|&p| match p {
                    Pot::Plant => b'#',
                    Pot::Empty => b'.',
                })
                .collect(),
        )
        .unwrap()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Pot {
    Plant,
    Empty,
}

impl TryFrom<char> for Pot {
    type Error = Box<dyn std::error::Error>;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Pot::Empty),
            '#' => Ok(Pot::Plant),
            _ => Err(From::from("Unrecognized symbol")),
        }
    }
}

pub struct Transition {
    pub from: Vec<Pot>,
    pub to: Pot,
}

impl Transition {
    pub fn is_match(&self, pots: &[Pot]) -> bool {
        self.from == pots
    }
}

// ..#.. => .
lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<from>[.#]{5}) => (?P<to>[.#]{1})$").unwrap();
}

impl FromStr for Transition {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => {
                return Err(From::from("Unrecognized pattern"));
            }
        };

        let from = caps["from"]
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Pot>>>()?;

        let to = caps["to"].chars().next().unwrap().try_into()?;

        Ok(Transition { from, to })
    }
}
