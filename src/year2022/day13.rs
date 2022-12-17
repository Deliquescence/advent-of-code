use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use itertools::Itertools;

#[derive(Eq, PartialEq)]
enum Value {
    Integer(u32),
    List(Vec<Value>),
}

impl Value {
    fn parse(str_bytes: &[u8]) -> Result<Self, ()> {
        if str_bytes.iter().next() == Some(&b'[') {
            let mut unclosed = 0;
            let mut indices = VecDeque::new();
            let (i_closing, _) = str_bytes
                .iter()
                .enumerate()
                .find(|(i, c)| {
                    if **c == b'[' {
                        unclosed += 1;
                    } else if **c == b']' {
                        unclosed -= 1;
                    } else if **c == b',' && unclosed == 1 {
                        indices.push_back(*i);
                    }
                    unclosed == 0
                })
                .ok_or(())?;

            indices.push_back(i_closing);
            let mut values = Vec::new();
            let mut left = 1;
            while let Some(right) = indices.pop_front() {
                values.push(Value::parse(&str_bytes[left..right])?);
                left = right + 1;
            }
            Ok(Value::List(values))
        } else {
            atoi::atoi(str_bytes).ok_or(()).map(|u| Value::Integer(u))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(u) => write!(f, "{u}"),
            Value::List(values) => Ok({
                write!(f, "[")?;
                for v in values.iter().take(values.len().saturating_sub(1)) {
                    write!(f, "{v},")?;
                }
                if let Some(v) = values.last() {
                    write!(f, "{v}")?;
                }
                write!(f, "]")?;
            }),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

fn parse_pairs(input: &str) -> Vec<(Value, Value)> {
    input
        .lines()
        .filter_map(|l| Value::parse(l.as_bytes()).ok())
        .tuples()
        .collect()
}

pub fn part1(input: &str) -> usize {
    let packets = parse_pairs(input);
    dbg!(&packets);
    todo!();
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day13.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn parsing() {
        let pairs = parse_pairs(EXAMPLE);

        assert_eq!(
            Value::parse(b"[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
            pairs[7].0
        );
        assert_eq!(
            Value::parse(b"[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap(),
            pairs[7].1
        );
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
