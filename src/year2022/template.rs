
pub fn part1(input: &str) -> usize {
	todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
	todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/dayn.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"";

    #[test]
    pub fn part1_example() {
        assert_eq!(0, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
	// 	assert_eq!(0, part2(EXAMPLE));
    // }
}
