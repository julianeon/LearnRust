mod advent_of_code;

use std::io::Result as IOResult;
use advent_of_code::day_one::{count_floors, find_floor_minus_one};
use advent_of_code::day_two::{total_wrapping_paper_size, total_ribbon_size};

fn main() -> IOResult<()> {
    let count = total_wrapping_paper_size()?;
    println!("{}", count);

    Ok(())
}
