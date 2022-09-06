use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(mut possible_triangles: Vec<u64>) -> usize {
    let mut count = 0;
    for triple in possible_triangles.chunks_mut(3) {
        triple.sort_unstable();
        if triple[2] < triple[0] + triple[1] {
            count += 1;
        }
    }
    count
}

fn part2(possible_triangles: &[u64]) -> usize {
    let mut count = 0;
    for i in 0..(possible_triangles.len() / 9) {
        for j in 0..3 {
            let idx = i * 9 + j;
            let mut triple = vec![
                possible_triangles[idx],
                possible_triangles[idx + 3],
                possible_triangles[idx + 6],
            ];
            triple.sort_unstable();
            if triple[2] < triple[0] + triple[1] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let fl = File::open("res/day03/input").expect("couldn't open file");
    let reader = BufReader::new(fl);
    let numbers: Vec<u64> = reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line from file");
            line.split_whitespace()
                .map(|s| {
                    s.parse()
                        .expect("couldn't interpret string from file as a u64")
                })
                .collect::<Vec<u64>>()
        })
        .flatten()
        .collect();
    println!("Day 03");
    println!("Part 1: {}", part1(numbers.clone()));
    println!("Part 2: {}", part2(&numbers));
}
