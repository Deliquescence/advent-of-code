use std::collections::VecDeque;

#[derive(Debug)]
struct Cpu {
    cycles_completed: usize,
    current_instruction: Instruction,
    x: isize,
    instruction_buffer: VecDeque<Instruction>,
    instruction_cycles_remaining: usize,
}

impl Cpu {
    pub fn new(instructions: VecDeque<Instruction>) -> Self {
        let mut s = Self {
            cycles_completed: 0,
            instruction_cycles_remaining: Default::default(),
            x: 1,
            instruction_buffer: instructions,
            current_instruction: Instruction::NoOp,
        };
        s.pop_instruction();
        s
    }

    pub fn cycle(&mut self) {
        if self.instruction_cycles_remaining > 1 {
            self.instruction_cycles_remaining -= 1;
        } else {
            match self.current_instruction {
                Instruction::AddX(n) => self.x += n,
                Instruction::NoOp => (),
            }
            self.pop_instruction()
        }
        self.cycles_completed += 1;
    }

    fn pop_instruction(&mut self) {
        self.current_instruction = self
            .instruction_buffer
            .pop_front()
            .unwrap_or(Instruction::NoOp);
        match self.current_instruction {
            Instruction::AddX(_) => self.instruction_cycles_remaining = 2,
            Instruction::NoOp => self.instruction_cycles_remaining = 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AddX(isize),
    NoOp,
}

fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .map(|mut splits| match splits.next().unwrap() {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(splits.next().unwrap().parse().unwrap()),
            other => panic!("Unknown instruction {other:?}"),
        })
        .collect()
}

fn sample_interesting_cycles(input: &str) -> Vec<isize> {
    let mut samples = Vec::with_capacity(6);
    let instructions = parse_instructions(input);
    let mut cpu = Cpu::new(instructions);
    while !cpu.instruction_buffer.is_empty() {
        cpu.cycle();
        let during_cycle = isize::try_from(cpu.cycles_completed).unwrap() + 1;
        if during_cycle % 40 == 20 {
            let signal_strength = cpu.x * during_cycle;
            // dbg!(during_cycle, &cpu, signal_strength);
            samples.push(signal_strength);
        }
    }
    samples
}

pub fn part1(input: &str) -> isize {
    let samples = sample_interesting_cycles(input);
    samples.iter().sum()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let instructions = parse_instructions(input);
    let mut cpu = Cpu::new(instructions);
    let mut output = String::with_capacity((WIDTH * HEIGHT) + HEIGHT); // newlines
    for _ in 0..WIDTH * HEIGHT {
        let during_cycle = cpu.cycles_completed + 1;
        let writing_horizontal_pixel: usize = cpu.cycles_completed % 40;
        let sprite_left = cpu.x - 1;
        let sprite_right = cpu.x + 1;
        // dbg!(sprite_left, sprite_right, writing_horizontal_pixel);
        if (sprite_left..=sprite_right).contains(&(writing_horizontal_pixel as isize)) {
            output.push('#');
        } else {
            output.push('.');
        }
        if writing_horizontal_pixel % WIDTH == WIDTH - 1 {
            output.push('\n');
        }
        cpu.cycle();
    }
    output.pop();
    output
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day10.txt").unwrap();
    dbg!(part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"noop
addx 3
addx -5";

    #[test]
    pub fn test_cpu() {
        let instructions = parse_instructions(EXAMPLE);
        let mut cpu = Cpu::new(instructions);

        dbg!(&cpu);
        assert_eq!(0, cpu.cycles_completed);
        assert_eq!(1, cpu.x);
        cpu.cycle();
        dbg!(&cpu);
        assert_eq!(1, cpu.cycles_completed);
        assert_eq!(1, cpu.x);
        cpu.cycle();
        dbg!(&cpu);
        assert_eq!(2, cpu.cycles_completed);
        assert_eq!(1, cpu.x);
        cpu.cycle();
        dbg!(&cpu);
        assert_eq!(3, cpu.cycles_completed);
        assert_eq!(4, cpu.x);
        cpu.cycle();
        dbg!(&cpu);
        assert_eq!(4, cpu.cycles_completed);
        assert_eq!(4, cpu.x);
        cpu.cycle();
        dbg!(&cpu);
        assert_eq!(5, cpu.cycles_completed);
        assert_eq!(-1, cpu.x);
        cpu.cycle();
    }

    #[test]
    pub fn part1_example() {
        let input = std::fs::read_to_string("input/2022/day10_example1.txt").unwrap();

        let samples = sample_interesting_cycles(&input);
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], samples);

        assert_eq!(13140, part1(&input));
    }

    const PART2_EXPECTED: &'static str = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    pub fn part2_example() {
        let input = std::fs::read_to_string("input/2022/day10_example1.txt").unwrap();

        for (i, (e, a)) in PART2_EXPECTED
            .lines()
            .zip(part2(&input).lines())
            .enumerate()
        {
            assert_eq!(e, a, "line {i}");
        }
        assert_eq!(PART2_EXPECTED, part2(&input));
    }
}
