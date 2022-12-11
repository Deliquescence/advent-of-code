pub fn part1(input: &str) -> usize {
    todo!();
}

pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day6.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &'static str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &'static str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &'static str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &'static str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    pub fn part1_examples() {
        assert_eq!(7, part1(EXAMPLE1));
        assert_eq!(5, part1(EXAMPLE2));
        assert_eq!(6, part1(EXAMPLE3));
        assert_eq!(10, part1(EXAMPLE4));
        assert_eq!(11, part1(EXAMPLE5));
    }

    #[test]
    pub fn part2_example() {
		todo!();
    }
}
