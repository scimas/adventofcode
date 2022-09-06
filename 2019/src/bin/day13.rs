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
    let program = load_data("data/Day13_input.txt");
    let mut screen: HashMap<(usize, usize), u32> = HashMap::new();

    let mut arcade = Computer::new();
    arcade.load_program(&program);
    arcade.initialize();
    let mut out_state: char = 'x';
    let mut x: usize = 0;
    let mut y: usize = 0;

    loop {
        match arcade.get_state() {
            State::HALT => break,
            State::WAIT => {
                if out_state == 'x' {
                    x = arcade.get_output().unwrap() as usize;
                    out_state = 'y';
                } else if out_state == 'y' {
                    y = arcade.get_output().unwrap() as usize;
                    out_state = 'p'
                } else if out_state == 'p' {
                    screen.insert((x, y), arcade.get_output().unwrap() as u32);
                    out_state = 'x'
                }
                arcade.initialize();
            }
            _ => arcade.run(),
        }
    }
    let blocks = screen
        .values()
        .fold(0, |acc, &p| if p == 2 { acc + 1 } else { acc });
    println!("{}", blocks);
}

fn part2() {
    let mut program = load_data("data/Day13_input.txt");
    program.insert(0, 2);
    let mut screen: HashMap<(i64, i64), u32> = HashMap::new();

    let mut arcade = Computer::new();
    arcade.load_program(&program);
    arcade.initialize();
    let mut score: i64 = -1;
    let mut out_state: char = 'x';
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut ball_pos_x: i64 = 0;
    let mut paddle_pos_x: i64 = 0;

    loop {
        match arcade.get_state() {
            State::HALT => break,
            State::WAIT => {
                if out_state == 'x' {
                    x = arcade.get_output().unwrap();
                    out_state = 'y';
                } else if out_state == 'y' {
                    y = arcade.get_output().unwrap();
                    if x == -1 && y == 0 {
                        out_state = 's';
                    } else {
                        out_state = 'p';
                    }
                } else if out_state == 'p' {
                    out_state = 'x';
                    screen.insert((x, y), arcade.get_output().unwrap() as u32);
                    for (&k, &p) in screen.iter() {
                        if p == 4 {
                            ball_pos_x = k.0;
                        } else if p == 3 {
                            paddle_pos_x = k.0;
                        }
                    }
                } else if out_state == 's' {
                    out_state = 'x';
                    score = arcade.get_output().unwrap();
                    let blocks = screen
                        .values()
                        .fold(0, |acc, &p| if p == 2 { acc + 1 } else { acc });
                    if blocks == 0 {
                        break;
                    }
                    // arcade.add_input((ball_pos_x - paddle_pos_x).signum());
                }
                arcade.initialize();
            }
            _ => {
                let mem_add = arcade.get_instruction_pointer();
                if arcade.read_memory(&mem_add).unwrap() == &3 {
                    arcade.add_input((ball_pos_x - paddle_pos_x).signum());
                }
                arcade.run();
            }
        }
    }
    let blocks = screen
        .values()
        .fold(0, |acc, &p| if p == 2 { acc + 1 } else { acc });
    println!("{}, {}", blocks, score);
}
