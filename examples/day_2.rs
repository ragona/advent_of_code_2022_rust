const INPUT: &str = include_str!("files/day_2");

#[derive(Debug, Copy, Clone)]
enum Moves {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Debug, Copy, Clone)]
enum Outcomes {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

impl From<&str> for Moves {
    fn from(value: &str) -> Self {
        match value.trim() {
            "A" | "X" => Moves::ROCK,
            "B" | "Y" => Moves::PAPER,
            "C" | "Z" => Moves::SCISSORS,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Outcomes {
    fn from(value: &str) -> Self {
        match value.trim() {
            "X" => Outcomes::LOSE,
            "Y" => Outcomes::DRAW,
            "Z" => Outcomes::WIN,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut score = 0;

    for line in INPUT.lines() {
        let (a, b) = line.split_at(line.find(" ").unwrap());

        let a = Moves::from(a);
        let b = Moves::from(b);

        let outcome = match (&b, &a) {
            (Moves::ROCK, Moves::ROCK) => Outcomes::DRAW,
            (Moves::ROCK, Moves::PAPER) => Outcomes::LOSE,
            (Moves::ROCK, Moves::SCISSORS) => Outcomes::WIN,
            (Moves::PAPER, Moves::ROCK) => Outcomes::WIN,
            (Moves::PAPER, Moves::PAPER) => Outcomes::DRAW,
            (Moves::PAPER, Moves::SCISSORS) => Outcomes::LOSE,
            (Moves::SCISSORS, Moves::ROCK) => Outcomes::LOSE,
            (Moves::SCISSORS, Moves::PAPER) => Outcomes::WIN,
            (Moves::SCISSORS, Moves::SCISSORS) => Outcomes::DRAW,
        };

        score += b as usize + outcome as usize;
    }

    dbg!(score);
}

fn part_two() {
    let mut score = 0;

    for line in INPUT.lines() {
        let (a, b) = line.split_at(line.find(" ").unwrap());

        let their_move = Moves::from(a);
        let outcome = Outcomes::from(b);
        let our_move = match outcome {
            Outcomes::DRAW => their_move.clone(),
            Outcomes::LOSE => match their_move {
                Moves::ROCK => Moves::SCISSORS,
                Moves::PAPER => Moves::ROCK,
                Moves::SCISSORS => Moves::PAPER,
            },
            Outcomes::WIN => match their_move {
                Moves::ROCK => Moves::PAPER,
                Moves::PAPER => Moves::SCISSORS,
                Moves::SCISSORS => Moves::ROCK,
            },
        };

        score += our_move as usize + outcome as usize;
    }

    dbg!(score);
}
