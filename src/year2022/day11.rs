use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

type Operand = Option<u32>; // None == old

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operator: Operator,
    operand_left: Operand,
    operand_right: Operand,
    test_divisor: u32,
    true_monkey: usize,
    false_monkey: usize,
    num_inspects: usize,
}

impl Monkey {
    pub fn do_operation(&self, old: u32) -> u32 {
        let left = self.operand_left.unwrap_or(old);
        let right = self.operand_right.unwrap_or(old);

        match self.operator {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Multiply),
            "+" => Ok(Operator::Add),
            _ => Err(()),
        }
    }
}

static MONKEY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x-u)\s*
Monkey \s (?P<index>\d+):\s*
Starting \s items: \s (?P<items>[\d,\s]+)\s*
Operation: \s new \s = \s (?P<oper_left>old|\d+)\s(?P<operator>[\*\+])\s(?P<oper_right>old|\d+)\s*
Test: \s divisible \s by \s (?P<test_divisor>\d+)\s*
If \s true: \s throw \s to \s monkey \s (?P<true_monkey>\d+)\s*
If \s false: \s throw \s to \s monkey \s (?P<false_monkey>\d+)\s*
",
    )
    .unwrap()
});
static ITEM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?-u)(\d+)[, ]*").unwrap());

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    MONKEY_REGEX
        .captures_iter(input)
        .map(|c| {
            let items = ITEM_REGEX
                .captures_iter(&c["items"])
                .filter_map(|i| i[1].parse().ok())
                .collect();

            Monkey {
                items,
                operator: c["operator"].parse().unwrap(),
                operand_left: c["oper_left"].parse().ok(),
                operand_right: c["oper_right"].parse().ok(),
                test_divisor: c["test_divisor"].parse().unwrap(),
                true_monkey: c["true_monkey"].parse().unwrap(),
                false_monkey: c["false_monkey"].parse().unwrap(),
                num_inspects: Default::default(),
            }
        })
        .collect()
}

fn monkey_around(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        for item in std::mem::take(&mut monkeys[i].items) {
            monkeys[i].num_inspects += 1;
            let worry = monkeys[i].do_operation(item) / 3;
            let dest = if worry % monkeys[i].test_divisor == 0 {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };
            monkeys[dest].items.push_back(worry);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    for _ in 0..20 {
        monkey_around(&mut monkeys);
    }
    monkeys
        .iter()
        .map(|m| std::cmp::Reverse(m.num_inspects))
        .k_smallest(2)
        .fold(1, |a, x| a * x.0)
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day11.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    pub fn part1_example() {
        assert_eq!(10605, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	todo!();
    // }
}
