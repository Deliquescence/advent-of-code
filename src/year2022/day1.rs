pub struct Elf {
    calories: Vec<usize>,
}

// #[aoc_generator(day1)]
pub fn parse_elves(input: &str) -> Vec<Elf> {
    input
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(|cals| Elf {
            calories: cals
                .iter()
                .map(|c| c.parse().expect("input contains integers"))
                .collect(),
        })
        .collect()
}

impl Elf {
    pub fn total_calories(&self) -> usize {
        self.calories.iter().sum()
    }
}

// #[aoc(day1, part1, blah)]
pub fn solve_part1(input: &[Elf]) -> usize {
    input
        .iter()
        .map(Elf::total_calories)
        .max()
        .unwrap_or_default()
}

pub fn solve_part2(input: &[Elf]) -> usize {
    let mut calories: Vec<_> = input.iter().map(Elf::total_calories).collect();
    calories.sort_unstable_by_key(|c| std::cmp::Reverse(*c));
    calories.iter().take(3).sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day1.txt").unwrap();
    let elves = parse_elves(&input);
    let solution = solve_part2(&elves);

    println!("{}", solution);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn solve() {
//         let input = std::fs::read_to_string("input/2022/day1.txt").unwrap();
//         let elves = parse_elves(&input);
//         let soluton = solve_part1(&elves);
//     }
// }
