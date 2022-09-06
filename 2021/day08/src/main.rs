use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let f;
    if let Some(fpath) = args.nth(1) {
        let base_path = std::env::current_dir()
            .map_err(|_| "could not resolve current directory".to_string())?;
        f = File::open(base_path.join(&fpath))
            .map_err(|_| format!(r#"could not open file "{fpath}""#))?;
    } else {
        return Err("incorrect number of arguments".into());
    }
    let reader = BufReader::new(f);
    let outputs: Vec<(Vec<String>, Vec<String>)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(" | ");
            let patterns: Vec<String> =
                parts.next().unwrap().split(' ').map(|s| s.into()).collect();
            let digits: Vec<String> = parts.next().unwrap().split(' ').map(|s| s.into()).collect();
            (patterns, digits)
        })
        .collect();

    println!("Day 08");

    let start_time = Instant::now();
    let simple_digit_count = part1(&outputs);
    let duration = start_time.elapsed();
    println!(
        "Part 1: {}, time: {:.3e} s",
        simple_digit_count,
        duration.as_micros()
    );

    let start_time = Instant::now();
    let output_sum = part2(&outputs);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        output_sum,
        duration.as_micros()
    );
    Ok(())
}

fn part1(outputs: &[(Vec<String>, Vec<String>)]) -> usize {
    outputs.iter().fold(0, |acc, elem| {
        acc + elem
            .1
            .iter()
            .filter(|digit| [2usize, 3, 4, 7].contains(&digit.len()))
            .count()
    })
}

fn part2(outputs: &[(Vec<String>, Vec<String>)]) -> u64 {
    let mut n: u64 = 0;
    for (patterns, digits) in outputs {
        let mut digit_map: HashMap<u64, HashSet<char>> = HashMap::with_capacity(10);
        for p in patterns {
            let charset = HashSet::from_iter(p.chars());
            let digit = match charset.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            };
            if let Some(d) = digit {
                digit_map.insert(d, charset);
            }
        }
        let mut num = String::with_capacity(4);
        for od in digits {
            let charset = HashSet::from_iter(od.chars());
            num.push(match charset.len() {
                2 => '1',
                3 => '7',
                4 => '4',
                7 => '8',
                5 => {
                    if charset.is_superset(&digit_map[&7]) {
                        '3'
                    } else if charset.intersection(&digit_map[&4]).count() == 3 {
                        '5'
                    } else {
                        '2'
                    }
                }
                6 => {
                    if charset.is_superset(&digit_map[&4]) {
                        '9'
                    } else if charset.is_superset(&digit_map[&7]) {
                        '0'
                    } else {
                        '6'
                    }
                }
                _ => panic!("Unexpected length of digit"),
            });
        }
        n += num.parse::<u64>().unwrap();
    }
    n
}
