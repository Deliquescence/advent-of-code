use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone)]
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
                if let Ok(v) = Value::parse(&str_bytes[left..i_closing]) {
                    values.push(v);
                }
                left = right + 1;
            }
            Ok(Value::List(values))
        } else {
            atoi::atoi(str_bytes).ok_or(()).map(Value::Integer)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(u) => write!(f, "{u}"),
            Value::List(values) => {
                write!(f, "[")?;
                for v in values.iter().take(values.len().saturating_sub(1)) {
                    write!(f, "{v},")?;
                }
                if let Some(v) = values.last() {
                    write!(f, "{v}")?;
                }
                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(s), Value::Integer(o)) => s.cmp(o),
            (Value::List(s), Value::List(o)) => {
                use itertools::EitherOrBoth::*;
                s.iter()
                    .zip_longest(o.iter())
                    .map(|eob| match eob {
                        Both(l, r) => l.cmp(r),
                        Left(_) => std::cmp::Ordering::Greater,
                        Right(_) => std::cmp::Ordering::Less,
                    })
                    .find(|&ord| ord != std::cmp::Ordering::Equal)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
            (Value::Integer(s), Value::List(_)) => Value::List(vec![Value::Integer(*s)]).cmp(other),
            (Value::List(_), Value::Integer(o)) => self.cmp(&Value::List(vec![Value::Integer(*o)])),
        }
    }
}

fn parse_lines(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter_map(|l| Value::parse(l.as_bytes()).ok())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let lines = parse_lines(input);
    // dbg!(&lines);
    let correct = lines
        .iter()
        .tuples()
        .enumerate()
        .filter_map(|(i, (left, right))| if left < right { Some(i + 1) } else { None })
        .collect_vec();
    // dbg!(&correct);
    correct.iter().sum()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    let additional_packets = [
        Value::parse(b"[[2]]").unwrap(),
        Value::parse(b"[[6]]").unwrap(),
    ];
    let mut lines = parse_lines(input);
    lines.extend(additional_packets.iter().cloned());
    lines.sort();
    lines.iter().enumerate().fold(1, |a, (i, v)| {
        if additional_packets.contains(v) {
            (i + 1) * a
        } else {
            a
        }
    })
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day13.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"[1,1,3,1,1]
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
        let lines = parse_lines(EXAMPLE);

        assert_eq!(
            Value::parse(b"[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
            lines[14]
        );
        assert_eq!(
            Value::parse(b"[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap(),
            lines[15]
        );
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(140, part2(EXAMPLE));
    }
}
