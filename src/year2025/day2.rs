pub fn main() {
    let input = std::fs::read_to_string("input/2025/day2.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|r| r.split_once("-").unwrap())
        .flat_map(|(l, r)| l.parse::<usize>().unwrap()..=r.parse().unwrap())
        .filter(|n| {
            let s = format!("{n}");
            s.len() % 2 == 0 && {
                let (l, r) = s.split_at(s.len() / 2);
                l == r
            }
        })
        .sum()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    pub fn part1_example() {
        assert_eq!(1227775554, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
