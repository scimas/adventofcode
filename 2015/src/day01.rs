use std::{fs::File, io::Read};

use crate::Result;

fn part1(directions: &[i64]) -> i64 {
    directions.iter().sum()
}

fn part2(directions: &[i64]) -> usize {
    let mut floor = 0;
    for (i, dir) in directions.iter().enumerate() {
        floor += *dir;
        if floor < 0 {
            return i + 1;
        }
    }
    0
}

pub fn main() -> Result<()> {
    let mut fl = File::open("res/input01")?;
    let mut s = String::with_capacity(7001);
    fl.read_to_string(&mut s)?;
    let directions: Vec<i64> = s
        .chars()
        .map(|ch| {
            if ch == '(' {
                1
            } else if ch == ')' {
                -1
            } else {
                0
            }
        })
        .collect();
    println!("Day 01");
    println!("Part 1: {}", part1(&directions));
    println!("Part 2: {}", part2(&directions));
    Ok(())
}
