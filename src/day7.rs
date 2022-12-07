use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use once_cell::unsync::OnceCell;

type Input = INode;
type Output = usize;

// Metadata common between files and dirs
#[derive(Debug, Clone)]
struct INode {
    name: String,
    size_cell: OnceCell<usize>,
    inode_type: INodeType,
}

#[derive(Debug, Clone)]
enum INodeType {
    File,
    Dir(HashMap<String, INode>),
}

enum OutputLine {
    Cmd(Cmd),
    Ls(INode),
}

impl FromStr for OutputLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            Ok(OutputLine::Cmd(s.parse()?))
        } else {
            Ok(OutputLine::Ls(s.parse()?))
        }
    }
}

impl FromStr for INode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, name) = s.split_once(' ').context("No space")?;
        if first == "dir" {
            Ok(INode {
                name: name.to_string(),
                size_cell: OnceCell::new(),
                inode_type: INodeType::Dir(HashMap::new()),
            })
        } else {
            Ok(INode {
                name: name.to_string(),
                size_cell: OnceCell::with_value(first.parse()?),
                inode_type: INodeType::File,
            })
        }
    }
}

impl Display for INode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} ", self.name)?;
        match self.inode_type {
            INodeType::File => write!(f, "(file, size={})", self.size_cell.get().unwrap()),
            INodeType::Dir(_) => write!(f, "(dir)"),
        }
    }
}

impl INode {
    fn size(&self) -> usize {
        match &self.inode_type {
            INodeType::File => *self.size_cell.get().unwrap(),
            INodeType::Dir(children) => *self
                .size_cell
                .get_or_init(|| children.values().map(|c| c.size()).sum()),
        }
    }

    fn apply<F, O>(&self, f: F) -> Vec<O>
    where
        F: Fn(&INode) -> Option<O> + Copy,
        O: Sized,
    {
        let mut result = vec![];
        if let Some(t) = f(self) {
            result.push(t);
        }
        if let INodeType::Dir(children) = &self.inode_type {
            for child in children.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
                result.extend(child.1.apply(f));
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
enum Cmd {
    Ls,
    Cd(String),
}

impl FromStr for Cmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.strip_prefix("$ ").context("Not a command")?.trim();
        if cmd == "ls" {
            Ok(Cmd::Ls)
        } else {
            let dir = cmd.strip_prefix("cd ").context("Unknown command")?;
            Ok(Cmd::Cd(dir.to_string()))
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Result<Input> {
    let items: Vec<OutputLine> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<OutputLine>>>()?;

    let mut item_iter = items.iter();
    item_iter.next();
    parse_subinput("/", &mut item_iter)
}

fn parse_subinput(name: &str, items: &mut std::slice::Iter<OutputLine>) -> Result<Input> {
    // We're just going to assume we don't go into the same directory twice
    let mut children: HashMap<String, INode> = HashMap::new();

    while let Some(entry) = items.next() {
        if let OutputLine::Ls(node) = entry {
            children
                .entry(node.name.to_string())
                .or_insert_with(|| node.clone());
        } else if let OutputLine::Cmd(Cmd::Cd(dir)) = entry {
            if dir == ".." {
                break;
            }
            let child = parse_subinput(dir, items)?;
            if let Some(current) = children.get_mut(dir) {
                current.inode_type = child.inode_type.clone();
            } else {
                children.insert(dir.to_string(), child);
            }
        }
    }

    Ok(Input {
        name: name.to_string(),
        size_cell: OnceCell::new(),
        inode_type: INodeType::Dir(children),
    })
}

#[allow(dead_code)]
fn print_fs(node: &INode, depth: u32) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("{}", node);
    if let INodeType::Dir(children) = &node.inode_type {
        for child in children.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            print_fs(child.1, depth + 1);
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> Result<Output> {
    // println!("{}", input.size());
    // print_fs(input, 0);
    let result = input
        .apply(|n| {
            if let INodeType::Dir(_) = n.inode_type {
                let size = n.size();
                if size <= 100000 {
                    // println!("{} -> {}", n, size);
                    Some(size)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .iter()
        .sum();
    Ok(result)
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> Result<Output> {
    let free_space = 70000000 - input.size();
    let space_needed = 30000000 - free_space;

    let result = *input
        .apply(|n| {
            if let INodeType::Dir(_) = n.inode_type {
                let size = n.size();
                if size > space_needed {
                    // println!("{} -> {}", n, size);
                    Some(size)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .iter()
        .min()
        .unwrap();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "$ cd /
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
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 95437);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 24933642);
        Ok(())
    }
}
