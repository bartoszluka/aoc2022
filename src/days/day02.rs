use anyhow::{anyhow, Result};
use std::num::ParseIntError;

#[derive(Clone, Copy, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}
use Rps::*;

impl Rps {
    fn from_string(char: &str) -> Result<Self> {
        match char {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(anyhow!(format!("invalid string '{char}'"))),
        }
    }
    fn get_points(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Loss,
}
impl GameResult {
    fn get_points(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}
use GameResult::*;

// fn to_pair<T: Copy>(list: impl Iterator<Item = T>) -> Result<(T, T)> {
//     match list.collect::<Vec<T>>()[..] {
//         [first, second, ..] => Ok((first, second)),
//         _ => Err(anyhow!("list does not have exactly 2 elements")),
//     }
// }

fn to_pair(vec: Vec<Rps>) -> Result<(Rps, Rps)> {
    match &vec[..] {
        &[first, second, ..] => Ok((first, second)),
        _ => Err(anyhow!("list does not have exactly 2 elements")),
    }
}


fn game_result(opponent: &Rps, player: &Rps) -> GameResult {
    match (opponent, player) {
        (Rock, Paper) => Win,
        (Scissors, Paper) => Loss,
        (Scissors, Rock) => Win,
        (Paper, Rock) => Loss,
        (Paper, Scissors) => Win,
        (Rock, Scissors) => Loss,
        _ => Draw,
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let rounds = input
        .trim_end()
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(Rps::from_string)
                .collect::<Result<Vec<Rps>>>()
                .and_then(|vec| to_pair(vec))
        })
        .collect::<Result<Vec<(Rps, Rps)>>>()?;
    Ok(rounds
        .iter()
        .map(|(opponent, player)| player.get_points() + game_result(opponent, player).get_points())
        .sum())
}

pub fn part2(input: &String) -> Result<i32> {
    Ok(0)
}
