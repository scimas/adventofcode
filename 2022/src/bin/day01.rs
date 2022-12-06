use std::{fs::File, io::Read, num::ParseIntError};

fn main() -> Result<(), anyhow::Error> {
    let mut file = File::open("resources/input01")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let elves = parse_input(&input)?;
    println!("Day 1");
    println!("Part 1: {:?}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, ParseIntError> {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = Vec::new();
            continue;
        }
        elf.push(line.parse::<u64>()?);
    }
    elves.push(elf);
    Ok(elves)
}

fn part1(elves: &[Vec<u64>]) -> Option<u64> {
    elves.iter().map(|elf| elf.iter().sum()).max()
}

fn part2(elves: &[Vec<u64>]) -> u64 {
    assert!(elves.len() >= 3);
    let mut calories: Vec<u64> = elves.iter().map(|elf| elf.iter().sum()).collect();
    calories.sort_unstable_by(|a, b| b.cmp(a));
    calories[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    fn test_input_1() -> String {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        match parse_input(&input) {
            Ok(elves) => assert_eq!(elves, expected),
            Err(e) => panic!("couldn't parse input as u64 {e}"),
        };
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let elves = parse_input(&input).unwrap();
        assert_eq!(part1(&elves), Some(24000));
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let elves = parse_input(&input).unwrap();
        assert_eq!(part2(&elves), 45000);
    }
}
