use aoc11::SummedArea;

const SERIAL: i32 = 4842;

fn main() {
    let cells = SummedArea::new(SERIAL);
    let maxima = cells.maxima();

    println!("({}, {})", maxima[1].1, maxima[1].2);

    let max = maxima.into_iter().max().unwrap();
    println!("({}, {}), {}", max.1, max.2, max.3);
}
