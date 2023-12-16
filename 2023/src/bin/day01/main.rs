use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("res/day01")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    println!("Day 01");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digit1 = line
                .chars()
                .find(|ch| ch.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let digit2 = line
                .chars()
                .rev()
                .find(|ch| ch.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            digit1 * 10 + digit2
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    const DIGITS_THREE: [&str; 3] = ["one", "two", "six"];
    const DIGITS_FOUR: [&str; 3] = ["four", "five", "nine"];
    const DIGITS_FIVE: [&str; 3] = ["three", "seven", "eight"];
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    input
        .lines()
        .map(|line| {
            let characters: Vec<char> = line.chars().collect();
            let mut digits = Vec::new();
            let mut i = 0;
            while i < line.len() {
                match characters[i] {
                    digit @ '0'..='9' => {
                        digits.push(digit.to_digit(10).unwrap());
                        i += 1;
                    }
                    _ => {
                        let three_range = i..(i + 3).min(line.len());
                        let four_range = i..(i + 4).min(line.len());
                        let five_range = i..(i + 5).min(line.len());
                        i += 1;
                        match DIGITS_THREE
                            .iter()
                            .find(|s| s == &&&line[three_range.clone()])
                        {
                            Some(digit) => {
                                digits.push(digit_map[digit]);
                                continue;
                            }
                            None => (),
                        }
                        match DIGITS_FOUR
                            .iter()
                            .find(|s| s == &&&line[four_range.clone()])
                        {
                            Some(digit) => {
                                digits.push(digit_map[digit]);
                                continue;
                            }
                            None => (),
                        }
                        match DIGITS_FIVE
                            .iter()
                            .find(|s| s == &&&line[five_range.clone()])
                        {
                            Some(digit) => {
                                digits.push(digit_map[digit]);
                                continue;
                            }
                            None => (),
                        }
                    }
                }
            }
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = include_str!("test_input01");
    assert_eq!(142, part1(input));
}

#[test]
fn test_part2() {
    let input = include_str!("test_input02");
    assert_eq!(281, part2(input));
}
