
pub fn part1(input: &str) -> usize {
	todo!();
}

pub fn part2(input: &str) -> usize {
	todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day7.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    pub fn part1_example() {
        assert_eq!(95437, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
	// 	todo!();
    // }
}
