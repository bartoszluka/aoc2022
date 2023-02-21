use anyhow::{anyhow, Result};

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

#[derive(Clone, Copy, Debug)]
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

    fn from_string(string: &str) -> Result<Self> {
        match string {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(anyhow!(format!("invalid string '{string}'"))),
        }
    }
}
use GameResult::*;

fn to_pair<T: Copy>(vec: Vec<T>) -> Result<(T, T)> {
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
    let rounds = input
        .trim_end()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|vec| to_pair(vec))
        .collect::<Result<Vec<_>>>()?;
    let points = rounds
        .iter()
        .map(|(first, second)| {
            let opponent = Rps::from_string(first)?;
            let game_result = GameResult::from_string(second)?;
            let player = match_result(opponent, game_result);
            Ok(player.get_points() + game_result.get_points())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(points.iter().sum())
}

fn match_result(opponent: Rps, game_result: GameResult) -> Rps {
    match (opponent, game_result) {
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (any, Draw) => any,
    }
}
