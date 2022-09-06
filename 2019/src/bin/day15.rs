use std::fs;
use std::collections::HashMap;

use adventofcode2019::intcode;
use intcode::{Computer, State};

fn main() {
    part1();
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new<T: Into<i64>>(x: T, y: T) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

fn load_data(fname: &str) -> HashMap<usize, i64> {
    let program_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    let program_text = program_text.trim();
    let program_vec: Vec<i64> = program_text.split(",").map(|x| x.parse::<i64>().expect("Couldn't parse instruction!")).collect();
    let program: HashMap<usize, i64> = (0..program_vec.len()).zip(program_vec.iter()).map(|(k, v)| (k, *v)).collect();
    return program;
}

fn part1() {
    let program = load_data("data/Day15_input.txt");
    let mut my_computer = Computer::new();

    let movex: HashMap<&str, i64> = [("North", 0i64), ("South", 0), ("West", -1), ("East", 1)].iter().cloned().collect();
    let movey: HashMap<&str, i64> = [("North", 1i64), ("South", -1), ("West", 0), ("East", 0)].iter().cloned().collect();
    let inputs: Vec<&str> = vec!["North", "South", "West", "East"];
    let mut grid: HashMap<Point, i64> = HashMap::new();

    my_computer.load_program(&program);
    my_computer.initialize();

    let mut attempt: usize = 0;
    let mut current_pos = Point::new(0, 0);
    let mut attempt_pos = Point::new(0, 0);
    grid.insert(current_pos, 1);
    my_computer.add_input(attempt as i64 + 1);

    let mut tried_directions = 0;
    let mut move_possible = true;

    loop {
        match move_possible {
            false => break,
            true => {
                match my_computer.get_state() {
                    State::WAIT => {
                        let out = grid.entry(attempt_pos).or_insert(my_computer.get_output().unwrap());
                        if out == &0 {
                            tried_directions += 1;
                            attempt = (attempt + 1) % 4;
                            my_computer.add_input(attempt as i64 + 1);
                        }
                        else if out == &2 {
                            break;
                        }
                        else {
                            tried_directions = 0;
                            current_pos = attempt_pos;
                        }
                        if tried_directions == 4 {
                            move_possible = false;
                        }
                        attempt_pos = Point::new(
                            current_pos.x + movex[inputs[attempt]],
                            current_pos.y + movey[inputs[attempt]]
                        );
                    },
                    State::HALT => {
                        move_possible = false;
                    },
                    _ => {
                        my_computer.run();
                    }
                }
            }
        }
    }
}
