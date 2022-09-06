use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

fn load_data() -> Vec<u64> {
    let f = File::open("res/input09.txt").expect("Couldn't open input for day 9");
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 9 input")
                .parse()
                .unwrap()
        })
        .collect()
}

fn sum_of_nums<T: Add<Output = T> + PartialOrd + Copy>(s: T, nums: &[T]) -> bool {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == s && nums[i] != nums[j] {
                return true;
            }
        }
    }
    false
}

pub fn part1() -> Option<u64> {
    let nums = load_data();
    let mut cur_min = nums[0..25].iter().min().unwrap();
    let mut cur_max = nums[0..25].iter().max().unwrap();
    for i in 25..nums.len() {
        if (nums[i] <= 2 * cur_min)
            || (2 * cur_max <= nums[i])
            || !sum_of_nums(nums[i], &nums[i - 25..i])
        {
            return Some(nums[i]);
        }
        if cur_min == &nums[i - 25] {
            cur_min = nums[i - 24..i + 1].iter().min().unwrap();
        }
        cur_max = cur_max.max(&nums[i]);
    }
    None
}

pub fn part2() -> Option<u64> {
    let nums: Vec<u64> = load_data();
    let target = nums[520];
    let nums: Vec<u64> = nums[..520].iter().copied().collect();
    let partial_sums: Vec<u64> = nums
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    for i in 0..(partial_sums.len() - 1) {
        let sub_el = if i == 0 { 0 } else { partial_sums[i - 1] };
        for j in (i + 1)..(partial_sums.len() - 1) {
            if partial_sums[j] - sub_el == target {
                let min = nums[i..(j + 1)].iter().min().unwrap();
                let max = nums[i..(j + 1)].iter().max().unwrap();
                return Some(min + max);
            }
        }
    }
    None
}
