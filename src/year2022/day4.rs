use std::{ops::RangeInclusive, str::FromStr};

pub struct PairAssignments {
    pub left: RangeInclusive<u64>,
    pub right: RangeInclusive<u64>,
}

fn contains(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> bool {
    left.start() <= right.start() && left.end() >= right.end()
}

fn contains_symmetric(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> bool {
    contains(left, right) || contains(right, left)
}

fn overlap(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> bool {
    if left.start() == right.start() || left.end() == right.end() {
        true
    } else if left.start() < right.start() {
        right.start() <= left.end()
    } else {
        left.start() <= right.end()
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| PairAssignments::from_str(l).expect("line format"))
        .filter(|pair| contains_symmetric(&pair.left, &pair.right))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| PairAssignments::from_str(l).expect("line format"))
        .filter(|pair| overlap(&pair.left, &pair.right))
        .count()
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day4.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

impl FromStr for PairAssignments {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<u64> = s
            .split([',', '-'])
            .map(|c| c.parse().expect("pair input format"))
            .collect();

        let mut iter = splits.chunks(2).map(|c| RangeInclusive::new(c[0], c[1]));
        Ok(Self {
            left: iter.next().unwrap(),
            right: iter.next().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    pub fn part1_example() {
        assert_eq!(2, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(4, part2(EXAMPLE));
    }
}
