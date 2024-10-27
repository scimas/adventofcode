use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {}

fn load_data() -> (u64, Vec<u64>) {
    let f = File::open("res/input13.txt").expect("Couldn't open day 13 input");
    let mut reader = BufReader::new(f);

    let mut tstring = String::new();
    reader
        .read_line(&mut tstring)
        .expect("Couldn't read line from day 13 input");
    let timestamp: u64 = tstring.trim().parse().unwrap();

    let mut buss = String::new();
    reader
        .read_line(&mut buss)
        .expect("Couldn't read line from day 13 input");
    let bus_ids: Vec<u64> = buss.split(',').filter_map(|s| s.parse().ok()).collect();
    (timestamp, bus_ids)
}

fn closest_multiple_higher(close_to: u64, mult_of: u64) -> u64 {
    let rem = close_to % mult_of;
    if rem == 0 {
        return close_to;
    }
    let div = close_to / mult_of + 1;
    div * mult_of
}

pub fn part1() -> u64 {
    let (timestamp, bus_ids) = load_data();
    let mut earliest_departures: Vec<u64> = Vec::new();
    for bid in &bus_ids {
        earliest_departures.push(closest_multiple_higher(timestamp, *bid));
    }
    let (idx, &depart) = earliest_departures
        .iter()
        .enumerate()
        .min_by_key(|(_, ts)| *ts)
        .unwrap();
    bus_ids[idx] * (depart - timestamp)
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r: Vec<i64> = vec![a, b];
    let mut s: Vec<i64> = vec![1, 0];
    let mut t: Vec<i64> = vec![0, 1];
    let mut i: usize = 1;
    while r[i] != 0 {
        let q = r[i - 1].div_euclid(r[i]);
        r.push(r[i - 1] - q * r[i]);
        s.push(s[i - 1] - q * s[i]);
        t.push(t[i - 1] - q * t[i]);
        i += 1;
    }
    (r[i - 1], s[i - 1], t[i - 1])
}

#[test]
fn euclid_test() {
    assert_eq!(extended_euclidean(240, 46), (2, -9, 47));
    assert_eq!(extended_euclidean(46, 240), (2, 47, -9));
}

fn chinese_remainder(rem_nums: &[(i64, i64)]) -> i64 {
    let mut x: i64 = 0;
    let n = rem_nums.iter().fold(1, |acc, (_, x)| acc * *x);
    for (off, i) in rem_nums {
        let (_, _, bcb) = extended_euclidean(*i, n / *i);
        x += (off * bcb * (n / *i)).rem_euclid(n);
    }
    x.rem_euclid(n)
}

#[test]
fn crt_test() {
    let bus_ids = vec![(-1, 13), (-4, 59), (-7, 19), (0, 7), (-6, 31)];
    assert_eq!(chinese_remainder(&bus_ids), 1068781);
    let bus_ids = vec![(0, 17), (-2, 13), (-3, 19)];
    assert_eq!(chinese_remainder(&bus_ids), 3417);
    let bus_ids = vec![(0, 67), (-2, 59), (-1, 7), (-3, 61)];
    assert_eq!(chinese_remainder(&bus_ids), 754018);
    let bus_ids = vec![(0, 67), (-2, 7), (-3, 59), (-4, 61)];
    assert_eq!(chinese_remainder(&bus_ids), 779210);
    let bus_ids = vec![(0, 67), (-1, 7), (-3, 59), (-4, 61)];
    assert_eq!(chinese_remainder(&bus_ids), 1261476);
    let bus_ids = vec![(0, 1789), (-1, 37), (-2, 47), (-3, 1889)];
    assert_eq!(chinese_remainder(&bus_ids), 1202161486);
}

pub fn part2() -> i64 {
    let f = File::open("res/input13.txt").expect("Couldn't open day 13 input");
    let reader = BufReader::new(f);
    let buss = reader.lines().nth(1).unwrap().unwrap();
    let mut bus_ids: Vec<(i64, i64)> = buss
        .trim()
        .split(',')
        .enumerate()
        .filter_map(|(idx, s)| {
            if s == "x" {
                None
            } else {
                Some((-(idx as i64), s.parse().unwrap()))
            }
        })
        .collect();
    bus_ids.sort_by_key(|(_, bid)| *bid);
    bus_ids.reverse();
    chinese_remainder(&bus_ids)
}
