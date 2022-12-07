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

pub fn parse_strategy(input: &str) -> Vec<Stratagem> {
    input
        .lines()
        .map(|l| l.parse().expect("line should be in expected format"))
        .collect()
}

pub fn calculate_score(input: &[Stratagem]) -> u64 {
    input.iter().fold(0, |acc, s| {
        acc + s.response as u64 + s.response.compare(&s.opponent) as u64
    })
}

pub fn part1(input: &str) -> u64 {
    calculate_score(&parse_strategy(input))
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day2.txt").unwrap();
    let score = part1(&input);
    println!("{score}");
}

impl FromStr for Stratagem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let opponent = parts.next().ok_or(())?.parse()?;
        let response = parts.next().ok_or(())?.parse()?;
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
    pub fn example() {
        let input = r"A Y
B X
C Z";
        assert_eq!(15, part1(&input));
    }

    #[test]
    pub fn into() {
        let rock = Shape::Rock;
        let int: usize = rock as usize;
        assert_eq!(1, int);
    }
}
