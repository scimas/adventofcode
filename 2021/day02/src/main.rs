use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

enum Direction {
    Horizontal(i64),
    Vertical(i64),
}

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
    let instructions: Vec<Direction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(' ');
            let dir = parts.next().unwrap();
            if dir == "forward" {
                Direction::Horizontal(parts.next().unwrap().parse().unwrap())
            } else if dir == "up" {
                Direction::Vertical(parts.next().unwrap().parse().unwrap())
            } else {
                Direction::Vertical(-parts.next().unwrap().parse::<i64>().unwrap())
            }
        })
        .collect();
    println!("Day 02");

    let start_time = Instant::now();
    let p1 = part1(&instructions);
    let duration = start_time.elapsed();
    println!("Part 1: {}, time: {:.3e} s", p1, duration.as_secs_f32());

    let start_time = Instant::now();
    let p2 = part2(&instructions);
    let duration = start_time.elapsed();
    println!("Part 2: {}, time: {:.3e} s", p2, duration.as_secs_f32());
    Ok(())
}

fn part1(instructions: &[Direction]) -> i64 {
    let mut horz_pos = 0;
    let mut vert_pos = 0;
    for dir in instructions {
        match dir {
            Direction::Horizontal(mv) => horz_pos += mv,
            Direction::Vertical(mv) => vert_pos += mv,
        }
    }
    -(horz_pos * vert_pos)
}

fn part2(instructions: &[Direction]) -> i64 {
    let mut horz_pos = 0;
    let mut vert_pos = 0;
    let mut aim = 0;
    for dir in instructions {
        match dir {
            Direction::Horizontal(mv) => {
                horz_pos += mv;
                vert_pos += mv * aim;
            }
            Direction::Vertical(mv) => aim += mv,
        }
    }
    -(horz_pos * vert_pos)
}
