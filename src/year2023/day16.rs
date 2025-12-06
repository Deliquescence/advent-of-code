type Tile = u8;
type Grid = Box<[Box<[Tile]>]>;

fn parse_grid(str: &str) -> Grid {
    str.split_ascii_whitespace()
        .map(|r| r.chars().map(|c| c as u8).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        use Direction as D;
        match self {
            D::Right => (0, 1),
            D::Down => (1, 0),
            D::Left => (0, -1),
            D::Up => (-1, 0),
        }
    }
}

fn next_directions(going: Direction, to: Tile) -> Box<[Direction]> {
    use Direction as D;
    match to {
        b'|' if going == D::Right || going == D::Left => Box::new([D::Up, D::Down]),
        b'-' if going == D::Up || going == D::Down => Box::new([D::Right, D::Left]),
        b'/' if going == D::Right => Box::new([D::Up]),
        b'/' if going == D::Left => Box::new([D::Down]),
        b'/' if going == D::Up => Box::new([D::Right]),
        b'/' if going == D::Down => Box::new([D::Left]),
        b'\\' if going == D::Right => Box::new([D::Down]),
        b'\\' if going == D::Left => Box::new([D::Up]),
        b'\\' if going == D::Up => Box::new([D::Left]),
        b'\\' if going == D::Down => Box::new([D::Right]),
        _ => Box::new([going]),
    }
}

fn next_coords(at: (usize, usize), going: Direction, n: usize) -> Option<(usize, usize)> {
    let offset = going.as_offset();
    let i = at.0.checked_add_signed(offset.0)?;
    let j = at.1.checked_add_signed(offset.1)?;
    if i < n && j < n {
        Some((i, j))
    } else {
        None
    }
}

fn mark(
    at: (usize, usize),
    going: Direction,
    grid: &Grid,
    energized: &mut Box<[Box<[Vec<Direction>]>]>,
) {
    println!("{:?} hit by going {:?}", at, going);

    if !energized[at.0][at.1].contains(&going) {
        energized[at.0][at.1].push(going);

        for next_dir in next_directions(going, grid[at.0][at.1]).iter() {
            if let Some(next) = next_coords(at, *next_dir, grid.len()) {
                mark(next, *next_dir, grid, energized);
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut energized: Box<[Box<[Vec<Direction>]>]> =
        vec![vec![Vec::with_capacity(4); grid.len()].into(); grid.len()].into();
    mark((0, 0), Direction::Right, &grid, &mut energized);

    energized
        .iter()
        .flat_map(|r| r.iter())
        .filter(|m| !m.is_empty())
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2023/day16.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    pub fn part1_example() {
        assert_eq!(46, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
