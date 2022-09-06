use std::{fs::File, io::Read, time::Instant};

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let mut f;
    if let Some(fpath) = args.nth(1) {
        let base_path = std::env::current_dir()
            .map_err(|_| "could not resolve current directory".to_string())?;
        f = File::open(base_path.join(&fpath))
            .map_err(|_| format!(r#"could not open file "{fpath}""#))?;
    } else {
        return Err("incorrect number of arguments".into());
    }
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let positions: Vec<i64> = buf.trim().split(',').map(|n| n.parse().unwrap()).collect();
    println!("Day 07");

    let start_time = Instant::now();
    let p1_fuel = part1(positions.clone());
    let duration = start_time.elapsed();
    println!(
        "Part 1: {}, time: {:.3e} s",
        p1_fuel,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let p2_fuel = part2(&positions);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        p2_fuel,
        duration.as_secs_f32()
    );
    Ok(())
}

fn part1(mut positions: Vec<i64>) -> i64 {
    positions.sort_unstable();
    let median = positions[positions.len() / 2];
    positions.iter().map(|&p| (p - median).abs()).sum()
}

fn part2(positions: &[i64]) -> i64 {
    let mean = positions.iter().sum::<i64>() / (positions.len() as i64);
    positions
        .iter()
        .map(|&p| {
            let a = (p - mean).abs();
            a * (a + 1)
        })
        .sum::<i64>()
        / 2
}
