use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use itertools::Itertools;

enum Value {
    Integer(u32),
    List(Vec<Value>),
}

impl Value {
    fn parse(str_bytes: &[u8]) -> Result<Self, ()> {
        if str_bytes.iter().next() == Some(&b'[') {
            let mut unclosed = 0;
            let mut commas = VecDeque::new();
            if let Some((i_closing, _)) = str_bytes.iter().enumerate().find(|(i, c)| {
                if **c == b'[' {
                    unclosed += 1;
                    false
                } else if **c == b']' {
                    unclosed -= 1;
                    unclosed == 0
                } else if **c == b',' && unclosed == 1 {
                    commas.push_back(*i);
                    false
                } else {
                    false
                }
            }) {
                let mut values = Vec::new();
                let mut left = 1;
                while let Some(right) = commas.pop_front() {
                    if let Ok(v) = Value::parse(&str_bytes[left..right]) {
                        values.push(v);
                    }
                    left = right + 1;
                }
                if let Ok(v) = Value::parse(&str_bytes[left..i_closing]) {
                    values.push(v);
                }
                Ok(Value::List(values))
            } else {
                panic!("{:?}", String::from_utf8(str_bytes.to_vec()));
            }
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

pub fn part1(input: &str) -> usize {
    let packets: Vec<(Value, Value)> = input
        .lines()
        .filter_map(|l| Value::parse(l.as_bytes()).ok())
        .tuples()
        .collect();
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
    pub fn part1_example() {
        assert_eq!(13, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	assert_eq!(0, part2(EXAMPLE));
    // }
}
