use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

use crate::{InputParseError, Result};

enum Input {
    Signal(u16),
    Wire(String),
}

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u16>() {
            return Ok(Self::Signal(n));
        }
        Ok(Self::Wire(s.to_owned()))
    }
}

impl Input {
    fn value(
        &self,
        gate_lookup: &HashMap<String, Gate>,
        value_lookup: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Input::Signal(v) => *v,
            Self::Wire(s) => {
                if value_lookup.contains_key(s) {
                    value_lookup[s]
                } else {
                    let v = gate_lookup[s].value(gate_lookup, value_lookup);
                    value_lookup.insert(s.to_owned(), v);
                    v
                }
            }
        }
    }
}

enum Gate {
    And { x: Input, y: Input },
    Or { x: Input, y: Input },
    Not { x: Input },
    Lshift { x: Input, shift: Input },
    Rshift { x: Input, shift: Input },
    I { x: Input },
}

impl FromStr for Gate {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut inputs = s.split(' ');
        let inp1 = inputs
            .next()
            .ok_or_else(|| InputParseError::new("Unexpectedly empty input"))?;
        if inp1 == "NOT" {
            let notted_input = inputs
                .next()
                .ok_or_else(|| InputParseError::new("Nothing to NOT"))?
                .parse()?;
            return Ok(Self::Not { x: notted_input });
        }
        let x = inp1.parse()?;
        let opt_gate = inputs.next();
        let gate;
        match opt_gate {
            None => return Ok(Self::I { x }),
            Some(g) => gate = g,
        }
        let y = inputs
            .next()
            .ok_or_else(|| InputParseError::new("Second operand missing"))?;
        match gate {
            "AND" => Ok(Self::And { x, y: y.parse()? }),
            "OR" => Ok(Self::Or { x, y: y.parse()? }),
            "LSHIFT" => {
                let shift = y
                    .parse()
                    .map_err(|_| InputParseError::new("Couldn't parse shift"))?;
                Ok(Self::Lshift { x, shift })
            }
            "RSHIFT" => {
                let shift = y
                    .parse()
                    .map_err(|_| InputParseError::new("Couldn't parse shift"))?;
                Ok(Self::Rshift { x, shift })
            }
            _ => Err(InputParseError::new("Unknown gate").into()),
        }
    }
}

impl Gate {
    fn value(
        &self,
        gate_lookup: &HashMap<String, Gate>,
        value_lookup: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Self::Not { x } => !x.value(gate_lookup, value_lookup),
            Self::And { x, y } => {
                x.value(gate_lookup, value_lookup) & y.value(gate_lookup, value_lookup)
            }
            Self::Or { x, y } => {
                x.value(gate_lookup, value_lookup) | y.value(gate_lookup, value_lookup)
            }
            Self::Lshift { x, shift } => {
                x.value(gate_lookup, value_lookup) << shift.value(gate_lookup, value_lookup)
            }
            Self::Rshift { x, shift } => {
                x.value(gate_lookup, value_lookup) >> shift.value(gate_lookup, value_lookup)
            }
            Self::I { x } => x.value(gate_lookup, value_lookup),
        }
    }
}

fn part1(gate_lookup: &HashMap<String, Gate>) -> u16 {
    let mut value_lookup: HashMap<String, u16> = HashMap::new();
    gate_lookup["a"].value(gate_lookup, &mut value_lookup)
}

fn part2(gate_lookup: &HashMap<String, Gate>) -> u16 {
    let mut value_lookup: HashMap<String, u16> = HashMap::new();
    gate_lookup["a"].value(gate_lookup, &mut value_lookup)
}

pub fn main() -> Result<()> {
    let fl = File::open("res/input07")?;
    let reader = BufReader::new(fl);

    let mut gate_lookup: HashMap<String, Gate> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let mut sides = line.split(" -> ");
        let lhs = sides
            .next()
            .ok_or_else(|| InputParseError::new("Unexpected end of input"))?;
        let rhs = sides
            .next()
            .ok_or_else(|| InputParseError::new("Unexpected end of input"))?;

        gate_lookup.insert(rhs.to_owned(), lhs.parse()?);
    }

    println!("Day 07");
    let p1 = part1(&gate_lookup);
    println!("Part 1: {}", p1);
    gate_lookup.insert(
        "b".to_owned(),
        Gate::I {
            x: Input::Signal(p1),
        },
    );
    println!("Part 2: {}", part2(&gate_lookup));
    Ok(())
}
