use adventofcode2020::*;
use std::thread;

fn main() {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        match day01::part1() {
            None => println!("Couldn't solve day 1 part 1"),
            Some(n) => println!("Day 1 part 1 multiplication of entries: {}", n),
        };

        match day01::part2() {
            None => println!("Couldn't solve day 1 part 2"),
            Some(n) => println!("Day 1 part 2 multiplication of entries: {}", n),
        };
    }));

    handles.push(thread::spawn(|| {
        println!("Day 2 part 1 valid password count: {}", day02::part1());
        println!("Day 2 part 2 valid password count: {}", day02::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 3 part 1 tree count: {}", day03::part1());
        println!("Day 3 part 2 tree product: {}", day03::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 4 part 1 valid passports: {}", day04::part1());
        println!("Day 4 part 2 valid passports: {}", day04::part2());
    }));
    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        println!("Day 5 part 1 highest seat ID: {}", day05::part1());
        println!("Day 5 part 2 my seat ID: {}", day05::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 6 part 1 answer total: {}", day06::part1());
        println!("Day 6 part 2 answer total: {}", day06::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 7 part 1 bag count: {}", day07::part1());
        println!("Day 7 part 2 bag count: {}", day07::part2());
    }));

    handles.push(thread::spawn(|| {
        let (acc, loopidx) = day08::part1();
        println!("Day 8 part 1 accumulator {} and loop {}", acc, loopidx);
        println!("Day 8 part 2 non loopy accumulator: {}", day08::part2());
    }));
    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        println!("Day 9 part 1 pattern breaker: {}", day09::part1().unwrap());
        println!("Day 9 part 2 weakness: {}", day09::part2().unwrap());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 10 part 1 distribution product: {}", day10::part1());
        println!("Day 10 part 2 combinations: {}", day10::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 11 part 1 occupied seats: {}", day11::part1());
        println!("Day 11 part 2 occupied seats: {}", day11::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 12 part 1 Manhattan distance: {}", day12::part1());
        println!("Day 12 part 2 Manhattan distance: {}", day12::part2());
    }));
    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        println!("Day 13 part 1 bus id * wait time: {}", day13::part1());
        println!("Day 13 part 2 earliest timestamp: {}", day13::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 14 part 1 memory sum: {}", day14::part1());
        println!("Day 14 part 2 memory sum: {}", day14::part2());
    }));

    // handles.push(thread::spawn(|| {
    //     println!("Day 15 part 1 2020th num: {}", day15::part1());
    //     println!("Day 15 part 2 30,000,000th num: {}", day15::part2());
    // }));

    handles.push(thread::spawn(|| {
        println!(
            "Day 16 part 1 ticket scanning error rate: {}",
            day16::part1()
        );
        println!("Day 16 part 2 my ticket's value: {}", day16::part2());
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        println!("Day 17 part 1 active cubes: {}", day17::part1());
        println!("Day 17 part 2 active hypercubes: {}", day17::part2());
    }));

    handles.push(thread::spawn(|| {
        println!("Day 18 part 1 sum of expressions: {}", day18::part1());
        println!("Day 18 part 2 sum of expressions: {}", day18::part2());
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(|| {
        println!("Day 19 part 1 valid messages: {}", day19::part1());
        println!("Day 19 part 2 valid messages: {}", day19::part2());
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}
