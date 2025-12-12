use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

pub fn main() {
    let input = std::fs::read_to_string("input/2025/day5.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    let (ranges, items) = parse(input);
    let ranges = ranges.collect::<Vec<_>>();

    items
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    let (ranges, _items) = parse(input);
    let ranges = ranges.collect::<Vec<_>>();
    let mut unmerged = ranges.clone();
    unmerged.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    let mut merged = Vec::<RangeInclusive<usize>>::new();

    let mut prev_len;
    loop {
        prev_len = unmerged.len();
        while let Some(to_merge) = unmerged.pop() {
            if let Some(existing) = merged
                .iter_mut()
                .find(|r| r.contains(to_merge.start()) || r.contains(to_merge.end()))
            {
                let start = min(*existing.start(), *to_merge.start());
                let end = max(*existing.end(), *to_merge.end());
                *existing = start..=end;
            } else {
                merged.push(to_merge);
            }
        }
        if merged.len() == prev_len {
            break;
        }
        std::mem::swap(&mut unmerged, &mut merged);
    }

    merged
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<usize>()
}

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = RangeInclusive<usize>>,
    impl Iterator<Item = usize>,
) {
    let (raw_ranges, raw_items) = input.split_once("\n\n").unwrap();

    let ranges = raw_ranges.split_ascii_whitespace().map(|raw_range| {
        let (l, r) = raw_range.split_once("-").unwrap();
        l.parse().unwrap()..=r.parse().unwrap()
    });

    let items = raw_items
        .split_ascii_whitespace()
        .map(|raw_item| raw_item.parse().unwrap());

    (ranges, items)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    pub fn part1_example() {
        assert_eq!(3, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(14, part2(EXAMPLE));
    }
}
