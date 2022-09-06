use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Result;

fn sort3<T: PartialOrd>(a: T, b: T, c: T) -> (T, T, T) {
    let (lg, md, sm);
    if c >= b {
        if c >= a {
            lg = c;
            if a >= b {
                md = a;
                sm = b; // b, a, c
            } else {
                md = b;
                sm = a; // a, b, c
            }
        } else {
            lg = a;
            md = c;
            sm = b; // b, c, a
        }
    } else if b >= a {
        lg = b;
        if a >= c {
            md = a;
            sm = c; // c, a, b
        } else {
            md = c;
            sm = a; // a, c, b
        }
    } else {
        lg = a;
        md = b;
        sm = c; // c, b, a
    }
    (sm, md, lg)
}

struct Cuboid {
    sm: u64,
    md: u64,
    lg: u64,
}

impl Cuboid {
    fn new(l: u64, w: u64, h: u64) -> Self {
        let (sm, md, lg) = sort3(l, w, h);
        Self { sm, md, lg }
    }

    fn volume(&self) -> u64 {
        self.lg * self.md * self.sm
    }

    fn surface_area(&self) -> u64 {
        2 * (self.lg * self.md + self.md * self.sm + self.sm * self.lg)
    }
}

fn part1(dimensions: &[Cuboid]) -> u64 {
    dimensions
        .iter()
        .map(|cb| cb.surface_area() + cb.md * cb.sm)
        .sum()
}

fn part2(dimensions: &[Cuboid]) -> u64 {
    dimensions
        .iter()
        .map(|cb| cb.volume() + 2 * (cb.md + cb.sm))
        .sum()
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input02")?;
    let reader = BufReader::new(fl);
    let mut dimensions: Vec<Cuboid> = Vec::with_capacity(1000);
    let mut dim = vec![0, 0, 0];
    for line in reader.lines() {
        for (i, ch) in line?.split('x').enumerate() {
            dim[i] = ch.parse()?;
        }
        dimensions.push(Cuboid::new(dim[0], dim[1], dim[2]));
    }
    println!("Day 02");
    println!("Part 1: {}", part1(&dimensions));
    println!("Part 2: {}", part2(&dimensions));
    Ok(())
}
