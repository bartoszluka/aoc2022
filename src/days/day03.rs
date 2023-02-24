use anyhow::Result;
use std::collections::{hash_map::RandomState, HashSet};

pub fn part1(input: &String) -> Result<i32> {
    Ok(input
        .trim_end()
        .split("\n")
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let backpack1: HashSet<char, RandomState> = HashSet::from_iter(first.chars());
            let backpack2 = HashSet::from_iter(second.chars());
            let in_both = backpack1
                .intersection(&backpack2)
                .last()
                .expect("that the intersection is not empty");
            char_value(*in_both)
        })
        .sum())
}

fn char_value(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        ch as i32 - ('a' as i32) + 1
    } else {
        ch as i32 - ('A' as i32) + 26 + 1
    }
}

pub fn part2(input: &String) -> Result<i32> {
    let sum_of_priorities = input
        .trim_end()
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|three_elves| {
            let backpack1: HashSet<char, RandomState> = HashSet::from_iter(three_elves[0].chars());
            let backpack2 = HashSet::from_iter(three_elves[1].chars());
            let backpack3: HashSet<char, RandomState> = HashSet::from_iter(three_elves[2].chars());
            let in_1_and_2 = backpack1.intersection(&backpack2);
            let in_1_and_2_set = HashSet::from_iter(in_1_and_2.into_iter().map(|c| *c));
            let in_all = backpack3.intersection(&in_1_and_2_set);
            let common_character = in_all.last().expect("the intersection is not empty");
            char_value(*common_character)
        })
        .sum();
    Ok(sum_of_priorities)
}
