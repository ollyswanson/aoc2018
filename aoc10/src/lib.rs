use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Points {
    points: Vec<Point>,
    pub second: u32,
}

impl Points {
    pub fn new(input: &str) -> Result<Self> {
        let mut points: Vec<Point> = Vec::new();

        for line in input.lines() {
            points.push(line.parse()?);
        }

        Ok(Points { points, second: 0 })
    }

    pub fn step(&mut self) {
        for p in self.points.iter_mut() {
            p.step();
        }
        self.second += 1;
    }

    pub fn bounds(&self) -> BoundingBox {
        let mut bb = BoundingBox {
            xmin: self.points[0].x,
            ymin: self.points[0].y,
            xmax: self.points[0].x,
            ymax: self.points[0].y,
        };

        for p in self.points.iter() {
            bb.xmin = cmp::min(p.x, bb.xmin);
            bb.ymin = cmp::min(p.y, bb.ymin);
            bb.xmax = cmp::max(p.x, bb.xmax);
            bb.ymax = cmp::max(p.x, bb.ymax);
        }

        bb
    }

    pub fn grid(&self) -> String {
        let bb = self.bounds();
        let mut grid = vec![vec![b'.'; bb.width()]; bb.height()];

        for p in self.points.iter() {
            let x = (p.x - bb.xmin) as usize;
            let y = (p.y - bb.ymin) as usize;
            grid[y][x] = b'#';
        }

        let mut buf = String::new();
        for row in grid {
            unsafe { buf.push_str(std::str::from_utf8_unchecked(&row)) };
            buf.push('\n');
        }
        buf
    }
}

pub struct BoundingBox {
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
}

impl BoundingBox {
    pub fn width(&self) -> usize {
        (self.xmax - self.xmin + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.ymax - self.ymin + 1) as usize
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl Point {
    /// Updates the position of the point in a unit time step
    pub fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)>
                \s+
                velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>
            "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => return Err(From::from("Unrecognized format for Point.")),
        };

        let p = Point {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            vx: caps["vx"].parse()?,
            vy: caps["vy"].parse()?,
        };

        Ok(p)
    }
}
