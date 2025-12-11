pub fn main() {
    let input = std::fs::read_to_string("input/2025/day3.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .map(|bank| largest(bank, 2))
        .sum()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .map(|bank| largest(bank, 12))
        .sum()
}

fn largest(bank: &str, len: usize) -> usize {
    let bank = bank.as_bytes();
    let mut max = vec![0_u8; len];
    'outer: for i_b in 0..bank.len() {
        let min_i_m = len.saturating_sub(bank.len() - i_b);
        for i_m in min_i_m..len {
            if bank[i_b + i_m - min_i_m] > max[i_m] {
                max[i_m..].copy_from_slice(&bank[i_b + i_m - min_i_m..i_b + len - min_i_m]);
                continue 'outer;
            }
        }
    }
    String::from_utf8(max).unwrap().parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    pub fn part1_example() {
        assert_eq!(357, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(3121910778619, part2(EXAMPLE));
    }
}
