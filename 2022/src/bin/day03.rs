use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input03")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let rucksacks = parse_input(&input);
    println!("Day 3");
    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
    Ok(())
}

type Compartment = HashMap<char, usize>;
type Rucksack = (Compartment, Compartment);

fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let len = line.len();
            let mut compartment_1 = HashMap::new();
            line[..len / 2].chars().for_each(|ch| {
                compartment_1
                    .entry(ch)
                    .and_modify(|quant| *quant += 1)
                    .or_insert_with(|| 1);
            });
            let mut compartment_2 = HashMap::new();
            line[len / 2..].chars().for_each(|ch| {
                compartment_2
                    .entry(ch)
                    .and_modify(|quant| *quant += 1)
                    .or_insert_with(|| 1);
            });
            (compartment_1, compartment_2)
        })
        .collect()
}

fn priority(ch: char) -> u64 {
    if ch >= 'a' && ch <= 'z' {
        u64::from(ch) - u64::from('a') + 1
    } else {
        u64::from(ch) - u64::from('A') + 27
    }
}

fn part1(rucksacks: &[Rucksack]) -> u64 {
    let mut prio = 0;
    for (comp1, comp2) in rucksacks {
        let item_types_1: HashSet<&char> = HashSet::from_iter(comp1.keys());
        let item_types_2 = HashSet::from_iter(comp2.keys());
        for item_type in item_types_1.intersection(&item_types_2) {
            prio += priority(**item_type);
        }
    }
    prio
}

fn find_badge(rucksacks: &[Rucksack]) -> char {
    assert_eq!(rucksacks.len(), 3, "an elf group must have 3 rucksacks");
    let item_types_1: HashSet<&char> =
        HashSet::from_iter(rucksacks[0].0.keys().chain(rucksacks[0].1.keys()));
    let item_types_2: HashSet<&char> =
        HashSet::from_iter(rucksacks[1].0.keys().chain(rucksacks[1].1.keys()));
    let item_types_3: HashSet<&char> =
        HashSet::from_iter(rucksacks[2].0.keys().chain(rucksacks[2].1.keys()));
    let interm: HashSet<&char> = item_types_1.intersection(&item_types_2).copied().collect();
    **interm
        .intersection(&item_types_3)
        .next()
        .expect("each group expected to have a common item")
}

fn part2(rucksacks: &[Rucksack]) -> u64 {
    rucksacks
        .chunks(3)
        .map(|group| find_badge(group))
        .map(|badge| priority(badge))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{find_badge, parse_input, part1, part2};

    fn test_input_1() -> String {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            (
                HashMap::from([
                    ('v', 1),
                    ('J', 2),
                    ('r', 2),
                    ('w', 2),
                    ('p', 1),
                    ('W', 2),
                    ('t', 1),
                    ('g', 1),
                ]),
                HashMap::from([
                    ('h', 2),
                    ('c', 1),
                    ('s', 1),
                    ('F', 4),
                    ('M', 2),
                    ('f', 1),
                    ('h', 2),
                    ('p', 1),
                ]),
            ),
            (
                HashMap::from([
                    ('j', 3),
                    ('q', 3),
                    ('H', 1),
                    ('R', 2),
                    ('N', 1),
                    ('z', 1),
                    ('G', 2),
                    ('D', 1),
                    ('L', 2),
                ]),
                HashMap::from([
                    ('r', 3),
                    ('s', 2),
                    ('F', 3),
                    ('M', 1),
                    ('f', 1),
                    ('Z', 2),
                    ('S', 2),
                    ('L', 2),
                ]),
            ),
            (
                HashMap::from([
                    ('P', 2),
                    ('m', 2),
                    ('d', 1),
                    ('z', 1),
                    ('q', 1),
                    ('r', 1),
                    ('V', 1),
                ]),
                HashMap::from([
                    ('v', 1),
                    ('P', 1),
                    ('w', 3),
                    ('T', 1),
                    ('W', 1),
                    ('B', 1),
                    ('g', 1),
                ]),
            ),
            (
                HashMap::from([
                    ('w', 2),
                    ('M', 3),
                    ('q', 1),
                    ('v', 2),
                    ('L', 2),
                    ('Z', 1),
                    ('H', 3),
                    ('h', 1),
                ]),
                HashMap::from([
                    ('j', 2),
                    ('b', 1),
                    ('v', 2),
                    ('c', 1),
                    ('n', 4),
                    ('S', 1),
                    ('B', 1),
                    ('T', 1),
                    ('Q', 1),
                    ('F', 1),
                ]),
            ),
            (
                HashMap::from([('t', 3), ('g', 1), ('J', 2), ('R', 1), ('G', 1)]),
                HashMap::from([('Q', 1), ('c', 1), ('t', 2), ('T', 2), ('Z', 2)]),
            ),
            (
                HashMap::from([
                    ('C', 1),
                    ('r', 1),
                    ('Z', 2),
                    ('s', 3),
                    ('J', 1),
                    ('P', 2),
                    ('G', 1),
                    ('z', 1),
                ]),
                HashMap::from([
                    ('w', 5),
                    ('s', 1),
                    ('L', 2),
                    ('m', 1),
                    ('p', 1),
                    ('M', 1),
                    ('D', 1),
                ]),
            ),
        ];
        for (i, (rucksack, exp)) in parse_input(&input).iter().zip(expected.iter()).enumerate() {
            assert_eq!(rucksack, exp, "mismatch in {i}th element");
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let rucksacks = parse_input(&input);
        let expected = 157;
        assert_eq!(part1(&rucksacks), expected);
    }

    #[test]
    fn find_badge_test_1() {
        let input = test_input_1();
        let rucksacks = parse_input(&input);
        let expected = ['r', 'Z'];
        for (group, exp) in rucksacks.chunks(3).zip(expected) {
            assert_eq!(find_badge(group), exp);
        }
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let rucksacks = parse_input(&input);
        let expected = 70;
        assert_eq!(part2(&rucksacks), expected);
    }
}
