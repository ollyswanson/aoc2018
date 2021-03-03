pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct SummedArea {
    table: Vec<Vec<i32>>,
}

impl SummedArea {
    pub fn new(serial: i32) -> Self {
        // grid is 300 x 300 but is also indexed from 1, so rather than dealing with converting we
        // will just have an extra row and column and count from 1 as well.
        let size = 300;
        let mut table: Vec<Vec<i32>> = vec![vec![0; size + 1]];

        // a b
        // c d
        // d = b + c - a
        for y in 1..=size {
            let row: Vec<i32> = (0..=size)
                .scan(0, |acc, x| {
                    // if the value at (x,y) is d, then acc is equal to c
                    *acc += if x == 0 {
                        0
                    } else {
                        cell_power(x, y, serial) + table[y - 1][x] - table[y - 1][x - 1]
                    };
                    Some(*acc)
                })
                .collect();
            table.push(row);
        }

        SummedArea { table }
    }

    /// returns a vec of tuples where each tuple contains the max fuel for a given size of square
    /// and the coordinates that gave the maxima (fuel, x, y, size)
    pub fn maxima(&self) -> Vec<(i32, usize, usize, usize)> {
        let mut maxima = vec![];
        let table_size = self.table.len() - 1;

        for size in 2..=table_size {
            let mut max_fuel = i32::MIN;
            let mut coord = (0, 0);
            for y in 1..=table_size - size + 1 {
                for x in 1..=table_size - size + 1 {
                    let fuel = self.table[y + size - 1][x + size - 1] + self.table[y - 1][x - 1]
                        - self.table[y - 1][x + size - 1]
                        - self.table[y + size - 1][x - 1];
                    if fuel > max_fuel {
                        max_fuel = fuel;
                        coord = (x, y);
                    }
                }
            }
            maxima.push((max_fuel, coord.0, coord.1, size));
        }
        maxima
    }
}

fn cell_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut fuel: i32 = rack_id * y as i32 + serial;
    fuel *= rack_id;
    (fuel / 100) % 10 - 5
}
