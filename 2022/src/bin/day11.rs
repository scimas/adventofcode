use std::collections::{HashMap, VecDeque};

fn main() -> Result<(), anyhow::Error> {
    todo!()
}

#[derive(Debug)]
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
            *level /= 3;
        }
    }

    fn test_worry(&mut self, communicators: &mut HashMap<usize, Vec<u64>>) {
        if let Some(level) = self.worry_levels.pop_front() {
            if level % self.test.div == 0 {
                communicators
                    .get_mut(&self.test.true_dest)
                    .unwrap()
                    .push(level);
            } else {
                communicators
                    .get_mut(&self.test.false_dest)
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

fn parse_input(input: &str) -> (HashMap<usize, Monkey>, HashMap<usize, Vec<u64>>) {
    let monkeys: HashMap<usize, Monkey> =
        input.trim_end().split("\n\n").map(parse_monkey).collect();
    let communicators = (0..monkeys.len()).map(|idx| (idx, Vec::new())).collect();
    (monkeys, communicators)
}

fn parse_monkey(monkey_str: &str) -> (usize, Monkey) {
    let (monkey_num_str, rest) = monkey_str.split_once('\n').unwrap();
    let monkey_num = monkey_num_str[7..monkey_num_str.len() - 1].parse().unwrap();
    (monkey_num, Monkey::from(rest))
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use crate::{parse_input, Monkey, Operand, Operation, Test};

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
        let expected = HashMap::from([
            (
                0,
                Monkey::new(
                    VecDeque::from([79, 98]),
                    Operation::Mult(Operand::Old, Operand::Number(19)),
                    Test::new(23, 2, 3),
                ),
            ),
            (
                1,
                Monkey::new(
                    VecDeque::from([54, 65, 75, 74]),
                    Operation::Add(Operand::Old, Operand::Number(6)),
                    Test::new(19, 2, 0),
                ),
            ),
            (
                2,
                Monkey::new(
                    VecDeque::from([79, 60, 97]),
                    Operation::Mult(Operand::Old, Operand::Old),
                    Test::new(13, 1, 3),
                ),
            ),
            (
                3,
                Monkey::new(
                    VecDeque::from([74]),
                    Operation::Add(Operand::Old, Operand::Number(3)),
                    Test::new(17, 0, 1),
                ),
            ),
        ]);
        assert_eq!(parse_input(&input).0, expected);
    }
}
