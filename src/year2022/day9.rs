use std::str::FromStr;

type Point = (isize, isize);

struct Simulation {
    head: Point,
    tail: Point,
    head_history: Vec<Point>,
    tail_history: Vec<Point>,
}

impl Default for Simulation {
    fn default() -> Self {
        let mut s = Self {
            head: Default::default(),
            tail: Default::default(),
            head_history: Default::default(),
            tail_history: Default::default(),
        };
        s.push_history();
        s
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Simulation {
    pub fn step_all(&mut self, steps: &[(Direction, usize)]) {
        for (dir, count) in steps {
            for _ in 0..*count {
                self.step_once(*dir);
            }
        }
    }

    pub fn step_once(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.head = (self.head.0, self.head.1 + 1),
            Direction::Down => self.head = (self.head.0, self.head.1 - 1),
            Direction::Left => self.head = (self.head.0 - 1, self.head.1),
            Direction::Right => self.head = (self.head.0 + 1, self.head.1),
        }

        let offset = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        match offset {
            (x, y) if x.abs() <= 1 && y.abs() <= 1 => (),
            // (x, y) if x == 0 => self.tail.1 += y.signum(),
            // (x, y) if y == 0 => self.tail.0 += x.signum(),
            (x, y) => {
                self.tail.0 += x.signum();
                self.tail.1 += y.signum();
            }
        }

        self.push_history()
    }

    fn push_history(&mut self) {
        self.head_history.push(self.head);
        self.tail_history.push(self.tail);
    }
}

fn run_simulation(input: &str) -> Simulation {
    let steps = parse_steps(input);
    let mut simulation: Simulation = Default::default();
    simulation.step_all(&steps);

    simulation
}

fn parse_steps(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_ascii_whitespace())
        .map(|mut iter| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let simulation = run_simulation(input);
    let mut sorted = simulation.tail_history.clone();
    sorted.sort();
    sorted.dedup();
    sorted.len()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day9.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	todo!();
    // }
}
