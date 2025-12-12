use std::{
    fmt::Display,
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

impl<const WIDTH: usize, const HEIGHT: usize> Display for Grid<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INTERESTING_WIDTH: usize = 500;
        const INTERESTING_HEIGHT: usize = 175;
        for y in 1..INTERESTING_HEIGHT {
            for x in (SAND_FROM.0 - INTERESTING_WIDTH + 1)..(SAND_FROM.0 + INTERESTING_WIDTH) {
                if self[(x, y)] {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

static ROCK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?-u)(\d+),(\d+)").unwrap());

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Grid<WIDTH, HEIGHT> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        let mut grid = Grid::<WIDTH, HEIGHT> {
            cells: vec![false; WIDTH * HEIGHT],
        };
        let coords = s.lines().flat_map(|l| {
            ROCK_REGEX
                .captures_iter(l)
                .filter_map(|c| {
                    c[1].parse()
                        .and_then(|x: usize| c[2].parse().map(|y: usize| (x, y)))
                        .ok()
                })
                .tuple_windows()
                .flat_map(|(l, r)| enumerate_line(l, r))
        });

        for c in coords {
            grid[c] = true;
        }
        Ok(grid)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid<WIDTH, HEIGHT> {
    pub fn create_floor(&mut self) {
        let y_height = (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .filter_map(|c| if self[c] { Some(c.1) } else { None })
            .max()
            .expect("non-empty")
            + 2;

        for x in 0..WIDTH {
            self[(x, y_height)] = true;
        }
    }

    pub fn place_sand(&mut self) -> bool {
        let mut c = SAND_FROM;
        loop {
            if c.1 >= HEIGHT - 1 || c.0 == 0 || c.0 >= WIDTH - 1 {
                break false;
            }
            if !self[(c.0, c.1 + 1)] {
                c.1 += 1;
            } else if !self[(c.0 - 1, c.1 + 1)] {
                c.0 -= 1;
                c.1 += 1;
            } else if !self[(c.0 + 1, c.1 + 1)] {
                c.0 += 1;
                c.1 += 1;
            } else {
                self[c] = true;
                break true;
            }
        }
    }

    pub fn place_until_settled(&mut self, visualize: bool) -> usize {
        if visualize {
            let mut c = 0;
            loop {
                if self.place_sand() {
                    c += 1;
                } else {
                    break;
                }
                print!("\x1B[2J\x1B[1;1H");
                print!("{self}");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            c
        } else {
            std::iter::repeat_with(|| self.place_sand())
                .take_while(|b| *b)
                .count()
        }
    }

    pub fn place_until_full(&mut self, _visualize: bool) -> usize {
        let mut c = 0;
        while !self[SAND_FROM] {
            assert!(self.place_sand());
            c += 1;
        }
        c
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid: Grid<1000, 200> = input.parse().unwrap();
    grid.place_until_settled(false)
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    let mut grid: Grid<1000, 200> = input.parse().unwrap();
    grid.create_floor();
    grid.place_until_full(false)
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day14.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"498,4 -> 498,6 -> 496,6
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
    pub fn placing_sand() {
        let mut grid: Grid<1000, 200> = EXAMPLE.parse().unwrap();
        assert!(!grid[(500, 8)]);
        assert!(grid.place_sand());
        assert!(grid[(500, 8)]);
        assert!(grid.place_sand());
        assert!(grid[(499, 8)]);
        assert!(grid.place_sand());
        assert!(grid[(501, 8)]);
        assert!(grid.place_sand());
        assert!(grid[(500, 7)]);
        assert!(grid.place_sand());
        assert!(grid[(498, 8)]);
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(24, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(93, part2(EXAMPLE));
    }
}
