use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;
use thiserror::Error;

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input07")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let root = parse_input(&input)?;
    println!("Day 7");
    println!("Part 1: {}", part1(&root));
    println!("Part 2: {}", part2(&root));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FSEntry {
    Directory {
        entries: HashMap<String, FSEntry>,
        size: usize,
    },
    File {
        size: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
enum FSError {
    #[error("{0} is valid for a directory only")]
    NotADirectory(String),
    #[error("path {0} already exists")]
    PathAlreadyExists(String),
}

impl FSEntry {
    fn new_directory(entries: HashMap<String, FSEntry>, size: usize) -> Self {
        Self::Directory { entries, size }
    }

    fn new_file(size: usize) -> Self {
        Self::File { size }
    }

    fn size(&self) -> usize {
        match self {
            FSEntry::Directory { size, .. } => *size,
            FSEntry::File { size } => *size,
        }
    }

    fn add_entry(&mut self, path: &[&str], entry: FSEntry) -> Result<(), FSError> {
        match self {
            FSEntry::File { .. } => Err(FSError::NotADirectory("add_entry".into())),
            FSEntry::Directory { entries, size } => match path.split_first() {
                Some((first, rest)) => {
                    if rest.is_empty() {
                        if entries.contains_key(*first) {
                            Err(FSError::PathAlreadyExists(path.join("/")))
                        } else {
                            *size += entry.size();
                            entries.insert(first.to_string(), entry);
                            Ok(())
                        }
                    } else {
                        *size += entry.size();
                        entries
                            .entry(first.to_string())
                            .or_insert_with(|| FSEntry::new_directory(HashMap::new(), 0))
                            .add_entry(rest, entry)
                    }
                }
                None => Err(FSError::PathAlreadyExists(".".into())),
            },
        }
    }

    #[allow(dead_code)]
    fn get(&self, path: &[&str]) -> Option<&FSEntry> {
        match path.split_first() {
            None => Some(self),
            Some((first, rest)) => match self {
                FSEntry::File { .. } => None,
                FSEntry::Directory { entries, .. } => {
                    entries.get(*first).and_then(|entry| entry.get(rest))
                }
            },
        }
    }
}

fn parse_input(input: &str) -> Result<FSEntry, FSError> {
    let cd_pattern = Regex::new(r#"^\$ cd (.+)$"#).expect("pattern is valid");
    let ls_pattern = Regex::new(r#"^\$ ls$"#).expect("pattern is valid");
    let dir_pattern = Regex::new(r#"^dir (.+)$"#).expect("pattern is valid");
    let file_pattern = Regex::new(r#"^(\d+) (.+)$"#).expect("pattern is valid");
    let mut root = FSEntry::new_directory(HashMap::new(), 0);
    let mut current_path: Vec<&str> = Vec::new();
    for line in input.lines().skip(1) {
        if let Some(caps) = cd_pattern.captures(line) {
            let dir_name = caps.get(1).unwrap().as_str();
            match dir_name {
                ".." => {
                    current_path.pop();
                }
                "/" => current_path.clear(),
                dir_name => current_path.push(dir_name),
            }
            match root.add_entry(&current_path, FSEntry::new_directory(HashMap::new(), 0)) {
                Ok(_) | Err(FSError::PathAlreadyExists(_)) => continue,
                e => e?,
            }
        }
        if ls_pattern.is_match(line) {
            continue;
        }
        if let Some(caps) = dir_pattern.captures(line) {
            let dir_name = caps.get(1).unwrap().as_str();
            current_path.push(dir_name);
            match root.add_entry(&current_path, FSEntry::new_directory(HashMap::new(), 0)) {
                Ok(_) | Err(FSError::PathAlreadyExists(_)) => {
                    current_path.pop();
                    continue;
                }
                e => e?,
            }
        }
        match file_pattern.captures(line) {
            Some(caps) => {
                let file_size = caps.get(1).unwrap().as_str().parse().unwrap();
                let file_name = caps.get(2).unwrap().as_str();
                current_path.push(file_name);
                root.add_entry(&current_path, FSEntry::new_file(file_size))?;
                current_path.pop();
            }
            None => panic!("input does not match any pattern"),
        }
    }
    Ok(root)
}

fn part1(entry: &FSEntry) -> usize {
    match entry {
        FSEntry::File { .. } => 0,
        FSEntry::Directory { entries, .. } => {
            let size = entry.size();
            let inner_size = entries.values().map(part1).sum();
            if size <= 100_000 {
                size + inner_size
            } else {
                inner_size
            }
        }
    }
}

fn sizes(entry: &FSEntry) -> Vec<usize> {
    match entry {
        FSEntry::File { .. } => vec![],
        FSEntry::Directory { entries, size } => {
            let mut entry_sizes = vec![*size];
            entry_sizes.extend(entries.values().flat_map(sizes));
            entry_sizes
        }
    }
}

fn part2(entry: &FSEntry) -> usize {
    let to_be_freed = 30_000_000 - (70_000_000 - entry.size());
    let sizes = sizes(entry);
    sizes
        .into_iter()
        .filter(|sz| sz >= &to_be_freed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_input, part1, part2, FSEntry};

    fn test_input_1() -> String {
        "$ cd /
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
7214296 k
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = FSEntry::new_directory(
            HashMap::from([
                (
                    "a".into(),
                    FSEntry::new_directory(
                        HashMap::from([
                            (
                                "e".into(),
                                FSEntry::new_directory(
                                    HashMap::from([("i".into(), FSEntry::new_file(584))]),
                                    584,
                                ),
                            ),
                            ("f".into(), FSEntry::new_file(29116)),
                            ("g".into(), FSEntry::new_file(2557)),
                            ("h.lst".into(), FSEntry::new_file(62596)),
                        ]),
                        94853,
                    ),
                ),
                (
                    "d".into(),
                    FSEntry::new_directory(
                        HashMap::from([
                            ("d.ext".into(), FSEntry::new_file(5626152)),
                            ("d.log".into(), FSEntry::new_file(8033020)),
                            ("j".into(), FSEntry::new_file(4060174)),
                            ("k".into(), FSEntry::new_file(7214296)),
                        ]),
                        24933642,
                    ),
                ),
                ("b.txt".into(), FSEntry::new_file(14848514)),
                ("c.dat".into(), FSEntry::new_file(8504156)),
            ]),
            48381165,
        );
        assert_eq!(parse_input(&input), Ok(expected));
    }

    #[test]
    fn size_test() {
        let input = test_input_1();
        let root = parse_input(&input).unwrap();
        let paths = vec![vec![], vec!["a"], vec!["a", "e"], vec!["d"]];
        let sizes = vec![48381165, 94853, 584, 24933642];
        for (path, size) in paths.iter().zip(sizes) {
            assert_eq!(
                root.get(path).unwrap().size(),
                size,
                "incorrect size for path {}",
                path.join("/")
            );
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let root = parse_input(&input).unwrap();
        let expected = 94853 + 584;
        assert_eq!(part1(&root), expected);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let root = parse_input(&input).unwrap();
        let expected = 24933642;
        assert_eq!(part2(&root), expected);
    }
}
