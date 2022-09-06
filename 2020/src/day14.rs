use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_data() -> (Vec<(usize, Vec<(usize, bool)>)>, Vec<(usize, Vec<bool>)>) {
    let f = File::open("res/input14.txt").expect("Couldn't open day 14 input");
    let reader = BufReader::new(f);

    let num_pat = Regex::new(r"\d+").unwrap();
    let mut num_instr: usize = 0;
    let mut masks: Vec<(usize, Vec<(usize, bool)>)> = Vec::new();
    let mut instructions: Vec<(usize, Vec<bool>)> = Vec::new();
    let mut i: usize = 0;
    for line in reader.lines() {
        let s = line.expect("Couldn't read line from day 14 input");
        if &s[..4] == "mask" {
            if i > 0 {
                masks[i - 1].0 = num_instr;
            }
            let mask: Vec<(usize, bool)> = s[7..]
                .char_indices()
                .filter(|(_, ch)| *ch != 'X')
                .map(|(idx, ch)| (idx, ch == '1'))
                .collect();
            masks.push((0, mask));
            i += 1;
            num_instr = 0;
            continue;
        }
        num_instr += 1;
        let mut mats = num_pat.find_iter(&s);
        let addr: usize = mats.next().unwrap().as_str().parse().unwrap();
        let num: u64 = mats.next().unwrap().as_str().parse().unwrap();
        instructions.push((addr, num_to_binarray(num, 36)));
    }
    masks[i - 1].0 = num_instr;
    (masks, instructions)
}

fn num_to_binarray(mut num: u64, len: usize) -> Vec<bool> {
    let mut v: Vec<bool> = Vec::new();
    for i in 0..len {
        let twop = 2u64.pow((len - i) as u32 - 1);
        v.push((num / twop) == 1);
        num %= twop;
    }
    v
}

fn binarray_to_num(barray: &[bool]) -> u64 {
    let len = barray.len() as u32;
    barray
        .iter()
        .enumerate()
        .filter(|(_, &el)| el)
        .fold(0, |acc, (i, _)| acc + 2u64.pow(len - 1 - i as u32))
}

fn apply_mask_part1(val: &[bool], mask: &[(usize, bool)]) -> Vec<bool> {
    let mut value = val.to_vec();
    for (bitpos, ovrrd) in mask {
        value[*bitpos] = *ovrrd;
    }
    value
}

fn execute_part1(
    addr: usize,
    val: &[bool],
    mask: &[(usize, bool)],
    memory: &mut HashMap<usize, Vec<bool>>,
) {
    let value = apply_mask_part1(val, mask);
    memory.insert(addr, value);
}

pub fn part1() -> u64 {
    let (masks, instructions) = load_data();
    let mut memory: HashMap<usize, Vec<bool>> = HashMap::new();
    let mut j: usize = 0;
    for (nins, mask) in masks {
        for (addr, val) in &instructions[j..j + nins] {
            execute_part1(*addr, val, &mask, &mut memory);
        }
        j += nins;
    }
    let mut res = 0u64;
    for (_, barray) in memory {
        res += binarray_to_num(&barray);
    }
    res
}

fn apply_mask_part2(addr: usize, mask: &[(usize, bool)]) -> Vec<usize> {
    let mut addr_arr = num_to_binarray(addr as u64, 36);
    let mut done_positions: Vec<bool> = vec![false; 36];
    for (bitpos, m) in mask {
        addr_arr[*bitpos] |= *m;
        done_positions[*bitpos] = true;
    }
    let floating_positions: Vec<usize> = done_positions
        .iter()
        .enumerate()
        .filter(|(_, &el)| !el)
        .map(|(idx, _)| idx)
        .collect();
    let combos = floating_positions.len();
    let mut addresses: Vec<usize> = Vec::new();
    for idx in 0..2u64.pow(combos as u32) {
        let changes = num_to_binarray(idx, combos);
        for i in 0..combos {
            addr_arr[floating_positions[i]] = changes[i];
        }
        addresses.push(binarray_to_num(&addr_arr) as usize);
    }
    addresses
}

fn execute_part2(
    addr: usize,
    val: &[bool],
    mask: &[(usize, bool)],
    memory: &mut HashMap<usize, Vec<bool>>,
) {
    let addresses = apply_mask_part2(addr, mask);
    let value = val.to_vec();
    for a in addresses {
        memory.insert(a, value.clone());
    }
}

pub fn part2() -> u64 {
    let (masks, instructions) = load_data();
    let mut memory: HashMap<usize, Vec<bool>> = HashMap::new();
    let mut j: usize = 0;
    for (nins, mask) in masks {
        for (addr, val) in &instructions[j..j + nins] {
            execute_part2(*addr, val, &mask, &mut memory);
        }
        j += nins;
    }
    let mut res = 0u64;
    for (_, barray) in memory {
        res += binarray_to_num(&barray);
    }
    res
}

#[test]
fn num_to_binarray_test() {
    let num = 11u64;
    let len = 36usize;
    let barray = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, true, true,
    ];
    assert_eq!(num_to_binarray(num, len), barray);
}

#[test]
fn binarray_to_num_test() {
    let barray = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, true, true,
    ];
    let num = 11u64;
    assert_eq!(binarray_to_num(&barray), num);
}

#[test]
fn masking_part1_test() {
    let barray = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, true, true,
    ];
    let mask = vec![(29, true), (34, false)];
    let masked = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, false, false, true, false, false, true,
    ];
    let res = apply_mask_part1(&barray, &mask);
    assert_eq!(res, masked);
    assert_eq!(binarray_to_num(&res), 73u64);
}

#[test]
fn masking_part2_test() {
    let addr = 42usize;
    let mask = vec![
        (0, false),
        (1, false),
        (2, false),
        (3, false),
        (4, false),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, false),
        (10, false),
        (11, false),
        (12, false),
        (13, false),
        (14, false),
        (15, false),
        (16, false),
        (17, false),
        (18, false),
        (19, false),
        (20, false),
        (21, false),
        (22, false),
        (23, false),
        (24, false),
        (25, false),
        (26, false),
        (27, false),
        (28, false),
        (29, false),
        (31, true),
        (32, false),
        (33, false),
        (34, true),
    ];
    let addresses: Vec<usize> = vec![26, 27, 58, 59];
    let res = apply_mask_part2(addr, &mask);
    for a in addresses {
        assert!(res.contains(&a));
    }
}
