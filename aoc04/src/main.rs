use aoc04::{RawLogs, Result, ID};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let raw_logs = RawLogs::new(&input)?;
    let logs_by_guard = raw_logs.process_logs()?;

    let sleepiest_guard: ID = logs_by_guard.sleeps_most();
    let sm = logs_by_guard
        .sleepiest_minute_by_guard(sleepiest_guard)
        .unwrap();

    println!(
        "{}: {} - {}",
        sleepiest_guard,
        sm.minute,
        sleepiest_guard * sm.minute
    );

    let sm = logs_by_guard.sleepiest_minute().unwrap();

    println!("{}: {} - {}", sm.0, sm.1.minute, sm.0 * sm.1.minute);

    Ok(())
}
