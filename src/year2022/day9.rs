use std::str::FromStr;

type Point = (isize, isize);

struct Simulation<const SNAKE_LEN: usize> {
    snake: [Point; SNAKE_LEN],
    tail_history: Vec<Point>,
}

impl<const L: usize> Default for Simulation<L> {
    fn default() -> Self {
        let mut s = Self {
            snake: [Default::default(); L],
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

impl<const L: usize> Simulation<L> {
    pub fn step_all(&mut self, steps: &[(Direction, usize)]) {
        for (dir, count) in steps {
            for _ in 0..*count {
                self.step_once(*dir);
            }
        }
    }

    pub fn step_once(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.snake[0].1 += 1,
            Direction::Down => self.snake[0].1 -= 1,
            Direction::Left => self.snake[0].0 -= 1,
            Direction::Right => self.snake[0].0 += 1,
        }

        for i in 1..L {
            let offset = (
                self.snake[i - 1].0 - self.snake[i].0,
                self.snake[i - 1].1 - self.snake[i].1,
            );
            match offset {
                (x, y) if x.abs() <= 1 && y.abs() <= 1 => (),
                (x, y) => {
                    self.snake[i].0 += x.signum();
                    self.snake[i].1 += y.signum();
                }
            }
        }

        self.push_history()
    }

    fn push_history(&mut self) {
        self.tail_history.push(self.snake[L - 1]);
    }
}

fn run_simulation<const L: usize>(input: &str) -> Simulation<L> {
    let steps = parse_steps(input);
    let mut simulation: Simulation<L> = Default::default();
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

fn distinct_tail_visits<const L: usize>(input: &str) -> usize {
    let simulation = run_simulation::<L>(input);
    // dbg!(&simulation.tail_history);
    let mut sorted = simulation.tail_history.clone();
    sorted.sort();
    sorted.dedup();
    sorted.len()
}

pub fn part1(input: &str) -> usize {
    distinct_tail_visits::<2>(input)
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    distinct_tail_visits::<10>(input)
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day9.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2: &'static str = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE1));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(1, part2(EXAMPLE1));
        assert_eq!(36, part2(EXAMPLE2));
    }
}
