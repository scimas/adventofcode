use std::collections::HashMap;

use md5::{Digest, Md5};

const INPUT: &str = "ffykfhsq";

fn main() {
    println!("Day 05");
    println!("Part 1: {}", part1(INPUT.to_string()));
    println!("Part 2: {}", part2(INPUT.to_string()));
}

fn part1(door_id: String) -> String {
    let mut hasher = Md5::new();
    let mut password: Vec<u8> = vec![];
    let mut idx = 0usize;
    while password.len() < 8 {
        let input = format!("{door_id}{idx}");
        hasher.update(input);
        let hash = hasher.finalize_reset();
        let hash = base16ct::lower::encode_string(&hash);
        let hash = hash.as_bytes();
        if hash.starts_with(b"00000") {
            password.push(hash[5]);
        }
        idx += 1;
    }
    String::from_utf8(password).unwrap()
}

fn part2(door_id: String) -> String {
    let mut hasher = Md5::new();
    let mut password: HashMap<u8, u8> = HashMap::new();
    let mut idx = 0usize;
    while password.len() < 8 {
        let input = format!("{door_id}{idx}");
        hasher.update(input);
        let hash = hasher.finalize_reset();
        let hash = base16ct::lower::encode_string(&hash);
        let hash = hash.as_bytes();
        if hash.starts_with(b"00000") {
            if hash[5] >= 48 && hash[5] < 56 {
                password.entry(hash[5]).or_insert(hash[6]);
            }
        }
        idx += 1;
    }
    let mut password: Vec<_> = password.into_iter().collect();
    password.sort_unstable_by_key(|val| val.0);
    let password = password.into_iter().map(|val| val.1).collect();
    String::from_utf8(password).unwrap()
}
