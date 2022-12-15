use std::{collections::VecDeque, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input11")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let (mut monkeys, mut communicators) = parse_input(&input);
    println!("Day 11");
    println!(
        "Part 1: {}",
        part1(&mut monkeys.clone(), &mut communicators.clone())
    );
    println!("Part 2: {}", part2(&mut monkeys, &mut communicators));
    Ok(())
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_levels: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.worry_levels == other.worry_levels
            && self.operation == other.operation
            && self.test == other.test
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(Operand, Operand),
    Mult(Operand, Operand),
}

impl<T> From<T> for Operation
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let mut operation_parts = s.as_ref().splitn(3, ' ');
        let op1 = Operand::from(operation_parts.next().unwrap());
        let opr_str = operation_parts.next().unwrap();
        let op2 = Operand::from(operation_parts.next().unwrap());
        match opr_str {
            "+" => Operation::Add(op1, op2),
            "*" => Operation::Mult(op1, op2),
            _ => panic!("unknown operation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Number(u64),
    Old,
}

impl<T> From<T> for Operand
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref() {
            "old" => Operand::Old,
            n => Operand::Number(n.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
    div: u64,
    true_dest: usize,
    false_dest: usize,
}

impl Test {
    fn new(div: u64, true_dest: usize, false_dest: usize) -> Self {
        Self {
            div,
            true_dest,
            false_dest,
        }
    }
}

impl<T> From<T> for Test
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let (div_str, rest) = s.as_ref().split_once('\n').unwrap();
        let div = div_str[21..].parse().unwrap();
        let (true_str, false_str) = rest.split_once('\n').unwrap();
        let true_dest = true_str[29..].parse().unwrap();
        let false_dest = false_str[30..].parse().unwrap();
        Test::new(div, true_dest, false_dest)
    }
}

impl Monkey {
    fn new(worry_levels: VecDeque<u64>, operation: Operation, test: Test) -> Self {
        Self {
            worry_levels,
            operation,
            test,
        }
    }

    fn receive<'a, T>(&mut self, queue: T)
    where
        T: Iterator<Item = &'a u64>,
    {
        self.worry_levels.extend(queue);
    }

    fn has_items(&self) -> bool {
        !self.worry_levels.is_empty()
    }

    fn inspect(&mut self) {
        if let Some(level) = self.worry_levels.front_mut() {
            match self.operation {
                Operation::Add(op1, op2) => {
                    let op1 = match op1 {
                        Operand::Number(n) => n,
                        Operand::Old => *level,
                    };
                    let op2 = match op2 {
                        Operand::Number(n) => n,
                        Operand::Old => *level,
                    };
                    *level = op1 + op2;
                }
                Operation::Mult(op1, op2) => {
                    let op1 = match op1 {
                        Operand::Number(n) => n,
                        Operand::Old => *level,
                    };
                    let op2 = match op2 {
                        Operand::Number(n) => n,
                        Operand::Old => *level,
                    };
                    *level = op1 * op2;
                }
            }
        }
    }

    fn relieve_worry(&mut self) {
        if let Some(level) = self.worry_levels.front_mut() {
            *level /= 3;
        }
    }

    fn manage_worry(&mut self, base: u64) {
        if let Some(level) = self.worry_levels.front_mut() {
            *level %= base;
        }
    }

    fn test_worry(&mut self, communicators: &mut [Vec<u64>]) {
        if let Some(level) = self.worry_levels.pop_front() {
            if level % self.test.div == 0 {
                communicators
                    .get_mut(self.test.true_dest)
                    .unwrap()
                    .push(level);
            } else {
                communicators
                    .get_mut(self.test.false_dest)
                    .unwrap()
                    .push(level);
            }
        }
    }
}

impl<T> From<T> for Monkey
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let (worries_str, rest) = s.as_ref().split_once('\n').unwrap();
        let worry_levels: VecDeque<u64> = worries_str[18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let (operation_str, test_str) = rest.split_once('\n').unwrap();
        let operation = Operation::from(&operation_str[19..]);
        let test = Test::from(test_str);
        Monkey::new(worry_levels, operation, test)
    }
}

