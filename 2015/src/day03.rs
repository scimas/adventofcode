use std::convert::TryFrom;
use std::ops::{Add, AddAssign, Neg};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

use crate::{InputParseError, Result};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vector2<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T> + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

fn part1(moves: &[Vector2<i64>]) -> Result<u64> {
    let mut pos = Vector2 { x: 0, y: 0 };
    let mut houses: HashSet<Vector2<i64>> = HashSet::new();
    houses.insert(pos);
    moves.iter().for_each(|v| {
        pos += *v;
        houses.insert(pos);
    });
    Ok(u64::try_from(houses.len())?)
}

fn part2(moves: &[Vector2<i64>]) -> Result<u64> {
    let mut santa_pos = Vector2 { x: 0, y: 0 };
    let mut robot_pos = Vector2 { x: 0, y: 0 };
    let mut houses: HashSet<Vector2<i64>> = HashSet::new();
    houses.insert(santa_pos);
    moves.iter().enumerate().for_each(|(i, v)| {
        if i & 1 == 0 {
            santa_pos += *v;
            houses.insert(santa_pos);
        } else {
            robot_pos += *v;
            houses.insert(robot_pos);
        }
    });
    Ok(u64::try_from(houses.len())?)
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input03")?;
    let mut reader = BufReader::new(fl);
    let mut moves_buf = String::with_capacity(1024);
    reader.read_to_string(&mut moves_buf)?;

    let moves: std::result::Result<Vec<Vector2<i64>>, _> = moves_buf
        .trim()
        .chars()
        .map(|ch| match ch {
            '^' => Ok(Vector2 { x: 0, y: 1 }),
            'v' => Ok(Vector2 { x: 0, y: -1 }),
            '>' => Ok(Vector2 { x: 1, y: 0 }),
            '<' => Ok(Vector2 { x: -1, y: 0 }),
            _ => Err(InputParseError::new("Unexpected character in input")),
        })
        .collect();
    let moves = moves?;
    println!("Day 03");
    println!("Part 01: {}", part1(&moves)?);
    println!("Part 02: {}", part2(&moves)?);
    Ok(())
}
