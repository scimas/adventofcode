use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {}

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn new(min: usize, max: usize, letter: char) -> Policy {
        Policy { min, max, letter }
    }
}

impl From<&str> for Policy {
    fn from(s: &str) -> Self {
        let letter_split: Vec<&str> = s.split(' ').collect();
        let letter: char = char::from_str(letter_split[1]).expect("Couldn't convert str to char");
        let bounds: Vec<usize> = letter_split[0]
            .split('-')
            .map(|n| usize::from_str(n).expect("Couldn't convert policy bound from str"))
            .collect();
        Policy::new(bounds[0], bounds[1], letter)
    }
}

fn is_valid_password_part1(policy: &Policy, password: &str) -> bool {
    let letter_count = password.chars().filter(|ch| ch == &policy.letter).count();
    policy.min <= letter_count && letter_count <= policy.max
}

pub fn part1() -> usize {
    let f = File::open("res/input02.txt").expect("Couldn't read day 2 input");
    let reader = BufReader::new(f);
    let mut count: usize = 0;
    for line in reader.lines() {
        if let Ok(s) = line {
            let policy_split: Vec<&str> = s.split(": ").collect();
            let policy: Policy = policy_split[0].into();
            count += is_valid_password_part1(&policy, policy_split[1]) as usize;
        }
    }
    count
}

fn is_valid_password_part2(policy: &Policy, password: &str) -> bool {
    let mut char_iter = password.chars();
    if let Some(ch1) = char_iter.nth(policy.min - 1) {
        if let Some(ch2) = char_iter.nth(policy.max - 1 - policy.min) {
            (ch1 == policy.letter) ^ (ch2 == policy.letter)
        } else {
            ch1 == policy.letter
        }
    } else {
        false
    }
}

pub fn part2() -> usize {
    let f = File::open("res/input02.txt").expect("Couldn't read day 2 input");
    let reader = BufReader::new(f);
    let mut count: usize = 0;
    for line in reader.lines() {
        if let Ok(s) = line {
            let policy_split: Vec<&str> = s.split(": ").collect();
            let policy: Policy = policy_split[0].into();
            count += is_valid_password_part2(&policy, policy_split[1]) as usize;
        }
    }
    count
}

#[test]
fn password_validation_part1() {
    let p = Policy::new(1, 3, 'a');
    assert!(is_valid_password_part1(&p, "abcde"));
    assert!(is_valid_password_part1(&p, "baaa"));
    assert!(!is_valid_password_part1(&p, "cdefg"));
    assert!(!is_valid_password_part1(&p, "aaaa"));
}

#[test]
fn password_validation_part2() {
    let p = Policy::new(1, 3, 'a');
    assert!(is_valid_password_part2(&p, "abcde"));
    assert!(is_valid_password_part2(&p, "bcat"));
    assert!(is_valid_password_part2(&p, "ad"));
    assert!(!is_valid_password_part2(&p, "cdefg"));
    assert!(!is_valid_password_part2(&p, "aaad"));
    assert!(!is_valid_password_part2(&p, "cd"));
}
