use std::{char, collections::HashSet};

use anyhow::Result;

const INPUT: &str = include_str!("files/day_3");
const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWxYZ";

fn main() -> Result<()> {
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
                .map(|c| CHARS.find(*c).unwrap() + 1)
                .sum::<usize>()
        })
        .sum();

    dbg!(score);

    Ok(())
}
