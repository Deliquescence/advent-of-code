use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const SAND_FROM: (usize, usize) = (500, 0);
struct Grid<const WIDTH: usize, const HEIGHT: usize> {
    cells: Vec<bool>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for Grid<WIDTH, HEIGHT> {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.1 * WIDTH + index.0]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)> for Grid<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.1 * WIDTH + index.0]
    }
}

static ROCK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?-u)(\d+),(\d+)").unwrap());

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Grid<WIDTH, HEIGHT> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::<WIDTH, HEIGHT> {
            cells: vec![false; WIDTH * HEIGHT],
        };
        let coords = s
            .lines()
            .flat_map(|l| ROCK_REGEX.captures_iter(l))
            .filter_map(|c| {
                c[1].parse()
                    .and_then(|x: usize| c[2].parse().map(|y: usize| (x, y)))
                    .ok()
            })
            .tuple_windows()
            .flat_map(|(l, r)| enumerate_line(l, r));

        for c in coords {
            grid[c] = true;
        }
        Ok(grid)
    }
}

fn enumerate_line(
    from: (usize, usize),
    to: (usize, usize),
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    use std::cmp::{max, min};
    use std::iter::repeat;
    if from.0 == to.0 {
        Box::new(repeat(from.0).zip(min(from.1, to.1)..=max(from.1, to.1)))
    } else if from.1 == to.1 {
        Box::new((min(from.0, to.0)..=max(from.0, to.0)).zip(repeat(from.1)))
    } else {
        Box::new(std::iter::empty())
    }
}

pub fn part1(input: &str) -> usize {
    todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day14.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    pub fn parsing() {
        let grid: Grid<1000, 200> = EXAMPLE.parse().unwrap();
        assert!(grid[(498, 4)]);
        assert!(grid[(498, 5)]);
        assert!(grid[(498, 6)]);
        assert!(grid[(497, 6)]);
        assert!(grid[(496, 6)]);
        assert!(grid[(503, 4)]);
        assert!(grid[(502, 4)]);
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(24, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
