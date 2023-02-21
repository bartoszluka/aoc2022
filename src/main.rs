use anyhow::Result;
use std::fs::read_to_string;
mod days;

fn main() -> Result<()> {
    let input = read_to_string("inputs/day01")?;
    let max_sum = days::day01::part1(input)?;
    println!("{:?}", max_sum);
    Ok(())
}
