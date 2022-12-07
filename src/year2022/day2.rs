use std::{
    cmp::{Eq, PartialEq},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Stratagem {
    pub opponent: Shape,
    pub response: Shape,
}

pub fn parse<F>(input: &str, from_str: F) -> Vec<Stratagem>
where
    F: Fn(&str) -> Result<Stratagem, ()>,
{
    input
        .lines()
        .map(|l| from_str(l).expect("line should be in expected format"))
        .collect()
}

pub fn calculate_score(input: &[Stratagem]) -> u64 {
    input.iter().fold(0, |acc, s| {
        acc + s.response as u64 + s.response.compare(&s.opponent) as u64
    })
}

pub fn part1(input: &str) -> u64 {
    calculate_score(&parse(input, Stratagem::from_str_part1))
}

pub fn part2(input: &str) -> u64 {
    calculate_score(&parse(input, Stratagem::from_str_part2))
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day2.txt").unwrap();
    let score = part2(&input);
    println!("{score}");
}

impl Stratagem {
    fn from_str_part1(s: &str) -> Result<Self, ()> {
        let mut parts = s.split_whitespace();
        let opponent = parts.next().ok_or(())?.parse()?;
        let response = parts.next().ok_or(())?.parse()?;
        Ok(Stratagem { opponent, response })
    }

    fn from_str_part2(s: &str) -> Result<Self, ()> {
        let mut parts = s.split_whitespace();
        let opponent = parts.next().ok_or(())?.parse()?;
        let outcome: Outcome = parts.next().ok_or(())?.parse()?;

        let response = match outcome {
            Outcome::Lose => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => opponent,
            Outcome::Win => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };

        Ok(Stratagem { opponent, response })
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl Shape {
    pub fn compare(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Lose,
            (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_part1() {
        let input = r"A Y
B X
C Z";
        assert_eq!(15, part1(&input));
    }

    #[test]
    pub fn example_part2() {
        let input = r"A Y
B X
C Z";
        assert_eq!(12, part2(&input));
    }

    #[test]
    pub fn into() {
        let rock = Shape::Rock;
        let int: usize = rock as usize;
        assert_eq!(1, int);
    }
}
