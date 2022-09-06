use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type ComputerOp = fn(i32, &mut i32) -> i32;

fn acc(inp: i32, accumulator: &mut i32) -> i32 {
    *accumulator += inp;
    1
}

fn jmp(inp: i32, _accumulator: &mut i32) -> i32 {
    inp
}

fn nop(_inp: i32, _accumulator: &mut i32) -> i32 {
    1
}

fn load_instructions() -> Vec<(ComputerOp, i32)> {
    let f = File::open("res/input08.txt").expect("Couldn't open day 8 input");
    let reader = BufReader::new(f);
    let mut instructions: Vec<(ComputerOp, i32)> = Vec::new();
    for line in reader.lines() {
        let s = line.expect("Couldn't read line from day 8 input");
        let mut instr_off_split = s.split(' ');
        let func = match instr_off_split.next() {
            None => unreachable!(),
            Some(s) => {
                if s == "acc" {
                    acc
                } else if s == "jmp" {
                    jmp
                } else if s == "nop" {
                    nop
                } else {
                    unreachable!()
                }
            }
        };
        let inp: i32 = instr_off_split.next().unwrap().parse().unwrap();
        instructions.push((func, inp));
    }
    instructions
}

fn execute_instructions(instructions: &[(ComputerOp, i32)]) -> (i32, usize) {
    let mut executed: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut idx: usize = 0;
    loop {
        let already_done = executed.insert(idx);
        if !already_done {
            break (accumulator, idx);
        }
        let d = instructions[idx].0(instructions[idx].1, &mut accumulator);
        if d < 0 {
            idx -= d.abs() as usize;
        } else {
            idx += d as usize;
        }
        if idx >= instructions.len() {
            break (accumulator, idx);
        }
    }
}

pub fn part1() -> (i32, usize) {
    let instructions = load_instructions();
    execute_instructions(&instructions)
}

fn get_trace() -> Vec<usize> {
    let instructions = load_instructions();
    let mut executed: HashSet<usize> = HashSet::new();
    let mut trace: Vec<usize> = Vec::new();
    let mut accumulator: i32 = 0;
    let mut idx: usize = 0;
    loop {
        let already_done = executed.insert(idx);
        trace.push(idx);
        if !already_done {
            break trace;
        }
        let d = instructions[idx].0(instructions[idx].1, &mut accumulator);
        if d < 0 {
            idx -= d.abs() as usize;
        } else {
            idx += d as usize;
        }
        if idx == instructions.len() {
            println!("Finished exec!! {}", accumulator);
            break trace;
        }
    }
}

pub fn part2() -> i32 {
    let trace = get_trace();
    let mut instructions = load_instructions();
    let mut acc: i32 = 0;
    let mut last_idx: usize;
    for idx in trace {
        let original_instr = instructions[idx].0;
        if original_instr as usize == nop as usize {
            instructions[idx].0 = jmp;
        } else if original_instr as usize == jmp as usize {
            instructions[idx].0 = nop;
        } else {
            continue;
        }
        let result = execute_instructions(&instructions);
        last_idx = result.1;
        if last_idx >= instructions.len() {
            acc = result.0;
            break;
        }
        instructions[idx].0 = original_instr;
    }
    acc
}
