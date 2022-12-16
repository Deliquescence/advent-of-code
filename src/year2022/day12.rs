use itertools::Itertools;
use tinyvec::ArrayVec;

#[derive(Debug)]
struct Graph {
    vertices: Vec<u8>,
    incoming: Vec<ArrayVec<[usize; 4]>>,
    outgoing: Vec<ArrayVec<[usize; 4]>>,
    start: usize,
    peak: usize,
}

impl Graph {
    fn dijkstra(&self) -> (Vec<u32>, Vec<Option<usize>>) {
        let mut unvisited = (0..self.vertices.len()).collect_vec();
        let mut distance: Vec<u32> = vec![u32::MAX; self.vertices.len()];
        let mut previous: Vec<Option<usize>> = vec![None; self.vertices.len()];
        distance[self.peak] = 0;

        while let Some((i, &u)) = unvisited
            .iter()
            .enumerate()
            .min_by_key(|(_i, v)| distance[**v])
        {
            unvisited.remove(i);

            for &n in self.incoming[u].iter().filter(|a| unvisited.contains(a)) {
                if distance[u].saturating_add(1) < distance[n] {
                    distance[n] = distance[u] + 1;
                    previous[n] = Some(u);
                }
            }
        }

        (distance, previous)
    }
}

fn dijkstra_path(previous: &[Option<usize>], from: usize) -> Vec<usize> {
    let mut path = Vec::new();
    path.push(from);
    let mut v = from;
    while let Some(p) = previous[v] {
        path.push(p);
        v = p;
    }
    path.reverse();
    path
}

fn parse_grid(input: &str) -> Graph {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().into()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let idx = |i, j| (i * width) + j;

    let mut start = 0;
    let mut peak = 0;
    let mut vertices = vec![0; width * height];
    let mut incoming: Vec<ArrayVec<[usize; 4]>> = vec![Default::default(); width * height];
    let mut outgoing: Vec<ArrayVec<[usize; 4]>> = vec![Default::default(); width * height];
    for i in 0..height {
        for j in 0..width {
            let v = idx(i, j);
            vertices[v] = match grid[i][j] {
                b'S' => {
                    start = v;
                    b'a'
                }
                b'E' => {
                    peak = v;
                    b'z'
                }
                b => b,
            };
        }
    }
    for i in 0..height {
        for j in 0..width {
            let v = idx(i, j);
            let mut maybe_push = |ii, jj| {
                let u = idx(ii, jj);
                if accessible(vertices[v], vertices[u]) {
                    outgoing[v].push(u);
                    incoming[u].push(v);
                }
            };
            if i > 0 {
                maybe_push(i - 1, j);
            }
            if j > 0 {
                maybe_push(i, j - 1);
            }
            if i < height - 1 {
                maybe_push(i + 1, j);
            }
            if j < width - 1 {
                maybe_push(i, j + 1);
            }
        }
    }

    Graph {
        vertices,
        incoming,
        outgoing,
        start,
        peak,
    }
}

fn accessible(src: u8, dest: u8) -> bool {
    dest <= src + 1
}

pub fn part1(input: &str) -> usize {
    let graph = parse_grid(input);
    // dbg!(&graph);
    let path = dijkstra_path(&graph.dijkstra().1, graph.start);
    // dbg!(&path);
    path.len() - 1
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    let graph = parse_grid(input);
    let (distance, previous) = graph.dijkstra();
    let closest = graph
        .vertices
        .iter()
        .enumerate()
        .filter_map(|(i, &h)| if h == b'a' { Some(i) } else { None })
        .min_by_key(|&i| distance[i])
        .unwrap();
    let path = dijkstra_path(&previous, closest);
    // dbg!(&path);
    path.len() - 1
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day12.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
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

    #[test]
    pub fn part2_example() {
        assert_eq!(29, part2(EXAMPLE));
    }
}
