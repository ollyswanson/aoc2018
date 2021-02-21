use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// a square piece of fabric, each u8 corresponds to a 1x1 bin, with the bins covering the fabric
pub struct Fabric {
    grid: Vec<u8>,
    side: u32,
}

impl Fabric {
    pub fn with_side(side: u32) -> Self {
        let nbins = side * side;

        Fabric {
            grid: vec![0; nbins as usize],
            side,
        }
    }

    // increments every bin in the grid that the claim claims by 1
    pub fn make_claim(&mut self, claim: &Claim) {
        for point in claim.iter_points() {
            let pos = point.0 + self.side * point.1;
            self.grid[pos as usize] += 1;
        }
    }

    // counts the number of bins that have 2 or more claims
    pub fn count_overlapping(&self) -> u32 {
        self.grid.iter().filter(|&count| *count > 1).count() as u32
    }

    pub fn claim_uncontested(&self, claim: &Claim) -> bool {
        for point in claim.iter_points() {
            let pos = point.0 + self.side * point.1;
            if self.grid[pos as usize] != 1 {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl TryFrom<&str> for Claim {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self> {
        // input of form #1 @ 45,64: 22x22
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
                "
            )
            .unwrap();
        }

        let caps = match RE.captures(input) {
            Some(caps) => caps,
            None => return Err(From::from("unrecognized format for claim")),
        };

        Ok(Claim {
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}

impl Claim {
    pub fn iter_points(&self) -> IterPoints {
        IterPoints {
            claim: self,
            px: self.x,
            py: self.y,
        }
    }
}

pub struct IterPoints<'c> {
    claim: &'c Claim,
    px: u32,
    py: u32,
}

impl<'c> Iterator for IterPoints<'c> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.px >= self.claim.x + self.claim.width {
            self.px = self.claim.x;
            self.py += 1;
        }

        if self.py >= self.claim.y + self.claim.height {
            return None;
        }

        let (px, py) = (self.px, self.py);
        self.px += 1;

        Some((px, py))
    }
}
