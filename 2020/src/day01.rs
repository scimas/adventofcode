use std::fs::File;
use std::io::{BufRead, BufReader};

fn pair_match(inp: &[u32], sum: u32) -> Option<(u32, u32)> {
    let nums = inp.len();
    for i in 0..nums {
        if inp[i] > 2020 {
            continue;
        }
        for j in (i + 1)..nums {
            if inp[i] + inp[j] == sum {
                return Some((inp[i], inp[j]));
            }
        }
    }
    None
}

pub fn part1() -> Option<u32> {
    let f = File::open("res/input01.txt").expect("Couldn't read day 1 input");
    let reader = BufReader::new(f);
    let nums: Vec<u32> = reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 1 input")
                .parse()
                .expect("Couldn't parse day 1 input")
        })
        .collect();
    let matched_pair = pair_match(&nums, 2020);
    matched_pair.map(|(n1, n2)| n1 * n2)
}

fn three_match(inp: &[u32], sum: u32) -> Option<(u32, u32, u32)> {
    let nums = inp.len();
    for i in 0..nums {
        if inp[i] > 2020 {
            continue;
        }
        for j in i..nums {
            for k in j..nums {
                if inp[i] + inp[j] + inp[k] == sum {
                    return Some((inp[i], inp[j], inp[k]));
                }
            }
        }
    }
    None
}

pub fn part2() -> Option<u32> {
    let f = File::open("res/input01.txt").expect("Couldn't read day 1 input");
    let reader = BufReader::new(f);
    let nums: Vec<u32> = reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 1 input")
                .parse()
                .expect("Couldn't parse day 1 input")
        })
        .collect();
    let matched_threes = three_match(&nums, 2020);
    matched_threes.map(|(n1, n2, n3)| n1 * n2 * n3)
}

#[test]
fn pair_matching() {
    let nums = vec![23, 374, 98472, 1646, 567, 394];
    let result = pair_match(&nums, 2020);
    assert_eq!(result, Some((374, 1646)));
}

#[test]
fn three_matching() {
    let nums = vec![23, 374, 98472, 1637, 567, 394, 9, 749];
    let result = three_match(&nums, 2020);
    assert_eq!(result, Some((374, 1637, 9)));
}
