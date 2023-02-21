use anyhow::Result;
use std::num::ParseIntError;

pub enum Errors {
    ParsingError(ParseIntError),
    EmptyList,
}

pub fn part1(input: String) -> Result<i32> {
    let calories = input
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .map(|num_as_str| num_as_str.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()
                .map(|numbers| numbers.iter().sum())
        })
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(Errors::ParsingError)?;

    calories
        .iter()
        .max()
        .map(|x| x.to_owned())
        .ok_or(Errors::EmptyList)
}
