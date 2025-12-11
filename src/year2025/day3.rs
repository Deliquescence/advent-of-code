pub fn main() {
    let input = std::fs::read_to_string("input/2025/day3.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .map(|bank| {
            let bank = bank.as_bytes();
            let mut lmax = 0;
            let mut rmax = 0;
            for i in 0..bank.len() - 1 {
                if bank[i] > lmax {
                    lmax = bank[i];
                    rmax = bank[i + 1];
                } else if bank[i] > rmax {
                    rmax = bank[i];
                }
            }
            if bank[bank.len() - 1] > rmax {
                rmax = bank[bank.len() - 1];
            }
            String::from_utf8(vec![lmax, rmax])
                .unwrap()
                .parse::<usize>()
                .unwrap()
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

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    pub fn part1_example() {
        assert_eq!(357, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
