use std::{fs::File, io::Read};

use crate::Result;

fn part1(secret_key: &str) -> u64 {
    let mut gold = 1;
    return loop {
        let gstring = format!("{}{}", secret_key, gold);
        let hash = md5::compute(gstring);
        if hash.starts_with(&[0, 0]) && hash[2] < 16 {
            break gold;
        }
        gold += 1;
    };
}

fn part2(secret_key: &str, start: u64) -> u64 {
    let mut gold = start;
    return loop {
        let gstring = format!("{}{}", secret_key, gold);
        let hash = md5::compute(gstring);
        if hash.starts_with(&[0, 0, 0]) {
            break gold;
        }
        gold += 1;
    };
}

pub fn main() -> Result<()> {
    let mut fl = File::open("res/input04")?;
    let mut secret_key = String::new();
    fl.read_to_string(&mut secret_key)?;

    println!("Day 04");
    let p1 = part1(secret_key.trim());
    println!("Part 01: {}", p1);
    println!("Part 02: {}", part2(secret_key.trim(), p1));
    Ok(())
}
