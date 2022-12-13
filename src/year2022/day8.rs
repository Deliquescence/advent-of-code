type Tree = (u8, bool);
type Grid = Vec<Vec<Tree>>;

fn parse_unmarked_grid(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("input is digits")
                        .try_into()
                        .expect("max height is 9")
                })
                .map(|d| (d, false))
                .collect()
        })
        .collect()
}

fn mark_grid_visibility(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();

    // visible from left
    for i in 0..height {
        assert_eq!(width, grid[i].len());
        for j in 0..width {
            if grid[i][..j].iter().all(|t| t.0 < grid[i][j].0) {
                grid[i][j].1 = true;
            }
        }
    }

    // visible from top
    for j in 0..width {
        for i in 0..height {
            if grid[..i].iter().map(|r| r[j]).all(|t| t.0 < grid[i][j].0) {
                grid[i][j].1 = true;
            }
        }
    }

    // visible from right
    for i in 0..height {
        for j in (0..width).rev() {
            if grid[i][j..].iter().skip(1).all(|t| t.0 < grid[i][j].0) {
                grid[i][j].1 = true;
            }
        }
    }

    // visible from bottom
    for j in 0..width {
        for i in (0..height).rev() {
            if grid[i..]
                .iter()
                .skip(1)
                .map(|r| r[j])
                .all(|t| t.0 < grid[i][j].0)
            {
                grid[i][j].1 = true;
            }
        }
    }
}

pub fn parse_grid(input: &str) -> Grid {
    let mut grid = parse_unmarked_grid(input);
    mark_grid_visibility(&mut grid);
    grid
}

pub fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    grid.iter().flat_map(|r| r.iter().filter(|t| t.1)).count()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    (0..grid.len())
        .flat_map(|i| (0..grid.len()).map(move |j| (i, j)))
        .map(|(i, j)| scenic_score(&grid, i, j))
        .max()
        .unwrap()
}

fn scenic_score(grid: &Grid, i: usize, j: usize) -> usize {
    let height = grid[i][j].0;
    let to_left = count_trees(height, grid[i][..j].iter().rev().copied());
    let to_up = count_trees(height, grid[..i].iter().rev().map(|r| r[j]));
    let to_right = count_trees(height, grid[i].iter().skip(j + 1).copied());
    let to_down = count_trees(height, grid.iter().skip(i + 1).map(|r| r[j]));

    to_up * to_left * to_down * to_right
}

fn count_trees(height: u8, mut trees: impl Iterator<Item = Tree>) -> usize {
    match trees.try_fold(0, |acc, t| {
        if t.0 < height {
            Ok(acc + 1)
        } else {
            Err(acc + 1)
        }
    }) {
        Ok(c) => c,
        Err(c) => c,
    }
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day8.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
30373
25512
65332
33549
35390";

    #[test]
    pub fn part1_example() {
        assert_eq!(21, part1(EXAMPLE));
    }

    #[test]
    pub fn my_part1() {
        assert_eq!(
            1736,
            part1(&std::fs::read_to_string("input/2022/day8.txt").unwrap())
        );
    }

    #[test]
    pub fn test_scenic_score() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(4, scenic_score(&grid, 1, 2));
        assert_eq!(8, scenic_score(&grid, 3, 2));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(8, part2(EXAMPLE));
    }
}
