const INPUT: &str = include_str!("files/day_4");

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let (min, max) = s.split_at(s.find("-").unwrap());
        let max = max.replace('-', "");
        let min = min.parse::<usize>().unwrap();
        let max = max.parse::<usize>().unwrap();

        Range { min, max }
    }

    fn fully_overlapping(&self, other: &Range) -> bool {
        (self.min <= other.min && self.max >= other.max)
            || (other.min <= self.min && other.max >= self.max)
    }

    fn partially_overlapping(&self, other: &Range) -> bool {
        (self.min >= other.min && self.min <= other.max)
            || (self.max >= other.min && self.max <= other.max)
            || (other.min >= self.min && other.min <= self.max)
            || (other.max >= self.min && other.max <= self.max)
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let count: usize = INPUT
        .lines()
        .into_iter()
        .map(|l| line_to_range_pair(&l))
        .filter(|(a, b)| a.fully_overlapping(b))
        .count();

    dbg!(count);
}

fn part_two() {
    let count: usize = INPUT
        .lines()
        .into_iter()
        .map(|l| line_to_range_pair(&l))
        .filter(|(a, b)| a.partially_overlapping(b))
        .count();

    dbg!(count);
}

fn line_to_range_pair(line: &str) -> (Range, Range) {
    let (a, b) = line.split_at(line.find(",").unwrap());
    let b = b.replace(",", "");
    let a = Range::from_str(&a);
    let b = Range::from_str(&b);

    (a, b)
}
