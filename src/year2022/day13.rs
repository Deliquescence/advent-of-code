
pub fn part1(input: &str) -> usize {
	todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
	todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day13.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
	// 	assert_eq!(0, part2(EXAMPLE));
    // }
}
