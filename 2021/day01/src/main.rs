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
    let depths: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("Day 01");

    let start_time = Instant::now();
    let p1 = depths.windows(2).filter(|&pair| pair[1] > pair[0]).count();
    let duration = start_time.elapsed();
    println!("Part 1: {}, time: {:.3e} s", p1, duration.as_secs_f32());

    let start_time = Instant::now();
    let p2 = depths.windows(4).filter(|&pair| pair[3] > pair[0]).count();
    let duration = start_time.elapsed();
    println!("Part 2: {}, time: {:.3e} s", p2, duration.as_secs_f32());
    Ok(())
}
