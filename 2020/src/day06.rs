use std::collections::HashSet;
use std::fs;

fn load_data() -> Vec<Vec<HashSet<char>>> {
    let answer_list = fs::read_to_string("res/input06.txt").expect("Couldn't read day 6 input");
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in answer_list.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> = group
            .lines()
            .filter(|line| line.contains(|ch: char| !ch.is_whitespace()))
            .map(|line| line.chars().filter(|ch| !ch.is_whitespace()).collect())
            .collect();
        groups.push(grp_vec);
    }
    groups
}

fn group_union(groups: Vec<Vec<HashSet<char>>>) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(HashSet::new(), |acc, x| acc.union(x).copied().collect())
                .len()
        })
        .sum()
}

pub fn part1() -> usize {
    let groups = load_data();
    group_union(groups)
}

fn group_intersection(groups: Vec<Vec<HashSet<char>>>) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(('a'..='z').collect(), |acc: HashSet<char>, x| {
                    acc.intersection(x).copied().collect()
                })
                .len()
        })
        .sum()
}

pub fn part2() -> usize {
    let groups = load_data();
    group_intersection(groups)
}

#[test]
fn union_test() {
    let s = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in s.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> =
            group.lines().map(|line| line.chars().collect()).collect();
        groups.push(grp_vec);
    }
    assert_eq!(group_union(groups), 11);
    let s = "abcx\nabcy\nabcz".to_string();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in s.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> =
            group.lines().map(|line| line.chars().collect()).collect();
        groups.push(grp_vec);
    }
    assert_eq!(group_union(groups), 6);
}

#[test]
fn intersection_test() {
    let s = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in s.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> =
            group.lines().map(|line| line.chars().collect()).collect();
        groups.push(grp_vec);
    }
    assert_eq!(group_intersection(groups), 6);
    let s = "abcx\nabcy\nabcz".to_string();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in s.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> =
            group.lines().map(|line| line.chars().collect()).collect();
        groups.push(grp_vec);
    }
    assert_eq!(group_intersection(groups), 3);
    let s: String = ('a'..='z').collect();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    for group in s.split("\n\n") {
        let grp_vec: Vec<HashSet<char>> = group
            .lines()
            .filter(|line| line.contains(|ch: char| !ch.is_whitespace()))
            .map(|line| line.chars().collect())
            .collect();
        groups.push(grp_vec);
    }
    assert_eq!(group_intersection(groups), 26);
}
