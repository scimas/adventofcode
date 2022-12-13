use std::{collections::VecDeque, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input10")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let instructions = parse_input(&input);
    let mut cpu = Cpu::new_with_instructions(instructions.clone());
    println!("Day 10");
    println!("Part 1: {}", part1(&mut cpu));
    cpu.reset();
    cpu.set_instructions(instructions);
    println!("Part 2:");
    part2(&mut cpu);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cpu {
    cycle: usize,
    x: i64,
    instructions: VecDeque<(Instruction, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    NoOp,
    AddX { v: i64 },
}
use Instruction::{AddX, NoOp};

impl Instruction {
    fn completion_cycles(&self) -> usize {
        match self {
            NoOp => 1,
            AddX { .. } => 2,
        }
    }
}

impl Cpu {
    fn new_with_instructions(instructions: VecDeque<(Instruction, usize)>) -> Self {
        Self {
            cycle: 0,
            x: 1,
            instructions,
        }
    }

    fn compute(&mut self) {
        if let Some((instruction, mut cycles)) = self.instructions.pop_front() {
            cycles += 1;
            if cycles >= instruction.completion_cycles() {
                match instruction {
                    NoOp => (),
                    AddX { v } => {
                        self.x += v;
                    }
                }
            } else {
                self.instructions.push_front((instruction, cycles));
            }
        }
    }

    fn draw(&self) {
        if ((self.x - 1)..=(self.x + 1)).contains(&(self.cycle as i64 % 40)) {
            print!("#");
        } else {
            print!(" ");
        }
        if (self.cycle + 1) % 40 == 0 {
            println!();
        }
    }

    fn tick(&mut self) {
        self.compute();
        self.cycle += 1;
    }

    fn tick_with_draw(&mut self) {
        self.draw();
        self.compute();
        self.cycle += 1;
    }

    fn signal_strength(&self) -> i64 {
        (self.cycle + 1) as i64 * self.x
    }

    fn is_completed(&self) -> bool {
        self.instructions.is_empty()
    }

    fn reset(&mut self) {
        self.cycle = 0;
        self.x = 1;
        self.instructions = VecDeque::new();
    }

    fn set_instructions(&mut self, instructions: VecDeque<(Instruction, usize)>) {
        self.instructions = instructions;
    }
}

fn parse_input(input: &str) -> VecDeque<(Instruction, usize)> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                (NoOp, 0)
            } else if line.starts_with("addx") {
                (
                    AddX {
                        v: line[5..].parse().unwrap(),
                    },
                    0,
                )
            } else {
                panic!("unknown instruction")
            }
        })
        .collect()
}

fn part1(cpu: &mut Cpu) -> i64 {
    let mut total_signal_strength = 0;
    while !cpu.is_completed() {
        cpu.tick();
        if cpu.cycle + 1 == 20 {
            total_signal_strength += cpu.signal_strength();
            break;
        }
    }
    while !cpu.is_completed() {
        cpu.tick();
        if (cpu.cycle - 19) % 40 == 0 {
            total_signal_strength += cpu.signal_strength();
        }
    }
    total_signal_strength
}

fn part2(cpu: &mut Cpu) {
    while !cpu.is_completed() {
        cpu.tick_with_draw();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{
        parse_input, part1, Cpu,
        Instruction::{AddX, NoOp},
    };

    fn test_input_1() -> String {
        "noop
addx 3
addx -5
"
        .to_string()
    }

    fn test_input_2() -> String {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = VecDeque::from([(NoOp, 0), (AddX { v: 3 }, 0), (AddX { v: -5 }, 0)]);
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn signal_strength_test_1() {
        let input = test_input_1();
        let instructions = parse_input(&input);
        let mut cpu = Cpu::new_with_instructions(instructions);
        while !cpu.is_completed() {
            cpu.tick();
        }
        assert_eq!(cpu.signal_strength(), -6);
    }

    #[test]
    fn signal_strength_test_2() {
        let input = test_input_2();
        let instructions = parse_input(&input);
        let mut cpu = Cpu::new_with_instructions(instructions);
        // expected values are in reverse order for easy popping
        let mut expected = vec![3960, 2880, 2940, 1800, 1140, 420];
        while !cpu.is_completed() {
            cpu.tick();
            if (cpu.cycle + 1) % 20 == 0 {
                assert_eq!(Some(cpu.signal_strength()), expected.pop());
                break;
            }
        }
        while !cpu.is_completed() {
            cpu.tick();
            if (cpu.cycle - 19) % 40 == 0 {
                assert_eq!(Some(cpu.signal_strength()), expected.pop());
            }
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_2();
        let instructions = parse_input(&input);
        let mut cpu = Cpu::new_with_instructions(instructions);
        assert_eq!(part1(&mut cpu), 13140);
    }
}
