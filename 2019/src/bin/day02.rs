use std::collections::HashMap;
use std::fs;

use anyhow::Result;

use adventofcode2019::intcode;
use intcode::Computer;

fn main() -> Result<()> {
    part1()?;
    part2()
}

fn load_data(fname: &str) -> HashMap<usize, i64> {
    let program_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    let program_text = program_text.trim();
    let program: HashMap<usize, i64> = program_text
        .split(',')
        .enumerate()
        .map(|(idx, x)| (idx, x.parse::<i64>().expect("Couldn't parse instruction!")))
        .collect();
    program
}

fn part1() -> Result<()> {
    let mut program: HashMap<usize, i64> = load_data("data/Day02_input.txt");
    program.insert(1, 12);
    program.insert(2, 2);

    let mut my_computer = Computer::new();
    my_computer.load_program(program);
    my_computer.initialize();
    my_computer.run()?;

    println!("{}", my_computer.read_memory(&0));
    Ok(())
}

fn part2() -> Result<()> {
    let program: HashMap<usize, i64> = load_data("data/Day02_input.txt");
    let mut my_computer = Computer::new();
    let mut found: bool = false;
    let looking_for: i64 = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut load_copy: HashMap<usize, i64> = program.clone();
            load_copy.insert(1, noun);
            load_copy.insert(2, verb);
            my_computer.load_program(load_copy);
            my_computer.initialize();
            my_computer.run()?;
            if my_computer.read_memory(&0) == looking_for {
                println!("{}{}", noun, verb);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    Ok(())
}