fn parse_input(input: &str) -> (Vec<Monkey>, Vec<Vec<u64>>) {
    let monkeys: Vec<Monkey> = input.trim_end().split("\n\n").map(parse_monkey).collect();
    let communicators = vec![vec![]; monkeys.len()];
    (monkeys, communicators)
}

fn parse_monkey(monkey_str: &str) -> Monkey {
    let (_, rest) = monkey_str.split_once('\n').unwrap();
    Monkey::from(rest)
}

fn inspections_with_relief(
    monkeys: &mut [Monkey],
    communicators: &mut [Vec<u64>],
    rounds: usize,
) -> Vec<usize> {
    let mut counts = Vec::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].receive(communicators[i].iter());
            communicators[i].clear();
            while monkeys[i].has_items() {
                monkeys[i].inspect();
                if let Some(c) = counts.get_mut(i) {
                    *c += 1;
                } else {
                    counts.push(1);
                }
                monkeys[i].relieve_worry();
                monkeys[i].test_worry(communicators);
            }
        }
    }
    counts
}

fn part1(monkeys: &mut [Monkey], communicators: &mut [Vec<u64>]) -> usize {
    let mut inspection_counts = inspections_with_relief(monkeys, communicators, 20);
    inspection_counts.sort_unstable();
    inspection_counts[inspection_counts.len() - 1] * inspection_counts[inspection_counts.len() - 2]
}

fn inspections_without_relief(
    monkeys: &mut [Monkey],
    communicators: &mut [Vec<u64>],
    rounds: usize,
) -> Vec<usize> {
    let base: u64 = monkeys.iter().map(|m| m.test.div).product();
    let mut counts = Vec::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].receive(communicators[i].iter());
            communicators[i].clear();
            while monkeys[i].has_items() {
                monkeys[i].inspect();
                if let Some(c) = counts.get_mut(i) {
                    *c += 1;
                } else {
                    counts.push(1);
                }
                monkeys[i].manage_worry(base);
                monkeys[i].test_worry(communicators);
            }
        }
    }
    counts
}

fn part2(monkeys: &mut [Monkey], communicators: &mut [Vec<u64>]) -> usize {
    let mut inspection_counts = inspections_without_relief(monkeys, communicators, 10_000);
    inspection_counts.sort_unstable();
    inspection_counts[inspection_counts.len() - 1] * inspection_counts[inspection_counts.len() - 2]
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{
        inspections_with_relief, inspections_without_relief, parse_input, part1, part2, Monkey,
        Operand, Operation, Test,
    };

    fn test_input_1() -> String {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            Monkey::new(
                VecDeque::from([79, 98]),
                Operation::Mult(Operand::Old, Operand::Number(19)),
                Test::new(23, 2, 3),
            ),
            Monkey::new(
                VecDeque::from([54, 65, 75, 74]),
                Operation::Add(Operand::Old, Operand::Number(6)),
                Test::new(19, 2, 0),
            ),
            Monkey::new(
                VecDeque::from([79, 60, 97]),
                Operation::Mult(Operand::Old, Operand::Old),
                Test::new(13, 1, 3),
            ),
            Monkey::new(
                VecDeque::from([74]),
                Operation::Add(Operand::Old, Operand::Number(3)),
                Test::new(17, 0, 1),
            ),
        ];
        assert_eq!(parse_input(&input).0, expected);
    }

    #[test]
    fn inspections_with_relief_test() {
        let input = test_input_1();
        let (mut monkeys, mut communicators) = parse_input(&input);
        let expected = vec![101, 95, 7, 105];
        assert_eq!(
            inspections_with_relief(&mut monkeys, &mut communicators, 20),
            expected
        );
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let (mut monkeys, mut communicators) = parse_input(&input);
        assert_eq!(part1(&mut monkeys, &mut communicators), 10_605);
    }

    #[test]
    fn inspections_without_relief_test() {
        let input = test_input_1();
        let (mut monkeys, mut communicators) = parse_input(&input);
        let expected = vec![52_166, 47_830, 1_938, 52_013];
        assert_eq!(
            inspections_without_relief(&mut monkeys, &mut communicators, 10_000),
            expected
        );
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let (mut monkeys, mut communicators) = parse_input(&input);
        assert_eq!(part2(&mut monkeys, &mut communicators), 2713310158);
    }
}
