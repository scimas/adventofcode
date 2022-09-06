use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Result;

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn part1(strings: &[String]) -> u64 {
    let mut nice_count = 0;
    for s in strings {
        let mut vowel_count: u8 = 0;
        for ch in s.chars() {
            vowel_count += match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => 1,
                _ => 0,
            };
            if vowel_count == 3 {
                break;
            }
        }
        if vowel_count != 3 {
            continue;
        }
        let mut double_char_found = false;
        let mut forbidden_pair_found = false;
        for two_chars in char_windows(s, 2) {
            match two_chars {
                "ab" | "cd" | "pq" | "xy" => {
                    forbidden_pair_found = true;
                    break;
                }
                chars => {
                    if !double_char_found && (chars[..1] == chars[1..]) {
                        double_char_found = true;
                    }
                }
            }
        }
        if forbidden_pair_found || !double_char_found {
            continue;
        }
        nice_count += 1;
    }
    nice_count
}

fn part2(strings: &[String]) -> u64 {
    let mut nice_count = 0;
    for s in strings {
        let mut single_repeater_found = false;
        for three_chars in char_windows(s, 3) {
            if three_chars[..1] == three_chars[2..] {
                single_repeater_found = true;
                break;
            }
        }
        if !single_repeater_found {
            continue;
        }
        let mut pair_positions: HashMap<String, Vec<usize>> = HashMap::new();
        for (idx, two_chars) in char_windows(s, 2).enumerate() {
            let positions = pair_positions
                .entry(two_chars.to_owned())
                .or_insert_with(Vec::new);
            positions.push(idx);
        }
        let mut repeating_pair_found = false;
        for (_, pos) in pair_positions {
            if pos.windows(2).any(|ps| ps[1] - ps[0] >= 2) {
                repeating_pair_found = true;
                break;
            }
        }
        if !repeating_pair_found {
            continue;
        }
        nice_count += 1;
    }
    nice_count
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input05")?;
    let reader = BufReader::new(fl);
    let strings: std::result::Result<Vec<String>, _> = reader.lines().collect();
    let strings = strings?;

    println!("Day 05");
    println!("Part 1: {}", part1(&strings));
    println!("Part 2: {}", part2(&strings));
    Ok(())
}
