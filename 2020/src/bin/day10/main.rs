use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {}

fn load_joltages() -> Vec<u32> {
    let f = File::open("res/input10.txt").expect("Couldn't open input for day 10");
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| line.expect("Couldn't read line from day 10 input"))
        .filter(|line| line != &"\n".to_string())
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1() -> usize {
    let mut joltages = load_joltages();
    joltages.sort_unstable();
    let differences: Vec<u32> = joltages
        .iter()
        .scan(0, |state, &x| {
            let out = x - *state;
            *state = x;
            Some(out)
        })
        .collect();
    let ones = differences.iter().filter(|&x| x == &1).count();
    let threes = differences.iter().filter(|&x| x == &3).count() + 1;
    ones * threes
}

fn num_ways_to_dest(dest_idx: usize, joltages: &[u32]) -> usize {
    fn _num_ways(dest_idx: usize, joltages: &[u32], ways: &mut HashMap<usize, usize>) -> usize {
        if let Some(val) = ways.get(&dest_idx) {
            return *val;
        }
        if dest_idx == 0 {
            return 1;
        }
        let mut num_ways: usize = 0;
        for i in (dest_idx - 3.min(dest_idx))..dest_idx {
            if joltages[dest_idx] - joltages[i] <= 3 {
                num_ways += _num_ways(i, joltages, ways);
            }
        }
        ways.insert(dest_idx, num_ways);
        ways[&dest_idx]
    }
    let mut ways: HashMap<usize, usize> = HashMap::new();
    _num_ways(dest_idx, joltages, &mut ways)
}

pub fn part2() -> usize {
    let mut joltages = load_joltages();
    let out_joltage = *joltages.iter().max().unwrap() + 3;
    joltages.push(0);
    joltages.push(out_joltage);
    joltages.sort_unstable();
    num_ways_to_dest(joltages.len() - 1, &joltages)
}

#[test]
fn path_counts() {
    let joltages = vec![0, 1, 2, 3, 4];
    assert_eq!(num_ways_to_dest(4, &joltages), 7);
    let joltages = vec![0, 2, 3, 4];
    assert_eq!(num_ways_to_dest(3, &joltages), 3);
    let joltages = vec![0, 2, 4];
    assert_eq!(num_ways_to_dest(2, &joltages), 1);
    let joltages = vec![0, 4];
    assert_eq!(num_ways_to_dest(1, &joltages), 0);
}
