use std::{ops::Index, str::FromStr};

pub fn main() {
    let input = std::fs::read_to_string("input/2025/day4.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    grid.indices()
        .filter(|idx| grid[*idx] && grid.neighbors(idx.0, idx.1).filter(|b| *b).count() < 4)
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

struct Grid {
    cells: Vec<bool>,
    width: usize,
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.1 * self.width + index.0]
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split_ascii_whitespace().peekable();
        let width = lines.peek().ok_or(())?.len();
        let cells = lines.flat_map(|l| l.chars()).map(|c| c == '@').collect();
        Ok(Self { cells, width })
    }
}

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Grid {
    pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width).flat_map(|i| std::iter::repeat(i).zip(0..self.width))
    }

    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = bool> {
        NEIGHBOR_OFFSETS.iter().filter_map(move |(o, p)| {
            let ni = i.checked_add_signed(*o)?;
            let nj = j.checked_add_signed(*p)?;
            if ni < self.width && nj < self.width {
                Some(self[(ni, nj)])
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
