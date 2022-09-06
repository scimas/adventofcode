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
    let program: HashMap<usize, i64> = load_data("data/Day05_input.txt");
    let mut my_computer = Computer::new();

    my_computer.load_program(program);
    my_computer.initialize();
    my_computer.run()?;
    Ok(())
}

fn part2() -> Result<()> {
    let program: HashMap<usize, i64> = load_data("data/Day05_input.txt");
    let mut my_computer = Computer::new();

    my_computer.load_program(program);
    my_computer.initialize();
    my_computer.run()?;
    Ok(())
}
