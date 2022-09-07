use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ip {
    supernet_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl Ip {
    fn new(supernet_sequences: Vec<String>, hypernet_sequences: Vec<String>) -> Self {
        Self {
            supernet_sequences,
            hypernet_sequences,
        }
    }

    fn supports_tls(&self) -> bool {
        // not(any hypernet sequence has abba) and (any ordnet sequence has abba)
        !self.hypernet_sequences.iter().any(|seq| has_abba(seq))
            && self.supernet_sequences.iter().any(|seq| has_abba(seq))
    }

    fn supports_sls(&self) -> bool {
        let supernet_abas: Vec<String> = self
            .supernet_sequences
            .iter()
            .flat_map(|seq| get_abas(seq))
            .collect();
        if !supernet_abas.is_empty() {
            let hypernet_abas: Vec<String> = self
                .hypernet_sequences
                .iter()
                .flat_map(|seq| get_abas(seq))
                .collect();
            if !hypernet_abas.is_empty() {
                let supernet_babs: Vec<String> = supernet_abas
                    .into_iter()
                    .map(|s| s[1..].repeat(2)[..3].to_string())
                    .collect();
                return supernet_babs.iter().any(|aba| hypernet_abas.contains(aba));
            }
        }
        false
    }
}

fn main() {
    let fl = File::open("res/day07/input").expect("couldn't read file");
    let reader = BufReader::new(fl);
    let ips: Vec<Ip> = reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line from file");
            let mut supernet_sequences = Vec::with_capacity(4);
            let mut hypernet_sequences = Vec::with_capacity(4);
            let mut ord = true;
            for seq in line.split(|ch| ch == '[' || ch == ']') {
                if ord {
                    supernet_sequences.push(seq.to_string());
                } else {
                    hypernet_sequences.push(seq.to_string());
                }
                ord = !ord;
            }
            Ip::new(supernet_sequences, hypernet_sequences)
        })
        .collect();
    println!("Day 07");
    println!("Part 1: {}", part1(&ips));
    println!("Part 2: {}", part2(&ips));
}

/// Check whether given string has Autonomous Bridge Bypass Annotation (ABBA)
fn has_abba(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(chars.len() - 3) {
        if chars[i] != chars[i + 1] {
            if chars[i + 1] == chars[i + 2] {
                if chars[i] == chars[i + 3] {
                    return true;
                }
            }
        }
    }
    false
}

fn part1(ips: &[Ip]) -> usize {
    ips.iter().filter(|ip| ip.supports_tls()).count()
}

fn get_abas(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut abas = vec![];
    for i in 0..(chars.len() - 2) {
        if chars[i] != chars[i + 1] {
            if chars[i] == chars[i + 2] {
                abas.push(s[i..=i + 2].to_string());
            }
        }
    }
    return abas;
}

fn part2(ips: &[Ip]) -> usize {
    ips.iter().filter(|ip| ip.supports_sls()).count()
}

#[test]
fn proper_aba() {
    let s = "zazbz";
    let expected_abas = vec!["zaz", "zbz"];
    assert_eq!(expected_abas, get_abas(s));
}

#[test]
fn combine_multiple_abas() {
    let s = "zazbz";
    let t = "xyx";
    let expected_combo = vec!["zaz", "zbz", "xyx"];
    let actual_combo: Vec<String> = vec![s, t].iter().flat_map(|seq| get_abas(seq)).collect();
    assert_eq!(expected_combo, actual_combo);
}
