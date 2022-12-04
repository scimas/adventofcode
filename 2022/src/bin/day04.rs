use std::{fs::File, io::Read, ops::RangeInclusive};

use regex::Regex;

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input04")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let elf_pairs = parse_input(&input);
    println!("Day 4");
    println!("Part 1: {}", part1(&elf_pairs));
    println!("Part 2: {}", part2(&elf_pairs));
    Ok(())
}

type SectionRange = RangeInclusive<usize>;

fn parse_input(input: &str) -> Vec<(SectionRange, SectionRange)> {
    let mut elf_pairs = Vec::new();
    let section_pattern = Regex::new(r#"^(\d+)-(\d+),(\d+)-(\d+)$"#).expect("pattern is valid");
    for line in input.lines() {
        match section_pattern.captures(line) {
            Some(captures) => {
                let elf1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
                    ..=captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let elf2 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap()
                    ..=captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
                assert!(
                    (elf1.start() <= elf1.end()) && (elf2.start() <= elf2.end()),
                    "invalid range on line {line}"
                );
                elf_pairs.push((elf1, elf2));
            }
            None => panic!("each line must have 2 elves"),
        }
    }
    elf_pairs
}

/// Do ranges intersect
fn is_intersection(range1: &SectionRange, range2: &SectionRange) -> bool {
    (range1.start() <= range2.start() && range1.end() >= range2.start())
        || (range2.start() <= range1.start() && range2.end() >= range1.start())
}

/// Is one a subset of the other? Checks both ways
fn is_subset(range1: &SectionRange, range2: &SectionRange) -> bool {
    (range1.start() <= range2.start() && range2.end() <= range1.end())
        || (range2.start() <= range1.start() && range1.end() <= range2.end())
}

fn part1(elf_pairs: &[(SectionRange, SectionRange)]) -> usize {
    elf_pairs
        .iter()
        .filter(|(range1, range2)| is_subset(range1, range2))
        .count()
}

fn part2(elf_pairs: &[(SectionRange, SectionRange)]) -> usize {
    elf_pairs
        .iter()
        .filter(|(range1, range2)| is_intersection(range1, range2))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{is_intersection, is_subset, parse_input, part1, part2};

    fn test_input_1() -> String {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ];
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn is_intersection_test() {
        let pairs = vec![(2..=3, 4..=5), (5..=7, 7..=9), (2..=8, 3..=7)];
        let expected = vec![false, true, true];
        for ((range1, range2), exp) in pairs.iter().zip(expected) {
            assert_eq!(
                is_intersection(range1, range2),
                exp,
                "failure for {range1:?}, {range2:?}"
            );
        }
    }

    #[test]
    fn is_subset_test() {
        let pairs = vec![
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
        ];
        let expected = vec![false, false, true, true];
        for ((range1, range2), exp) in pairs.iter().zip(expected) {
            assert_eq!(
                is_subset(range1, range2),
                exp,
                "failure for {range1:?}, {range2:?}"
            );
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let elf_pairs = parse_input(&input);
        assert_eq!(part1(&elf_pairs), 2);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let elf_pairs = parse_input(&input);
        assert_eq!(part2(&elf_pairs), 4);
    }
}
