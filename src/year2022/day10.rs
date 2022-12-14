use std::collections::VecDeque;

struct Cpu {
    x: isize,
    instruction_buffer: VecDeque<Instruction>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add(isize),
    NoOp,
}

pub fn part1(input: &str) -> usize {
    todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day10.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_example() {
        let input = std::fs::read_to_string("input/2022/day10_example1.txt").unwrap();
        assert_eq!(13140, part1(&input));
    }

    // #[test]
    // pub fn part2_example() {
    // 	todo!();
    // }
}
