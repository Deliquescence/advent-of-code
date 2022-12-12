
pub fn part1(input: &str) -> usize {
	todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
	todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day8.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
30373
25512
65332
33549
35390";

    #[test]
    pub fn part1_example() {
        assert_eq!(21, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
	// 	todo!();
    // }
}
