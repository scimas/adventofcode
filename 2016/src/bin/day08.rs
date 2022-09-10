use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn rect(&mut self, width: usize, height: usize) -> &mut Self {
        assert!(
            width <= self.width,
            "rect width must not be larger than screen width"
        );
        assert!(
            height <= self.height,
            "rect height must not be larger than screen height"
        );
        for j in 0..height {
            for i in 0..width {
                self.pixels[i + self.width * j] = true;
            }
        }
        self
    }

    fn rotate_row_right(&mut self, row: usize, shift: usize) -> &mut Self {
        assert!(
            row < self.height,
            "row must be smaller that the screen height"
        );
        let shift = shift % self.width;
        self.pixels[(self.width * row)..(self.width * (row + 1))].rotate_right(shift);
        self
    }

    fn rotate_column_down(&mut self, column: usize, shift: usize) -> &mut Self {
        assert!(
            column < self.width,
            "column must be smaller than screen width"
        );
        let shift = shift % self.height;
        if shift != self.height {
            let mut column_vec: Vec<bool> = self
                .pixels
                .iter()
                .skip(column)
                .step_by(self.width)
                .copied()
                .collect();
            column_vec.rotate_right(shift);
            self.pixels
                .iter_mut()
                .skip(column)
                .step_by(self.width)
                .zip(column_vec.iter())
                .for_each(|(pix, col)| *pix = *col);
        }
        self
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: String = self
            .pixels
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|pix| if *pix { '\u{2588}' } else { ' ' })
                    .collect::<String>()
                    + "\n"
            })
            .collect();
        write!(f, "{}", repr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Rect { width: usize, height: usize },
    Row { row: usize, shift: usize },
    Column { column: usize, shift: usize },
}

fn main() {
    let fl = File::open("res/day08/input").expect("couldn't open file");
    let reader = BufReader::new(fl);

    let rect_regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    let row_regex = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
    let column_regex = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line from file");
            if let Some(captures) = rect_regex.captures(&line) {
                let width: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let height: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                return Instruction::Rect { width, height };
            }
            if let Some(captures) = row_regex.captures(&line) {
                let row: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let shift: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                return Instruction::Row { row, shift };
            }
            if let Some(captures) = column_regex.captures(&line) {
                let column: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let shift: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                return Instruction::Column { column, shift };
            }
            panic!("line did not match any instruction pattern");
        })
        .collect();

    println!("Day 08");
    let screen = part1(&instructions);
    println!(
        "Part 1: {}",
        screen.pixels.iter().filter(|pix| **pix).count()
    );
    println!("Part 2:");
    println!("{screen}");
}

fn part1(instructions: &[Instruction]) -> Screen {
    let mut screen = Screen::new(50, 6);
    for instruction in instructions {
        match *instruction {
            Instruction::Rect { width, height } => screen.rect(width, height),
            Instruction::Row { row, shift } => screen.rotate_row_right(row, shift),
            Instruction::Column { column, shift } => screen.rotate_column_down(column, shift),
        };
    }
    screen
}
