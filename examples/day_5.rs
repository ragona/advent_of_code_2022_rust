use std::{char, ops::Index};

const INPUT: &str = include_str!("files/day_5");

fn main() {
    part_one();
    part_two();
}

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: vec![] }
    }

    fn push(&mut self, c: char) {
        self.crates.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }
}

impl Index<usize> for Stack {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.crates[index]
    }
}

#[derive(Debug)]
struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let digits = value
            .chars()
            .filter(|c| !c.is_alphabetic())
            .collect::<String>()
            .split(" ")
            .flat_map(|c| c.parse::<usize>())
            .collect::<Vec<usize>>();

        Command {
            quantity: digits[0],
            from: digits[1] - 1,
            to: digits[2] - 1,
        }
    }
}

fn part_one() {
    let mut stacks: Vec<Stack> = build_stacks();
    for line in INPUT.lines().skip_while(|l| l.chars().nth(0) != Some('m')) {
        let command = Command::from(line);

        for _ in 0..command.quantity {
            let item = stacks[command.from].pop();

            if let Some(item) = item {
                stacks[command.to].push(item);
            }
        }
    }

    let top = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>();

    dbg!(top);
}

fn part_two() {
    let mut stacks: Vec<Stack> = build_stacks();
    for line in INPUT.lines().skip_while(|l| l.chars().nth(0) != Some('m')) {
        let command = Command::from(line);
        let mut stack_to_move: Vec<char> = vec![];

        for _ in 0..command.quantity {
            let item = stacks[command.from].pop();
            if let Some(i) = item {
                stack_to_move.push(i);
            }
        }

        stack_to_move.reverse();

        for item in stack_to_move {
            stacks[command.to].push(item);
        }
    }

    let top = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap_or_default())
        .collect::<String>();

    dbg!(top);
}

fn build_stacks() -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![Stack::new(); 9];

    let mut crate_lines = INPUT
        .lines()
        .take_while(|l| l.chars().nth(1) != Some('1'))
        .collect::<Vec<&str>>();

    crate_lines.reverse();

    for line in crate_lines {
        for (i, c) in line.chars().skip(1).enumerate().filter(|(i, _)| i % 4 == 0) {
            if c != ' ' {
                stacks[i / 4].push(c);
            }
        }
    }

    stacks
}
