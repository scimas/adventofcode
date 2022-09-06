use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{InputParseError, Result};

struct EscapeStringInterpreter<'a> {
    inner_iter: std::str::Chars<'a>,
}

impl<'a> Iterator for EscapeStringInterpreter<'_> {
    type Item = std::result::Result<char, InputParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|c| match c {
            '\\' => match self.inner_iter.next() {
                None => Err(InputParseError::new("Incomplete escape sequence")),
                Some('\\') => Ok('\\'),
                Some('"') => Ok('"'),
                Some('x') => match self.inner_iter.next() {
                    None => Err(InputParseError::new("Incomplete escape sequence")),
                    Some(c1 @ ('0'..='9' | 'a'..='f')) => match self.inner_iter.next() {
                        None => Err(InputParseError::new("Incomplete escape sequence")),
                        Some(c2 @ ('0'..='9' | 'a'..='f')) => {
                            match char::from_u32(
                                c1.to_digit(0x10).unwrap() * 0x10 + c2.to_digit(0x10).unwrap(),
                            ) {
                                None => Err(InputParseError::new("Unknown escape sequence")),
                                Some(c) => Ok(c),
                            }
                        }
                        _ => Err(InputParseError::new("Unknown escape sequence")),
                    },
                    _ => Err(InputParseError::new("Unknown escape sequence")),
                },
                _ => Err(InputParseError::new("Unknown escape sequence")),
            },
            c => Ok(c),
        })
    }
}

fn part1(strings: &[String]) -> usize {
    let mut total = 0;
    for s in strings {
        let unescaper = EscapeStringInterpreter {
            inner_iter: s.chars(),
        };
        total += s.chars().count() - unescaper.count() + 2;
    }
    total
}

fn part2(strings: &[String]) -> usize {
    let mut total = 0;
    for s in strings {
        total += s.escape_default().count() - s.chars().count() + 2;
    }
    total
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input08")?;
    let reader = BufReader::new(fl);
    let strings: std::result::Result<Vec<String>, _> = reader.lines().collect();
    let strings = strings?;

    println!("Day 08");
    println!("Part 1: {}", part1(&strings));
    println!("Part 2: {}", part2(&strings));
    Ok(())
}
