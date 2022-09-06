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

fn next_permutation<T: PartialOrd + Copy>(seq: &Vec<T>) -> Vec<T> {
    let max_idx = seq.len();
    let mut k = max_idx + 1;
    let mut new_seq: Vec<T> = Vec::new();
    for i in 0..max_idx {
        new_seq.push(seq[i]);
    }
    for i in 0..(max_idx - 1) {
        if new_seq[i] < new_seq[i + 1] {
            k = i;
        }
    }
    if k > max_idx {
        return new_seq;
    }
    let mut l = max_idx + 1;
    for j in k..5 {
        if new_seq[k] < new_seq[j] {
            l = j;
        }
    }
    let temp = new_seq[k];
    new_seq[k] = new_seq[l];
    new_seq[l] = temp;
    new_seq[k + 1..].reverse();
    return new_seq;
}

// fn test_perm() {
//     let mut seq: Vec<i64> = vec![0, 1, 2, 3, 4];
//     let mut count = 1;
//     loop {
//         let new_seq = next_permutation(&seq);
//         if new_seq != seq {
//             count += 1;
//             seq = new_seq;
//             println!("{:?}", seq);
//         }
//         else {
//             break;
//         }
//     }
//     println!("{}", count);
// }

fn part1() {
    let program = load_data("data/Day07_input.txt");
    let mut amps: Vec<Computer> = Vec::new();
    for _ in 0..5 {
        amps.push(Computer::new());
    }
    let mut phase_settings: Vec<i64> = vec![0, 1, 2, 3, 4];
    let mut max_signal: i64 = 0;
    for _ in 0..120 {
        for i in 0..5 {
            amps[i].load_program(&program);
            amps[i].initialize();
            amps[i].add_input(phase_settings[i]);
        }
        amps[0].add_input(0);
        for i in 0..5 {
            loop {
                match amps[i].get_state() {
                    State::HALT => break,
                    State::WAIT => break,
                    _ => amps[i].run(),
                }
            }
            if i != 4 {
                let out = amps[i].get_output().unwrap();
                amps[(i + 1) % 5].add_input(out);
            }
        }
        let this_signal = amps[4].get_output().unwrap();
        if this_signal > max_signal {
            max_signal = this_signal;
        }
        phase_settings = next_permutation(&phase_settings);
    }
    println!("{}", max_signal);
}

fn part2() {
    let program = load_data("data/Day07_input.txt");
    let mut amps: Vec<Computer> = Vec::new();
    for _ in 0..5 {
        amps.push(Computer::new());
    }
    let mut phase_settings: Vec<i64> = vec![5, 6, 7, 8, 9];
    let mut max_signal: i64 = 0;
    for _ in 0..120 {
        let mut this_signal: i64 = 0;
        for i in 0..5 {
            amps[i].load_program(&program);
            amps[i].initialize();
            amps[i].add_input(phase_settings[i]);
        }
        amps[0].add_input(0);
        let mut done = false;
        loop {
            for i in 0..5 {
                loop {
                    match amps[i].get_state() {
                        State::HALT => {
                            if i == 4 {
                                done = true;
                            }
                            break;
                        }
                        State::WAIT => {
                            this_signal = amps[i].get_output().unwrap();
                            amps[(i + 1) % 5].add_input(this_signal);
                            amps[i].initialize();
                            break;
                        }
                        _ => amps[i].run(),
                    }
                }
            }
            if done {
                break;
            }
        }
        if this_signal > max_signal {
            max_signal = this_signal;
        }
        phase_settings = next_permutation(&phase_settings);
    }
    println!("{}", max_signal);
}
