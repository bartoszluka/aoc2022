use anyhow::{anyhow, Result};
use std::fs::read_to_string;
mod days;
use days::*;

fn main() -> Result<()> {
    print_day(1, day01::part1, day01::part2)?;
    print_day(2, day02::part1, day02::part2)?;
    Ok(())
}

fn print_day(
    day: i32,
    part1: fn(&String) -> Result<i32>,
    part2: fn(&String) -> Result<i32>,
) -> Result<()> {
    let formatted_day = if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    };
    let input = read_to_string(format!("inputs/day{formatted_day}")).map_err(|err| anyhow!(err))?;
    let day01_part1 = part1(&input)?;
    let day01_part2 = part2(&input)?;
    println!("day 1");
    println!("  part 1: {:?}", day01_part1);
    println!("  part 2: {:?}", day01_part2);
    Ok(())
}
