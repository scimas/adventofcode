use std::collections::HashMap;
use std::fs;

use adventofcode2019::intcode;
use intcode::{Computer, State};

fn main() {
    part1();
    part2();
}

fn load_data(fname: &str) -> HashMap<usize, i64> {
    let program_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    let program_text = program_text.trim();
    let program_vec: Vec<i64> = program_text
        .split(",")
        .map(|x| x.parse::<i64>().expect("Couldn't parse instruction!"))
        .collect();
    let program: HashMap<usize, i64> = (0..program_vec.len())
        .zip(program_vec.iter())
        .map(|(k, v)| (k, *v))
        .collect();
    return program;
}

fn part1() {
    let program: HashMap<usize, i64> = load_data("data/Day09_input.txt");
    let mut my_computer = Computer::new();

    my_computer.load_program(&program);
    my_computer.add_input(1);
    my_computer.initialize();
    loop {
        match my_computer.get_state() {
            State::HALT => break,
            _ => my_computer.run(),
        }
    }
    println!("{}", my_computer.get_output().expect(""));
}

fn part2() {
    let program: HashMap<usize, i64> = load_data("data/Day09_input.txt");
    let mut my_computer = Computer::new();

    my_computer.load_program(&program);
    my_computer.add_input(2);
    my_computer.initialize();
    loop {
        match my_computer.get_state() {
            State::HALT => break,
            _ => my_computer.run(),
        }
    }
    println!("{}", my_computer.get_output().expect(""));
}
