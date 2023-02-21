use anyhow::anyhow;
use anyhow::Result;
use std::num::ParseIntError;

pub fn part1(input: &String) -> Result<i32> {
    let calories = input
        .trim_end()
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .map(|num_as_str| num_as_str.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()
                .map(|numbers| numbers.iter().sum())
        })
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(|err| anyhow!(err))?;

    calories
        .iter()
        .max()
        .map(|x| x.to_owned())
        .ok_or(anyhow!("empty list!"))
}

pub fn part2(input: &String) -> Result<i32> {
    let mut calories = input
        .trim_end()
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .map(|num_as_str| num_as_str.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()
                .map(|numbers| numbers.iter().sum())
        })
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(|err| anyhow!(err))?;

    calories.sort_unstable_by(|a, b| b.cmp(a));
    let sum_3_larges = calories.iter().take(3).sum::<i32>();
    Ok(sum_3_larges)
}
