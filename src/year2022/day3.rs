use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Item(u8);

pub struct Rucksack {
    first_half: HashSet<Item>,
    second_half: HashSet<Item>,
}

impl Rucksack {
    pub fn misplaced_item(&self) -> Option<Item> {
        self.first_half
            .iter()
            .find(|&i| self.second_half.contains(i))
            .copied()
    }

    pub fn contains(&self, item: &Item) -> bool {
        self.first_half.contains(item) || self.second_half.contains(item)
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.first_half.iter().chain(self.second_half.iter())
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let size = bytes.len() / 2;

        Ok(Self {
            first_half: parse_items(&bytes[..size]),
            second_half: parse_items(&bytes[size..]),
        })
    }
}

fn parse_items<'a>(bytes: impl IntoIterator<Item = &'a u8>) -> HashSet<Item> {
    bytes.into_iter().map(|b| Item::from_ascii(*b)).collect()
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() {
            Ok(Self::from_ascii(s.as_bytes()[0]))
        } else {
            Err(())
        }
    }
}

impl Item {
    pub fn from_ascii(a: u8) -> Self {
        if a <= b'Z' {
            Self(a.checked_sub(38).unwrap())
        } else {
            Self(a.checked_sub(96).unwrap())
        }
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.parse().expect("input in correct format"))
        .map(|s: Rucksack| u64::from(s.misplaced_item().expect("always 1 misplaced").0))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let sacks: Vec<Rucksack> = input
        .lines()
        .map(|l| l.parse().expect("input in correct format"))
        .collect();

    sacks
        .chunks_exact(3)
        .map(|c| c[0].items().find(|i| c[1].contains(i) && c[2].contains(i)))
        .map(|i| u64::from(i.expect("each group has item in common").0))
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day3.txt").unwrap();
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn priority() {
        assert_eq!(1, "a".parse::<Item>().unwrap().0);
        assert_eq!(16, "p".parse::<Item>().unwrap().0);
        assert_eq!(26, "z".parse::<Item>().unwrap().0);
        assert_eq!(27, "A".parse::<Item>().unwrap().0);
        assert_eq!(38, "L".parse::<Item>().unwrap().0);
        assert_eq!(42, "P".parse::<Item>().unwrap().0);
        assert_eq!(52, "Z".parse::<Item>().unwrap().0);
    }

    macro_rules! assert_sack_contains {
        ($expected: literal, $half: expr) => {
            assert!($half.contains(&Item::from_ascii($expected[0])));
        };
    }

    #[test]
    pub fn rucksack_parse() {
        let sack: Rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".parse().unwrap();
        // "vJrwpWtwJgWr";
        // "hcsFMMfFFhFp";

        assert_sack_contains!(b"v", sack.first_half);
        assert_sack_contains!(b"J", sack.first_half);
        assert_sack_contains!(b"r", sack.first_half);
        assert_sack_contains!(b"w", sack.first_half);
        assert_sack_contains!(b"p", sack.first_half);
        assert_sack_contains!(b"W", sack.first_half);

        assert_sack_contains!(b"h", sack.second_half);
        assert_sack_contains!(b"c", sack.second_half);
        assert_sack_contains!(b"s", sack.second_half);
        assert_sack_contains!(b"p", sack.second_half);
    }

    macro_rules! assert_misplaced_item {
        ($expected: literal, $sack: literal) => {
            assert_eq!(Item::from_ascii($expected[0]), get_misplaced_item(&$sack));
        };
    }

    #[test]
    pub fn misplaced_item() {
        assert_misplaced_item!(b"p", "vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_misplaced_item!(b"L", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_misplaced_item!(b"P", "PmmdzqPrVvPwwTWBwg");
        assert_misplaced_item!(b"v", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_misplaced_item!(b"t", "ttgJtRGJQctTZtZT");
        assert_misplaced_item!(b"s", "CrZsJsPPZsGzwwsLwLmpwMDw");
    }

    fn get_misplaced_item(input: &str) -> Item {
        let sack: Rucksack = input.parse().unwrap();
        sack.misplaced_item().unwrap()
    }

    const EXAMPLE_SACKS: &'static str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn part1_example() {
        let result = part1(EXAMPLE_SACKS);
        assert_eq!(157, result);
    }

    #[test]
    pub fn part2_example() {
        let result = part2(EXAMPLE_SACKS);
        assert_eq!(70, result);
    }
}
