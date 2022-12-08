use std::collections::HashSet;

const INPUT: &str = include_str!("files/day_6");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let bytes = INPUT.chars().collect::<Vec<char>>();

    for i in 4..bytes.len() {
        let last_4 = &bytes[i - 4..i];
        let set: HashSet<char> = HashSet::from_iter(last_4.iter().cloned());
        if set.len() == 4 {
            dbg!(i);
            break;
        }
    }
}

fn part_two() {
    let bytes = INPUT.chars().collect::<Vec<char>>();

    for i in 14..bytes.len() {
        let last_14 = &bytes[i - 14..i];
        let set: HashSet<char> = HashSet::from_iter(last_14.iter().cloned());
        if set.len() == 14 {
            dbg!(i);
            break;
        }
    }
}
