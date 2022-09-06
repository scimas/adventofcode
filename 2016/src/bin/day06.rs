use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let fl = File::open("res/day06/input").expect("couldn't open file");
    let reader = BufReader::new(fl);
    let messages: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line from file");
            line.chars().collect()
        })
        .collect();
    println!("Day 06");
    println!("Part 1: {}", part1(&messages));
    println!("Part 2: {}", part2(&messages));
}

fn part1(messages: &[Vec<char>]) -> String {
    if messages.is_empty() {
        return "".to_string();
    }
    let width = messages[0].len();
    let mut chars_counts = vec![HashMap::new(); width];
    for message in messages {
        for i in 0..width {
            chars_counts[i]
                .entry(message[i])
                .and_modify(|count| *count += 1)
                .or_insert(1u64);
        }
    }
    chars_counts
        .into_iter()
        .map(|ch_counts| {
            ch_counts
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .unwrap()
                .0
        })
        .collect()
}

fn part2(messages: &[Vec<char>]) -> String {
    if messages.is_empty() {
        return "".to_string();
    }
    let width = messages[0].len();
    let mut chars_counts = vec![HashMap::new(); width];
    for message in messages {
        for i in 0..width {
            chars_counts[i]
                .entry(message[i])
                .and_modify(|count| *count += 1)
                .or_insert(1u64);
        }
    }
    chars_counts
        .into_iter()
        .map(|ch_counts| {
            ch_counts
                .into_iter()
                .min_by_key(|(_, count)| *count)
                .unwrap()
                .0
        })
        .collect()
}
