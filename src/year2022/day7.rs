pub enum Tree {
    File(File),
    Directory(Directory),
}

pub struct File {
    name: String,
    size: usize,
}

pub struct Directory {
    name: String,
    children: Vec<Tree>,
}

impl Tree {
    pub fn name(&self) -> &str {
        match self {
            Tree::File(f) => &f.name,
            Tree::Directory(d) => &d.name,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Tree::File(f) => f.size,
            Tree::Directory(d) => d.size(),
        }
    }
}

impl Directory {
    pub fn size(&self) -> usize {
        self.children.iter().map(|c| c.size()).sum()
    }
}

pub fn parse_tree(input: &str) -> Tree {
    todo!();
}

pub fn part1(input: &str) -> usize {
    todo!();
}

pub fn part2(input: &str) -> usize {
    todo!();
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day7.txt").unwrap();
    dbg!(part1(&input));
    // dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    pub fn parse_tree_example() {
        let tree = parse_tree(EXAMPLE);

        let root = match tree {
            Tree::Directory(d) => d,
            _ => panic!("root should be directory"),
        };
        assert_eq!("/", root.name);
        assert_eq!(48381165, root.size());
        assert_children(&root, vec!["a", "b.txt", "c.dat", "d"]);
        assert_children_sizes(&root, vec![94853, 14848514, 8504156, 24933642]);

        match &root.children[0] {
            Tree::File(_) => panic!("expected 'a' to be directory"),
            Tree::Directory(dir_a) => {
                assert_eq!("a", dir_a.name);
                assert_children(&dir_a, vec!["e", "f", "g", "h.lst"]);
                assert_children_sizes(&dir_a, vec![584, 29116, 2557, 62596]);

                match &dir_a.children[0] {
                    Tree::File(_) => panic!("expected 'e' to be directory"),
                    Tree::Directory(dir_e) => {
                        assert_eq!("e", dir_e.name);
                        assert_eq!(584, dir_e.size());
                    }
                }
            }
        }
    }

    fn assert_children(directory: &Directory, expected: Vec<&'static str>) {
        assert_eq!(
            expected,
            directory
                .children
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
        );
    }

    fn assert_children_sizes(directory: &Directory, expected: Vec<usize>) {
        assert_eq!(
            expected,
            directory
                .children
                .iter()
                .map(|c| c.size())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(95437, part1(EXAMPLE));
    }

    // #[test]
    // pub fn part2_example() {
    // 	todo!();
    // }
}
