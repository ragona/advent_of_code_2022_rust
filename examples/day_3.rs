use itertools::Itertools;
use std::{char, collections::HashSet};

const INPUT: &str = include_str!("files/day_3");
const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWxYZ";

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let score: usize = INPUT
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<HashSet<char>>(),
                b.chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(left, right)| {
            left.intersection(&right)
                .map(|c| score_from_char(c))
                .sum::<usize>()
        })
        .sum();

    dbg!(score);
}

fn part_two() {
    let score: usize = INPUT
        .lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|team| {
            team.map(|s| s.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>()
        })
        .map(|sets| {
            score_from_char(
                sets[0]
                    .iter()
                    .filter(|&i| sets[1].contains(&i) && sets[2].contains(&i))
                    .next()
                    .unwrap(),
            )
        })
        .sum();

    dbg!(score);
}

fn score_from_char(c: &char) -> usize {
    CHARS.find(*c).unwrap() + 1
}
