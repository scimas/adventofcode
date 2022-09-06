use std::{collections::HashMap, io};

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Clone)]
struct Ram {
    memory: HashMap<usize, i64>,
}

impl Ram {
    fn new() -> Self {
        Ram {
            memory: HashMap::new(),
        }
    }

    fn set_memory(&mut self, memory: HashMap<usize, i64>) {
        self.memory = memory;
    }

    fn set_at(&mut self, at: usize, value: i64) {
        self.memory.insert(at, value);
    }

    fn read_at(&self, at: &usize) -> i64 {
        *self.memory.get(at).unwrap_or(&0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Run,
    AdvanceBy(usize),
    MoveTo(usize),
    Halt,
}

#[derive(Debug, Clone, Copy)]
enum Parameter {
    ReadPosition(usize),
    ReadImmediate(i64),
    WritePosition(i64),
}

impl Parameter {
    fn parse(mode: i64, at: usize, read: bool, ram: &Ram) -> Result<Self, InstructionParseError> {
        match read {
            true => match mode {
                0 => Ok(Self::ReadPosition(ram.read_at(&at) as usize)),
                1 => Ok(Self::ReadImmediate(ram.read_at(&at))),
                _ => Err(InstructionParseError::InvalidParameterMode { mode, position: at }),
            },
            false => match mode {
                0 => Ok(Self::WritePosition(ram.read_at(&at))),
                _ => Err(InstructionParseError::InvalidParameterMode { mode, position: at }),
            },
        }
    }

    fn get(&self, ram: &Ram) -> i64 {
        match *self {
            Self::ReadPosition(at) => ram.read_at(&at),
            Self::ReadImmediate(value) => value,
            Self::WritePosition(at) => at,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Halt, // 99
    Add {
        op1: Parameter,
        op2: Parameter,
        out_pos: Parameter,
    }, // 1
    Multiply {
        op1: Parameter,
        op2: Parameter,
        out_pos: Parameter,
    }, // 2
    StoreInput {
        out_pos: Parameter,
    }, // 3
    GiveOutput {
        read_pos: Parameter,
    }, // 4
    JumpIfTrue {
        op1: Parameter,
        jump_pos: Parameter,
    }, // 5
    JumpIfFalse {
        op1: Parameter,
        jump_pos: Parameter,
    }, // 6
    LessThan {
        op1: Parameter,
        op2: Parameter,
        out_pos: Parameter,
    }, // 7
    Equals {
        op1: Parameter,
        op2: Parameter,
        out_pos: Parameter,
    }, // 8
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("invalid opcode {opcode:?} at position {position:?}")]
    InvalidOpcode { opcode: i64, position: usize },
    #[error("invalid parameter mode {mode:?} at position {position:?}")]
    InvalidParameterMode { mode: i64, position: usize },
}

impl Instruction {
    fn parse(ram: &Ram, at: usize) -> Result<Self, InstructionParseError> {
        let instr = ram.read_at(&at);
        let opcode = instr % 100;
        let instr = instr / 100;
        let param1_mode = instr % 10;
        let instr = instr / 10;
        let param2_mode = instr % 10;
        let param3_mode = instr / 10;

        match opcode {
            99 => Ok(Self::Halt),
            1 => Ok(Self::Add {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                op2: Parameter::parse(param2_mode, at + 2, true, ram)?,
                out_pos: Parameter::parse(param3_mode, at + 3, false, ram)?,
            }),
            2 => Ok(Self::Multiply {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                op2: Parameter::parse(param2_mode, at + 2, true, ram)?,
                out_pos: Parameter::parse(param3_mode, at + 3, false, ram)?,
            }),
            3 => Ok(Self::StoreInput {
                out_pos: Parameter::parse(param1_mode, at + 1, false, ram)?,
            }),
            4 => Ok(Self::GiveOutput {
                read_pos: Parameter::parse(param1_mode, at + 1, true, ram)?,
            }),
            5 => Ok(Self::JumpIfTrue {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                jump_pos: Parameter::parse(param2_mode, at + 2, true, ram)?,
            }),
            6 => Ok(Self::JumpIfFalse {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                jump_pos: Parameter::parse(param2_mode, at + 2, true, ram)?,
            }),
            7 => Ok(Self::LessThan {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                op2: Parameter::parse(param2_mode, at + 2, true, ram)?,
                out_pos: Parameter::parse(param3_mode, at + 3, false, ram)?,
            }),
            8 => Ok(Self::Equals {
                op1: Parameter::parse(param1_mode, at + 1, true, ram)?,
                op2: Parameter::parse(param2_mode, at + 2, true, ram)?,
                out_pos: Parameter::parse(param3_mode, at + 3, false, ram)?,
            }),
            _ => Err(InstructionParseError::InvalidOpcode {
                opcode,
                position: at,
            }),
        }
    }

    fn execute(&self, ram: &mut Ram) -> State {
        match *self {
            Self::Halt => State::Halt,
            Self::Add { op1, op2, out_pos } => {
                let out = op1.get(ram) + op2.get(ram);
                ram.set_at(out_pos.get(ram) as usize, out);
                State::AdvanceBy(4)
            }
            Self::Multiply { op1, op2, out_pos } => {
                let out = op1.get(ram) * op2.get(ram);
                ram.set_at(out_pos.get(ram) as usize, out);
                State::AdvanceBy(4)
            }
            Self::StoreInput { out_pos } => {
                println!("Give input");
                let mut buf = String::new();
                io::stdin()
                    .read_line(&mut buf)
                    .expect("error reading input");
                let value: i64 = buf.trim().parse().expect("input is not an integer");
                ram.set_at(out_pos.get(ram) as usize, value);
                State::AdvanceBy(2)
            }
            Self::GiveOutput { read_pos } => {
                println!("{}", read_pos.get(ram));
                State::AdvanceBy(2)
            }
            Self::JumpIfTrue { op1, jump_pos } => {
                if op1.get(ram) != 0 {
                    State::MoveTo(jump_pos.get(ram) as usize)
                } else {
                    State::AdvanceBy(3)
                }
            }
            Self::JumpIfFalse { op1, jump_pos } => {
                if op1.get(ram) == 0 {
                    State::MoveTo(jump_pos.get(ram) as usize)
                } else {
                    State::AdvanceBy(3)
                }
            }
            Self::LessThan { op1, op2, out_pos } => {
                let at = out_pos.get(ram) as usize;
                if op1.get(ram) < op2.get(ram) {
                    ram.set_at(at, 1);
                } else {
                    ram.set_at(at, 0);
                }
                State::AdvanceBy(4)
            }
            Self::Equals { op1, op2, out_pos } => {
                let at = out_pos.get(ram) as usize;
                if op1.get(ram) == op2.get(ram) {
                    ram.set_at(at, 1);
                } else {
                    ram.set_at(at, 0);
                }
                State::AdvanceBy(4)
            }
        }
    }
}

pub struct Computer {
    ram: Ram,
    state: State,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            ram: Ram::new(),
            state: State::Idle,
            instruction_pointer: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.state = State::Run;
    }

    pub fn load_program(&mut self, program: HashMap<usize, i64>) {
        self.ram.set_memory(program);
        self.instruction_pointer = 0;
    }

    pub fn read_memory(&self, at: &usize) -> i64 {
        self.ram.read_at(at)
    }

    pub fn run(&mut self) -> Result<()> {
        while self.state != State::Halt {
            let instruction = Instruction::parse(&self.ram, self.instruction_pointer)?;
            // println!("{instruction:?}");
            let new_state = instruction.execute(&mut self.ram);
            match new_state {
                State::Halt => {
                    self.instruction_pointer += 1;
                    self.state = State::Halt;
                }
                State::AdvanceBy(by) => {
                    self.instruction_pointer += by;
                    self.state = State::Run;
                }
                State::MoveTo(position) => {
                    self.instruction_pointer = position;
                    self.state = State::Run;
                }
                State::Idle => self.state = State::Idle,
                State::Run => self.state = State::Run,
            }
        }
        Ok(())
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}
