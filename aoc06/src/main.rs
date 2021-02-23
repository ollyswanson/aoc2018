use std::cmp;
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut locations: Vec<Coordinate> = Vec::new();

    for line in input.lines() {
        let location: Coordinate = line.parse()?;
        locations.push(location);
    }

    let bb = BoundingBox::new(&locations)?;
    let mut area_sizes = vec![0; locations.len()];

    for p in bb.points() {
        let dists: Vec<_> = locations.iter().map(|loc| loc.distance(&p)).collect();
        let &min_dist = dists.iter().min().unwrap();
        let v: Vec<_> = dists
            .into_iter()
            .enumerate()
            .filter(|t| t.1 == min_dist)
            .map(|t| t.0)
            .collect();

        // only count the point if it has one and only one closest location
        if v.len() == 1 {
            let i = v[0];
            if bb.on_edge(&p) {
                // if the point is on the border then it has an infinite area and we can use a
                // sentinel value of i32::MIN to remove it from running
                area_sizes[i] = i32::MIN;
            } else {
                area_sizes[i] += 1;
            }
        }
    }

    println!("{}", area_sizes.iter().max().unwrap());

    // for part 2 will make assumption that all points that have have a cumulative manhattan
    // distance to all locations of less than 10_000 are 1) within the bounding box and 2)
    // contiguous.

    let area = bb
        .points()
        .map(|p| locations.iter().map(|loc| loc.distance(&p)).sum::<i32>())
        .filter(|&d| d < 10_000)
        .count();

    println!("{}", area);

    Ok(())
}

// aaaaa.cccc
// aAaaa.cccc
// aaaddecccc
// aadddeccCc
// ..dDdeeccc
// bb.deEeecc
// bBb.eeee..
// bbb.eeefff
// bbb.eeffff
// bbb.ffffFf
//
// hypothesis: if a location (A, B, C) is on the bounding box, or is the nearest coordinate to a point on
// the bounding box then its area will extend infinitely

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    // Manhattan distance between self and c
    fn distance(&self, c: &Coordinate) -> i32 {
        (self.x - c.x).abs() + (self.y - c.y).abs()
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Coordinate> {
        if let Some(i) = s.find(",") {
            let x: i32 = s[..i].trim().parse()?;
            let y: i32 = s[i + 1..].trim().parse()?;

            return Ok(Coordinate { x, y });
        }

        Err(From::from("input line does not contain comma"))
    }
}

#[derive(Debug)]
struct BoundingBox {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

impl BoundingBox {
    fn new(locations: &[Coordinate]) -> Result<Self> {
        let mut loc_iter = locations.iter();

        let mut bb = if let Some(loc) = loc_iter.next() {
            BoundingBox {
                xmin: loc.x,
                xmax: loc.x,
                ymin: loc.y,
                ymax: loc.y,
            }
        } else {
            return Err(From::from("No locations"));
        };

        for loc in loc_iter {
            bb.xmin = cmp::min(bb.xmin, loc.x);
            bb.xmax = cmp::max(bb.xmax, loc.x);
            bb.ymin = cmp::min(bb.ymin, loc.y);
            bb.ymax = cmp::max(bb.ymax, loc.y);
        }

        Ok(bb)
    }

    /// Returns an iterator over all of the points in (including edge) the bounding box
    fn points(&self) -> impl Iterator<Item = Coordinate> {
        let (xmin, xmax) = (self.xmin, self.xmax); // lifetime shenanigans
        (self.ymin..=self.ymax).flat_map(move |y| (xmin..=xmax).map(move |x| Coordinate { x, y }))
    }

    fn on_edge(&self, c: &Coordinate) -> bool {
        if c.x == self.xmin || c.x == self.xmax || c.y == self.ymin || c.y == self.ymax {
            return true;
        }
        false
    }
}
