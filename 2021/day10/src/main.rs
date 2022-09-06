use std::{
    collections::HashMap,
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
    let nav_subsystem: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    println!("Day 10");

    let start_time = Instant::now();
    let (error_points, corrupt_indices) = part1(&nav_subsystem);
    let duration = start_time.elapsed();

    println!(
        "Part 1: {}, time: {:.3e} s",
        error_points,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let completion_points = part2(&nav_subsystem, &corrupt_indices);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        completion_points,
        duration.as_secs_f32()
    );
    Ok(())
}

fn part1(lines: &[String]) -> (u64, Vec<usize>) {
    let point_map = HashMap::from([(')', 3u64), (']', 57), ('}', 1197), ('>', 25137)]);
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut points = 0;
    let mut corrupt_indices = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let mut stack = Vec::with_capacity(line.len());
        for ch in line.trim().chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => {
                    if let Some(open_ch) = stack.pop() {
                        if ch != pairs[&open_ch] {
                            points += point_map[&ch];
                            corrupt_indices.push(idx);
                        }
                    } else {
                        points += point_map[&ch];
                        corrupt_indices.push(idx);
                    }
                }
                _ => panic!("unexpected character encountered {}", ch),
            }
        }
    }
    (points, corrupt_indices)
}

fn part2(lines: &[String], corrupt_lines: &[usize]) -> u64 {
    let point_map = HashMap::from([(')', 1u64), (']', 2), ('}', 3), ('>', 4)]);
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut points = Vec::new();
    for line in lines
        .iter()
        .enumerate()
        .filter(|(idx, _)| !corrupt_lines.contains(idx))
        .map(|(_, l)| l)
    {
        let mut stack = Vec::with_capacity(line.len());
        for ch in line.trim().chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => panic!("unexpected character encountered {}", ch),
            }
        }
        let mut tot_points = 0;
        for ch in stack.iter().rev() {
            let close_ch = pairs[ch];
            tot_points *= 5;
            tot_points += point_map[&close_ch];
        }
        points.push(tot_points);
    }
    points.sort_unstable();
    points[points.len() / 2]
}
