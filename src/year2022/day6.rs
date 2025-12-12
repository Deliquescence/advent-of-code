use hashbag::HashBag;

pub fn find_distinct(input: &str, window_size: usize) -> usize {
    let data = input.trim().as_bytes();
    let mut window = HashBag::with_capacity(window_size);
    for d in data.iter().take(window_size){
        window.insert(*d);
    }
    for i in window_size..data.len() {
        if window.set_len() == window_size {
            return i;
        }
        window.remove(&data[i - window_size]);
        window.insert(data[i]);
    }

    panic!("nothing found");
}

pub fn part1(input: &str) -> usize {
    find_distinct(input, 4)
}

pub fn part2(input: &str) -> usize {
    find_distinct(input, 14)
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day6.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    pub fn part1_examples() {
        assert_eq!(7, part1(EXAMPLE1));
        assert_eq!(5, part1(EXAMPLE2));
        assert_eq!(6, part1(EXAMPLE3));
        assert_eq!(10, part1(EXAMPLE4));
        assert_eq!(11, part1(EXAMPLE5));
    }

    #[test]
    pub fn part2_examples() {
        assert_eq!(19, part2(EXAMPLE1));
        assert_eq!(23, part2(EXAMPLE2));
        assert_eq!(23, part2(EXAMPLE3));
        assert_eq!(29, part2(EXAMPLE4));
        assert_eq!(26, part2(EXAMPLE5));
    }
}
