use std::{fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input02")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    println!("Day 2");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn parse_input_1(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let s1 = match chars.next().unwrap() {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissor,
                _ => panic!("unknown shape"),
            };
            let s2 = match chars.nth(1).unwrap() {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissor,
                _ => panic!("unknown shape"),
            };
            (s1, s2)
        })
        .collect()
}

fn is_win(round: &(Shape, Shape)) -> Outcome {
    match round {
        &(Shape::Rock, Shape::Paper) => Outcome::Win,
        &(Shape::Rock, Shape::Scissor) => Outcome::Loss,
        &(Shape::Paper, Shape::Rock) => Outcome::Loss,
        &(Shape::Paper, Shape::Scissor) => Outcome::Win,
        &(Shape::Scissor, Shape::Rock) => Outcome::Win,
        &(Shape::Scissor, Shape::Paper) => Outcome::Loss,
        &(Shape::Rock, Shape::Rock) => Outcome::Draw,
        &(Shape::Paper, Shape::Paper) => Outcome::Draw,
        &(Shape::Scissor, Shape::Scissor) => Outcome::Draw,
    }
}

fn score(rounds: &[(Shape, Shape)]) -> u64 {
    rounds
        .iter()
        .fold(0, |acc, el| acc + is_win(el) as u64 + el.1 as u64)
}

fn part1(input: &str) -> u64 {
    let rounds = parse_input_1(input);
    score(&rounds)
}

fn part2(input: &str) -> u64 {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let s1 = match chars.next().unwrap() {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissor,
            _ => panic!("unknown shape"),
        };
        match (s1, chars.nth(1).unwrap()) {
            (Shape::Rock, 'X') => rounds.push((s1, Shape::Scissor)),
            (Shape::Rock, 'Y') => rounds.push((s1, Shape::Rock)),
            (Shape::Rock, 'Z') => rounds.push((s1, Shape::Paper)),
            (Shape::Paper, 'X') => rounds.push((s1, Shape::Rock)),
            (Shape::Paper, 'Y') => rounds.push((s1, Shape::Paper)),
            (Shape::Paper, 'Z') => rounds.push((s1, Shape::Scissor)),
            (Shape::Scissor, 'X') => rounds.push((s1, Shape::Paper)),
            (Shape::Scissor, 'Y') => rounds.push((s1, Shape::Scissor)),
            (Shape::Scissor, 'Z') => rounds.push((s1, Shape::Rock)),
            (_, _) => panic!("unknown shape"),
        }
    }
    score(&rounds)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input_1, part2, score, Shape};

    fn test_input_1() -> String {
        "A Y
B X
C Z
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissor, Shape::Scissor),
        ];
        assert_eq!(parse_input_1(&input), expected);
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let rounds = parse_input_1(&input);
        let expected = 15;
        assert_eq!(score(&rounds), expected);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let expected = 12;
        assert_eq!(part2(&input), expected);
    }
}
