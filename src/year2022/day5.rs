type Item = u8;

#[derive(PartialEq, Eq, Debug)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
}

pub fn parse_crates(input: &str) -> Vec<Vec<Item>> {
    let unparsed_lines: Vec<_> = input
        .lines()
        .skip_while(|l| l.is_empty())
        .take_while(|l| !l.is_empty())
        .collect();
    let n_stacks = unparsed_lines
        .last()
        .expect("input non-empty")
        .split_ascii_whitespace()
        .last()
        .expect("stack headers separated by spaces")
        .parse()
        .expect("stack headers are ints");
    let mut stacks = vec![Vec::with_capacity(unparsed_lines.len()); n_stacks];

    for n in 0..n_stacks {
        let i_horizontal = (n * 4) + 1;
        for i_line in (0..unparsed_lines.len() - 1).rev() {
            let item = unparsed_lines[i_line].as_bytes()[i_horizontal];
            if item != b' ' {
                stacks[n].push(item);
            }
        }
    }

    stacks
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let unparsed_lines: Vec<_> = input
        .lines()
        .skip_while(|l| l.is_empty()) // allow extra newline at start
        .skip_while(|l| !l.is_empty()) // crates
        .filter(|l| !l.is_empty())
        .collect();

    unparsed_lines
        .iter()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_digit(10) || c.is_whitespace())
                .collect::<String>()
        })
        .map(|s| {
            let mut numbers = s.split_ascii_whitespace().map(|n| n.parse().unwrap());
            Instruction {
                count: numbers.next().unwrap(),
                from: numbers.next().unwrap(),
                to: numbers.next().unwrap(),
            }
        })
        .collect()
}

fn apply_instruction(crates: &mut Vec<Vec<Item>>, instruction: &Instruction) {
    for _ in 0..instruction.count {
        let item = crates[instruction.from - 1].pop().expect("stack non empty");
        crates[instruction.to - 1].push(item);
    }
}

pub fn part1(input: &str) -> String {
    let mut crates = parse_crates(input);
    let instructions = parse_instructions(input);

    for instruction in instructions.iter() {
        apply_instruction(&mut crates, instruction);
    }

    crates
        .iter()
        .map(|c| c.last().expect("stack non empty"))
        .map(|b| char::from(*b))
        .collect()
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day5.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    pub fn crate_parsing() {
        let crates = parse_crates(EXAMPLE);
        assert_eq!(3, crates.len());

        assert_eq!(vec![b'Z', b'N'], crates[0]);
        assert_eq!(vec![b'M', b'C', b'D'], crates[1]);
        assert_eq!(vec![b'P'], crates[2]);
    }

    #[test]
    pub fn instruction_parsing() {
        let instructions = parse_instructions(EXAMPLE);
        assert_eq!(4, instructions.len());

        assert_eq!(Instruction::new(1, 2, 1), instructions[0]);
        assert_eq!(Instruction::new(3, 1, 3), instructions[1]);
        assert_eq!(Instruction::new(2, 2, 1), instructions[2]);
        assert_eq!(Instruction::new(1, 1, 2), instructions[3]);
    }

    #[test]
    pub fn part1_example() {
        assert_eq!("CMZ", part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	todo!();
    // }
}
