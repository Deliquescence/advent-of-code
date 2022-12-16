use itertools::Itertools;
use tinyvec::ArrayVec;

#[derive(Debug)]
struct Graph {
    vertices: Vec<u8>,
    adjacencies: Vec<ArrayVec<[usize; 4]>>,
    start: usize,
    end: usize,
}

impl Graph {
    fn dijkstra(&self) -> Vec<usize> {
        let mut unvisited = (0..self.vertices.len()).collect_vec();
        let mut distance: Vec<u32> = vec![u32::MAX; self.vertices.len()];
        let mut previous: Vec<Option<usize>> = vec![None; self.vertices.len()];
        distance[self.start] = 0;

        while let Some((i, &u)) = unvisited
            .iter()
            .enumerate()
            .min_by_key(|(_i, v)| distance[**v])
        {
            unvisited.remove(i);
            if u == self.end {
                break;
            }

            for &n in self.adjacencies[u].iter().filter(|a| unvisited.contains(a)) {
                if distance[u] + 1 < distance[n] {
                    distance[n] = distance[u] + 1;
                    previous[n] = Some(u);
                }
            }
        }

        let mut path = Vec::new();
        path.push(self.end);
        let mut v = self.end;
        while let Some(p) = previous[v] {
            path.push(p);
            v = p;
        }
        path.reverse();
        path
    }
}

fn parse_grid(input: &str) -> Graph {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().into()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let idx = |ii, jj| (ii * width) + jj;

    let mut start = 0;
    let mut end = 0;
    let mut vertices = vec![0; width * height];
    let mut adjacencies: Vec<ArrayVec<[usize; 4]>> = vec![Default::default(); width * height];
    for i in 0..height {
        for j in 0..width {
            let v = idx(i, j);
            vertices[v] = match grid[i][j] {
                b'S' => {
                    start = v;
                    b'a'
                }
                b'E' => {
                    end = v;
                    b'z'
                }
                b => b,
            };
        }
    }
    for i in 0..height {
        for j in 0..width {
            let v = idx(i, j);
            let source = vertices[v];
            if i > 0 && accessible(source, vertices[idx(i - 1, j)]) {
                adjacencies[v].push(idx(i - 1, j));
            }
            if j > 0 && accessible(source, vertices[idx(i, j - 1)]) {
                adjacencies[v].push(idx(i, j - 1));
            }
            if i < height - 1 && accessible(source, vertices[idx(i + 1, j)]) {
                adjacencies[v].push(idx(i + 1, j));
            }
            if j < width - 1 && accessible(source, vertices[idx(i, j + 1)]) {
                adjacencies[v].push(idx(i, j + 1));
            }
        }
    }

    Graph {
        vertices,
        adjacencies,
        start,
        end,
    }
}

fn accessible(src: u8, dest: u8) -> bool {
    dest <= src + 1
}

pub fn part1(input: &str) -> usize {
    let graph = parse_grid(input);
    // dbg!(&graph);
    let path = graph.dijkstra();
    // dbg!(&path);
    path.len() - 1
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day12.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    pub fn part1_example() {
        assert_eq!(31, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
