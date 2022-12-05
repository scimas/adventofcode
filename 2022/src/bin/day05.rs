use std::{fs::File, io::Read};

use regex::Regex;

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input05")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let (stacks, moves) = parse_input(&input);
    println!("Day 5");
    println!("Part 1: {}", part1(stacks.clone(), &moves));
    println!("Part 2: {}", part2(stacks, &moves));
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    source: usize,
    destination: usize,
    quantity: usize,
}

impl Move {
    fn new(source: usize, destination: usize, quantity: usize) -> Self {
        Self {
            source,
            destination,
            quantity,
        }
    }
}

type Stack = Vec<char>;

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let stack_str = parts.next().unwrap();
    let moves_str = parts.next().unwrap();

    let (stack_str, stack_nums) = stack_str.rsplit_once('\n').unwrap();
    let num_stacks = stack_nums.split_whitespace().count();
    let mut stacks = vec![vec![]; num_stacks + 1];
    for line in stack_str.lines().rev() {
        for (ch, stack) in line.chars().skip(1).step_by(4).zip(&mut stacks[1..]) {
            if ch != ' ' {
                stack.push(ch);
            }
        }
    }

    let move_pattern = Regex::new(r#"\d+"#).expect("pattern is valid");
    let mut moves = Vec::new();
    for line in moves_str.lines() {
        let mut matches = move_pattern.find_iter(line);
        let quantity = matches.next().unwrap().as_str().parse().unwrap();
        let source = matches.next().unwrap().as_str().parse().unwrap();
        let destination = matches.next().unwrap().as_str().parse().unwrap();
        moves.push(Move::new(source, destination, quantity));
    }
    (stacks, moves)
}

fn part1(mut stacks: Vec<Stack>, moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.quantity {
            let source = stacks[m.source]
                .pop()
                .expect("expecting stack to have at least one item");
            stacks[m.destination].push(source);
        }
    }
    stacks[1..]
        .iter()
        .map(|stack| stack.last().unwrap_or_else(|| &' '))
        .collect()
}

fn part2(mut stacks: Vec<Stack>, moves: &[Move]) -> String {
    for m in moves {
        let source_len = stacks[m.source].len();
        let source = stacks[m.source].split_off(source_len - m.quantity);
        stacks[m.destination].extend_from_slice(&source);
    }
    stacks[1..]
        .iter()
        .map(|stack| stack.last().unwrap_or_else(|| &' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2, Move};

    fn test_input_1() -> String {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let exp_stacks = vec![vec![], vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let exp_moves = vec![
            Move::new(2, 1, 1),
            Move::new(1, 3, 3),
            Move::new(2, 1, 2),
            Move::new(1, 2, 1),
        ];
        assert_eq!(parse_input(&input), (exp_stacks, exp_moves));
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let (stacks, moves) = parse_input(&input);
        let expected = "CMZ";
        assert_eq!(part1(stacks, &moves), expected);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let (stacks, moves) = parse_input(&input);
        let expected = "MCD";
        assert_eq!(part2(stacks, &moves), expected);
    }
}
