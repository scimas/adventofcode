use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    ops::{Add, Mul},
};

trait VectorComponents<'a> {
    type Iter: Iterator<Item = &'a i64>;

    fn components(&'a self) -> Self::Iter;
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Component2<'a> {
    v: &'a Vec2,
    idx: usize,
}

impl<'a> Component2<'a> {
    fn new(v: &'a Vec2) -> Self {
        Self { v, idx: 0 }
    }
}

impl<'a> Iterator for Component2<'a> {
    type Item = &'a i64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx {
            0 => {
                self.idx += 1;
                Some(&self.v.x)
            }
            1 => {
                self.idx += 1;
                Some(&self.v.y)
            }
            _ => None,
        }
    }
}

impl<'a> VectorComponents<'a> for Vec2 {
    type Iter = Component2<'a>;

    fn components(&'a self) -> Self::Iter {
        Self::Iter::new(&self)
    }
}

trait L1Norm<'a>: VectorComponents<'a> {
    type Norm;

    fn l1norm(&self) -> Self::Norm;
}

impl<'a> L1Norm<'a> for Vec2 {
    type Norm = i64;

    fn l1norm(&self) -> Self::Norm {
        self.components().map(|c| c.abs()).sum()
    }
}

fn rotate_left(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}

fn rotate_right(v: Vec2) -> Vec2 {
    Vec2::new(v.y, -v.x)
}

fn part1(directions: &[Vec2]) -> i64 {
    let mut final_pos = Vec2::default();
    for d in directions {
        final_pos = final_pos + *d;
    }
    final_pos.l1norm()
}

fn part2(directions: &[Vec2]) -> i64 {
    let mut visited_positions: HashMap<Vec2, u8> = HashMap::new();
    let mut curr_pos = Vec2::default();
    'outer: for d in directions {
        let normed = Vec2::new(d.x / d.l1norm(), d.y / d.l1norm());
        for _ in 0..d.l1norm() {
            curr_pos = curr_pos + normed;
            let visit_count = visited_positions
                .entry(curr_pos)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            if *visit_count > 1 {
                break 'outer;
            }
        }
    }
    curr_pos.l1norm()
}

fn main() {
    let mut fl = File::open("res/day01/input").unwrap();
    let mut s = String::new();
    fl.read_to_string(&mut s).unwrap();

    let mut facing = Vec2::new(0, 1);
    let directions: Vec<Vec2> = s
        .trim()
        .split(", ")
        .map(|s| {
            if &s[..1] == "L" {
                facing = rotate_left(facing);
            } else {
                facing = rotate_right(facing);
            }
            facing * s[1..].parse().unwrap()
        })
        .collect();

    println!("Day 01");
    println!("Part 1: {}", part1(&directions));
    println!("Part 2: {}", part2(&directions));
}
