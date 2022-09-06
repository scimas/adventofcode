use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ndarray::{s, Array2};

use crate::{InputParseError, Result};

#[derive(Default)]
struct Region<T> {
    xmin: T,
    ymin: T,
    xmax: T,
    ymax: T,
}

impl<T> Region<T> {
    fn new(xmin: T, ymin: T, xmax: T, ymax: T) -> Self {
        Self {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

enum Instruction {
    On(Region<usize>),
    Off(Region<usize>),
    Toggle(Region<usize>),
}

fn parse_region<'a, T: Iterator<Item = &'a str>>(mut words: T) -> Result<Region<usize>> {
    let min_points = words
        .next()
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?;
    let mut points = min_points.split(',');
    let xmin = points
        .next()
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?
        .parse()?;
    let ymin = points
        .next()
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?
        .parse()?;
    let max_points = words
        .nth(1)
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?;
    let mut points = max_points.split(',');
    let xmax = points
        .next()
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?
        .parse()?;
    let ymax = points
        .next()
        .ok_or_else(|| InputParseError::new("Unexpected end of input"))?
        .parse()?;
    Ok(Region::new(xmin, ymin, xmax, ymax))
}

fn parse_instruction(line: &str) -> Result<Instruction> {
    let mut words = line.split(' ');
    match words.next() {
        Some("turn") => match words.next() {
            Some("on") => Ok(Instruction::On(parse_region(words)?)),
            Some("off") => Ok(Instruction::Off(parse_region(words)?)),
            _ => Err(InputParseError::new("Unexpected input").into()),
        },
        Some("toggle") => Ok(Instruction::Toggle(parse_region(words)?)),
        _ => Err(InputParseError::new("Unexpected input").into()),
    }
}

fn part1(instructions: &[Instruction]) -> u32 {
    let mut grid = Array2::from_elem((1000, 1000), false);
    for ins in instructions {
        match ins {
            Instruction::On(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| *el = true),
            Instruction::Off(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| *el = false),
            Instruction::Toggle(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| *el = !*el),
        }
    }
    grid.iter().map(|el| if *el { 1 } else { 0 }).sum()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut grid = Array2::from_elem((1000, 1000), 0u64);
    for ins in instructions {
        match ins {
            Instruction::On(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| *el += 1),
            Instruction::Off(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| {
                    *el = match *el {
                        0 => 0,
                        el => el - 1,
                    }
                }),
            Instruction::Toggle(region) => grid
                .slice_mut(s![region.xmin..=region.xmax, region.ymin..=region.ymax])
                .map_inplace(|el| *el += 2),
        }
    }
    grid.sum()
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input06")?;
    let reader = BufReader::new(fl);

    let instructions: std::result::Result<Vec<Instruction>, _> = reader
        .lines()
        .map(|line| parse_instruction(&line?))
        .collect();
    let instructions = instructions?;

    println!("Day 06");
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
    Ok(())
}
