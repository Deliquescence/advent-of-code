pub fn main() {
    let input = std::fs::read_to_string("input/2025/day1.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

pub fn part1(input: &str) -> usize {
    let turns = parse_turns(input);

    let mut pos = 50;
    let mut n_zeros = 0;
    for turn in turns {
        pos += turn;
        pos %= 100;

        if pos == 0 {
            n_zeros += 1;
        }
    }

    n_zeros
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> isize {
    let turns = parse_turns(input);

    let mut pos = 50;
    let mut n_zero_transitions = 0;
    for turn in turns {
        // dbg!(pos, turn);

        let orig = pos;
        n_zero_transitions += turn.abs() / 100;
        pos += turn % 100;

        if pos <= 0 {
            if orig > 0 {
                n_zero_transitions += 1;
            }
            pos = (pos + 100) % 100;
        } else if pos >= 100 {
            n_zero_transitions += 1;
            pos %= 100;
        }

        // dbg!(n_zero_transitions);
    }

    n_zero_transitions
}

fn parse_turns(input: &str) -> impl Iterator<Item = isize> + use<'_> {
    input.split_ascii_whitespace().map(|s| {
        let (d, amt) = s.split_at(1);
        match d {
            "L" => -amt.parse::<isize>().unwrap(),
            "R" => amt.parse().unwrap(),
            _ => panic!("unexpected prefix {d}"),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    pub fn part1_example() {
        assert_eq!(3, part1(EXAMPLE));
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(6, part2(EXAMPLE));
    }
}
