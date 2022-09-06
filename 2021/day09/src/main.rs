use std::{fs::File, io::Read, time::Instant};

const WIDTH: usize = 100;

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
    let heightmap: Vec<u32> = buf
        .trim()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect();
    println!("Day 09");
    let start_time = Instant::now();
    let (p1, low_points) = part1(&heightmap);
    let duration = start_time.elapsed();
    println!("Part 1: {}, time: {:.3e} s", p1, duration.as_micros());
    Ok(())
}

fn part1(heightmap: &[u32]) -> (u32, Vec<usize>) {
    let mut total_risk = 0;
    let mut low_points = Vec::new();
    for i in 0..heightmap.len() {
        let lower_than_left = match (i % WIDTH).checked_sub(1) {
            None => true,
            Some(_) => heightmap[i] < heightmap[i - 1],
        };
        let lower_than_right = match (i + 1) % WIDTH {
            0 => true,
            _ => heightmap[i] < heightmap[i + 1],
        };
        let lower_than_up = match i.checked_sub(WIDTH) {
            None => true,
            Some(d) => heightmap[i] < heightmap[d],
        };
        let lower_than_down = if i + WIDTH >= heightmap.len() {
            true
        } else {
            heightmap[i] < heightmap[i + WIDTH]
        };
        if lower_than_left && lower_than_right && lower_than_up && lower_than_down {
            total_risk += 1 + heightmap[i];
            low_points.push(i);
        }
    }
    (total_risk, low_points)
}
