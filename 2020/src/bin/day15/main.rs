use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {}

fn str_to_map(s: &str) -> HashMap<usize, usize> {
    s.trim()
        .split(',')
        .enumerate()
        .map(|(idx, i)| (i.parse().unwrap(), idx))
        .collect()
}

fn load_data() -> (HashMap<usize, usize>, usize) {
    let s = read_to_string("res/input15.txt").expect("Couldn't read day 15 input");
    let nums = str_to_map(&s);
    let n = *nums.iter().max_by_key(|(_, lp)| *lp).unwrap().0;
    (nums, n)
}

fn play_game(nums: &mut HashMap<usize, usize>, last_num: usize, turns: usize) -> usize {
    let mut n = last_num;
    for i in nums.len()..turns {
        let pos = nums.entry(n).or_insert(i - 1);
        n = i - 1 - *pos;
        *pos = i - 1;
    }
    n
}

pub fn part1() -> usize {
    let (mut nums, n) = load_data();
    play_game(&mut nums, n, 2020)
}

pub fn part2() -> usize {
    let (mut nums, n) = load_data();
    play_game(&mut nums, n, 30_000_000)
}

#[test]
fn string_map_conversion_test() {
    let s = "0,3,6";
    let mut m: HashMap<usize, usize> = HashMap::new();
    m.insert(0, 0);
    m.insert(3, 1);
    m.insert(6, 2);
    assert_eq!(str_to_map(s), m);
}

#[test]
fn play_test1() {
    let s = "0,3,6";
    let mut nums = str_to_map(s);
    let last_num = *nums.iter().max_by_key(|(_, lp)| *lp).unwrap().0;
    assert_eq!(play_game(&mut nums, last_num, 2020), 436);
}

// #[test]
// fn play_test2() {
//     let s = "0,3,6";
//     let mut nums = str_to_map(s);
//     let last_num = *nums.iter().max_by_key(|(_, lp)| *lp).unwrap().0;
//     assert_eq!(play_game(&mut nums, last_num, 30_000_000), 175594);
// }
