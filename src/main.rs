use anyhow::{anyhow, Result};
use std::fs::read_to_string;
mod days;
use days::*;

fn main() -> Result<()> {
    print_day(1, day01::part1, 74711, day01::part2, 209481)?;
    print_day(2, day02::part1, 9241, day02::part2, 14610)?;
    print_day(3, day03::part1, 7872, day03::part2, 2497)?;
    Ok(())
}

fn print_day(
    day: i32,
    part1: fn(&String) -> Result<i32>,
    part1_expected: i32,
    part2: fn(&String) -> Result<i32>,
    part2_expected: i32,
) -> Result<()> {
    let formatted_day = if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    };
    let input = read_to_string(format!("inputs/day{formatted_day}")).map_err(|err| anyhow!(err))?;
    let part1_result = part1(&input)?;
    let part2_result = part2(&input)?;
    println!("day {day}");
    println!("  part 1: {}", compare(part1_result, part1_expected));
    println!("  part 2: {}", compare(part2_result, part2_expected));
    Ok(())
}

fn compare<T: PartialEq + std::fmt::Debug>(got: T, expected: T) -> String {
    if got == expected {
        format!("✅ {:?}", got)
    } else {
        format!("🟥 got {:?} expected {:?}", got, expected)
    }
}
