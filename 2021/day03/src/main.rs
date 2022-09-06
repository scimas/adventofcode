use std::{
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
    let numbers: Vec<_> = reader
        .lines()
        .map(|line| u64::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();
    println!("Day 03");

    let start_time = Instant::now();
    let power_consumption = part1(&numbers);
    let duration = start_time.elapsed();
    println!(
        "Part 1: {}, time: {:.3e} s",
        power_consumption,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let life_support_rating = part2(&numbers);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        life_support_rating,
        duration.as_secs_f32()
    );
    Ok(())
}

fn part1(numbers: &[u64]) -> u64 {
    let max_count = numbers.len() as u64;
    let mut gamma_rate: u64 = 0;
    for i in 0..12usize {
        let mut indicator = 0;
        for n in numbers {
            indicator += (n >> i) & 1;
        }
        if indicator >= max_count / 2 {
            gamma_rate += 1 << i;
        }
    }
    let epsilon_rate = !gamma_rate & 0b111111111111;
    gamma_rate * epsilon_rate
}

fn part2(numbers: &[u64]) -> u64 {
    oxygen_number(numbers.to_vec()) * co2_number(numbers.to_vec())
}

fn oxygen_number(mut numbers: Vec<u64>) -> u64 {
    let mut bit_pos = 11;
    while numbers.len() > 1 {
        let mut indicator = 0;
        for n in &numbers {
            indicator += (n >> bit_pos) & 1;
        }
        let mask;
        if indicator * 2 >= numbers.len() as u64 {
            mask = 1;
        } else {
            mask = 0;
        }
        numbers = numbers
            .iter()
            .filter_map(|n| {
                if (n >> bit_pos) & 1 == mask {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        bit_pos -= 1;
    }
    numbers[0]
}

fn co2_number(mut numbers: Vec<u64>) -> u64 {
    let mut bit_pos = 11;
    while numbers.len() > 1 {
        let mut indicator = 0;
        for n in &numbers {
            indicator += (n >> bit_pos) & 1;
        }
        let mask;
        if indicator * 2 >= numbers.len() as u64 {
            mask = 0;
        } else {
            mask = 1;
        }
        numbers = numbers
            .iter()
            .filter_map(|n| {
                if (n >> bit_pos) & 1 == mask {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        bit_pos -= 1;
    }
    numbers[0]
}
