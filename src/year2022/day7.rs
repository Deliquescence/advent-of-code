use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone, PartialEq)]
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

impl File {
    pub fn new(name: impl Into<String>, size: usize) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

impl Directory {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Default::default(),
        }
    }

    pub fn size(&self) -> usize {
        self.children.iter().map(|c| c.size()).sum()
    }

    pub fn descendants_and_self<'a>(&'a self) -> impl Iterator<Item = &'a Self> + 'a {
        std::iter::once(self).chain(self.descendant_directories())
    }

    pub fn descendant_directories<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
        Box::new(
            self.children
                .iter()
                .filter_map(|c| {
                    if let Tree::Directory(d) = c {
                        Some(d.descendants_and_self())
                    } else {
                        None
                    }
                })
                .flatten(),
        )
    }
}

pub fn parse_tree(input: &str) -> Directory {
    let mut unparsed_lines: VecDeque<_> = input.lines().skip_while(|l| l.is_empty()).collect();
    parse_cd(&mut unparsed_lines)
}

pub fn parse_cd(unparsed_lines: &mut VecDeque<&str>) -> Directory {
    const PROMPT: &str = "$ cd ";
    let parent_line = unparsed_lines.pop_front().expect("more to do");
    assert!(parent_line.starts_with(PROMPT));
    let name = parent_line[PROMPT.len()..].to_string();
    let mut dir = Directory::new(name);
    dir.children = parse_ls(unparsed_lines);

    while let Some(child_line) = unparsed_lines.front() {
        assert!(child_line.starts_with(PROMPT));
        let child_name = child_line[PROMPT.len()..].to_string();
        if child_name == ".." {
            break;
        }
        let child = dir
            .children
            .iter_mut()
            .find(|c| matches!(c, Tree::Directory(d) if d.name == child_name))
            .expect("cd into dir from ls");
        let child_dir = parse_cd(unparsed_lines);
        unparsed_lines.pop_front();
        *child = Tree::Directory(child_dir);
    }

    dir
}

pub fn parse_ls(unparsed_lines: &mut VecDeque<&str>) -> Vec<Tree> {
    assert_eq!("$ ls", unparsed_lines[0]);
    let items: Vec<_> = unparsed_lines
        .iter()
        .skip(1)
        .take_while(|l| !l.starts_with("$"))
        .map(|l| {
            let (dir_or_size, name) = l.split_once(' ').expect("format of ls");
            match dir_or_size.parse() {
                Ok(size) => Tree::File(File::new(name, size)),
                Err(_) => Tree::Directory(Directory::new(name)),
            }
        })
        .collect();
    for _ in 0..=items.len() {
        unparsed_lines.pop_front();
    }
    items
}

pub fn part1(input: &str) -> usize {
    let tree = parse_tree(input);

    tree.descendants_and_self()
        .map(|d| d.size())
        .filter(|&s| s <= 100000)
        .sum()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const FREE_SPACE_REQUIRED: usize = 30000000;
    let tree = parse_tree(input);
    let total_used = tree.size();
    let total_unused = TOTAL_SPACE - total_used;
    let need_to_free = FREE_SPACE_REQUIRED - total_unused;

    tree.descendants_and_self()
        .map(|d| d.size())
        .filter(|&s| s >= need_to_free)
        .min()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/2022/day7.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"
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
        let root = parse_tree(EXAMPLE);

        assert_eq!("/", root.name);
        assert_eq!(48381165, root.size());
        assert_children_names(&root, vec!["a", "b.txt", "c.dat", "d"]);
        assert_children_sizes(&root, vec![94853, 14848514, 8504156, 24933642]);

        match &root.children[0] {
            Tree::File(_) => panic!("expected 'a' to be directory"),
            Tree::Directory(dir_a) => {
                assert_eq!("a", dir_a.name);
                assert_children_names(dir_a, vec!["e", "f", "g", "h.lst"]);
                assert_children_sizes(dir_a, vec![584, 29116, 2557, 62596]);

                match &dir_a.children[0] {
                    Tree::File(_) => panic!("expected 'e' to be directory"),
                    Tree::Directory(dir_e) => {
                        assert_eq!("e", dir_e.name);
                        assert_eq!(584, dir_e.size());

                        assert_children_names(dir_e, vec!["i"]);
                        assert_children_sizes(dir_e, vec![584]);
                    }
                }
            }
        }

        match &root.children[3] {
            Tree::File(_) => panic!("expected 'd' to be directory"),
            Tree::Directory(dir_d) => {
                assert_eq!("d", dir_d.name);
                assert_children_names(dir_d, vec!["j", "d.log", "d.ext", "k"]);
                assert_children_sizes(dir_d, vec![4060174, 8033020, 5626152, 7214296]);
            }
        }
    }

    fn assert_children_names(directory: &Directory, expected: Vec<&str>) {
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

    #[test]
    pub fn test_descendants() {
        let mut dir = Directory::new("/");
        assert_eq!(vec![&dir], dir.descendants_and_self().collect::<Vec<_>>());
        assert_eq!(
            Vec::<&Directory>::new(),
            dir.descendant_directories().collect::<Vec<_>>()
        );

        let mut a = Directory::new("a");
        let b = Directory::new("b");
        a.children.push(Tree::Directory(b.clone()));
        assert_eq!(vec![&b], a.descendant_directories().collect::<Vec<_>>());
        assert_eq!(vec![&a, &b], a.descendants_and_self().collect::<Vec<_>>());

        dir.children.push(Tree::Directory(a.clone()));
        assert_eq!(
            vec![&dir, &a, &b],
            dir.descendants_and_self().collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(24933642, part2(EXAMPLE));
    }
}
