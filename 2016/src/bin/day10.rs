use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Destination {
    Output(usize),
    Bot(usize),
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    chip1: Option<u8>,
    chip2: Option<u8>,
    low: Destination,
    high: Destination,
}

type BotOutput = (Destination, u8);

#[derive(Debug, Error)]
enum BotError {
    #[error("attempt to add chip to a full bot")]
    FullCapacity,
}

impl Bot {
    const fn is_full(&self) -> bool {
        self.chip1.is_some() && self.chip2.is_some()
    }

    fn add_chip(&mut self, chip: u8) -> Result<(), BotError> {
        if self.chip1.is_none() {
            self.chip1 = Some(chip);
        } else if self.chip2.is_none() {
            self.chip2 = Some(chip);
        } else {
            return Err(BotError::FullCapacity);
        }
        Ok(())
    }

    fn work(&mut self) -> Option<(BotOutput, BotOutput)> {
        match (self.chip1, self.chip2) {
            (None, _) => None,
            (_, None) => None,
            (Some(c1), Some(c2)) => {
                let (low, high) = match c1.cmp(&c2) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => (c1, c2),
                    std::cmp::Ordering::Greater => (c2, c1),
                };
                self.chip1 = None;
                self.chip2 = None;
                Some(((self.low, low), (self.high, high)))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<u8>>,
}

impl Factory {
    fn step(&mut self) -> usize {
        let mut new_bots = self.bots.clone();
        let mut new_outputs = self.outputs.clone();
        let mut worked = 0;
        for i in 0..self.bots.len() {
            match self.bots.get_mut(&i).unwrap().work() {
                Some((out1, out2)) => {
                    match out1 {
                        (Destination::Bot(d1), c1) => {
                            new_bots.entry(d1).and_modify(|bot| {
                                bot.add_chip(c1).expect("bot should not be full")
                            });
                        }
                        (Destination::Output(d1), c1) => {
                            new_outputs.entry(d1).and_modify(|output| output.push(c1));
                        }
                    }
                    match out2 {
                        (Destination::Bot(d2), c2) => {
                            new_bots.entry(d2).and_modify(|bot| {
                                bot.add_chip(c2).expect("bot should not be full")
                            });
                        }
                        (Destination::Output(d2), c2) => {
                            new_outputs.entry(d2).and_modify(|output| output.push(c2));
                        }
                    }
                    new_bots.insert(i, self.bots[&i]);
                    worked += 1;
                }
                _ => (),
            }
        }
        self.bots = new_bots;
        self.outputs = new_outputs;
        worked
    }
}

fn main() {
    let fl = File::open("res/day10/input").expect("couldn't open the file");
    let reader = BufReader::new(fl);
    let factory = parse_input(reader);
    println!("Day 10");
    println!("Part 1: {}", part1(factory.clone(), 61, 17));
    let finished_factory = part2(factory);
    println!("Part 2: {:?}", finished_factory.outputs);
}

fn parse_input<T: BufRead>(reader: T) -> Factory {
    let input_pattern = Regex::new(r"^value (?P<value>\d+) goes to bot (?P<bot>\d+)$").unwrap();
    let instruction_pattern = Regex::new(r"^bot (?P<bot>\d+) gives low to (?P<low>[a-z]+) (?P<low_n>\d+) and high to (?P<high>[a-z]+) (?P<high_n>\d+)$").unwrap();

    let mut outputs: HashMap<usize, Vec<u8>> = HashMap::new();
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    for maybe_line in reader.lines() {
        let line = maybe_line.expect("couldn't read line from file");
        if let Some(caps) = input_pattern.captures(&line) {
            let bot_num = caps.name("bot").unwrap().as_str().parse().unwrap();
            let chip_value = caps.name("value").unwrap().as_str().parse().unwrap();
            bots.entry(bot_num).and_modify(|bot| {
                if bot.chip1.is_none() {
                    bot.chip1 = Some(chip_value);
                } else if bot.chip2.is_none() {
                    bot.chip2 = Some(chip_value);
                } else {
                    panic!("invalid initialization found, both slots of bot {bot_num} already filled");
                }
            }).or_insert(Bot { chip1: Some(chip_value), chip2: None, low: Destination::Output(0), high: Destination::Output(0) });
        } else if let Some(caps) = instruction_pattern.captures(&line) {
            let bot_num = caps.name("bot").unwrap().as_str().parse().unwrap();
            let low = caps.name("low").unwrap().as_str();
            let low_num = caps.name("low_n").unwrap().as_str().parse().unwrap();
            let high = caps.name("high").unwrap().as_str();
            let high_num = caps.name("high_n").unwrap().as_str().parse().unwrap();
            let low_dest = match low {
                "output" => {
                    outputs.entry(low_num).or_default();
                    Destination::Output(low_num)
                }
                "bot" => {
                    bots.entry(low_num).or_insert(Bot {
                        chip1: None,
                        chip2: None,
                        low: Destination::Output(0),
                        high: Destination::Output(0),
                    });
                    Destination::Bot(low_num)
                }
                dest => panic!("invalid destination found {dest}"),
            };
            let high_dest = match high {
                "output" => {
                    outputs.entry(high_num).or_default();
                    Destination::Output(high_num)
                }
                "bot" => {
                    bots.entry(high_num).or_insert(Bot {
                        chip1: None,
                        chip2: None,
                        low: Destination::Output(0),
                        high: Destination::Output(0),
                    });
                    Destination::Bot(high_num)
                }
                dest => panic!("invalid destination found {dest}"),
            };
            bots.entry(bot_num)
                .and_modify(|bot| {
                    bot.low = low_dest;
                    bot.high = high_dest;
                })
                .or_insert(Bot {
                    chip1: None,
                    chip2: None,
                    low: low_dest,
                    high: high_dest,
                });
        }
    }
    Factory { bots, outputs }
}

fn part1(mut factory: Factory, chip1: u8, chip2: u8) -> usize {
    loop {
        for (idx, bot) in &factory.bots {
            if bot.is_full() {
                if (bot.chip1 == Some(chip1) && bot.chip2 == Some(chip2))
                    || (bot.chip1 == Some(chip2) && bot.chip2 == Some(chip1))
                {
                    return *idx;
                }
            }
        }
        factory.step();
    }
}

fn part2(mut factory: Factory) -> Factory {
    while factory.step() != 0 {}
    factory
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{parse_input, part1, Factory};

    fn setup_factory() -> Factory {
        let instructions = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";
        let reader = Cursor::new(instructions);
        parse_input(reader)
    }

    #[test]
    fn test_case_1() {
        let mut factory = setup_factory();
        factory.step();
        assert_eq!(factory.bots[&2].chip1, None);
        assert_eq!(factory.bots[&2].chip2, None);
        assert_eq!(factory.bots[&1].chip1, Some(3));
        assert_eq!(factory.bots[&1].chip2, Some(2));
        assert_eq!(factory.bots[&0].chip1, Some(5));
        assert_eq!(factory.bots[&0].chip2, None);
    }

    #[test]
    fn test_case_2() {
        let factory = setup_factory();
        assert_eq!(part1(factory, 5, 2), 2);
    }
}
