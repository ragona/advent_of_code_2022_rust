use std::cmp::Reverse;

use anyhow::Result;

const INPUT: &str = include_str!("files/day_1");

fn main() -> Result<()> {
    let mut elves: Vec<usize> = INPUT
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|i| i.parse::<usize>())
                .sum()
        })
        .collect::<Vec<usize>>();

    elves.sort_by_key(|i| Reverse(*i));

    let first = elves.first().unwrap();
    let top_3: usize = elves.iter().take(3).sum();

    println!("{}, {}", first, top_3);

    Ok(())
}
