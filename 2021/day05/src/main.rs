use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p1: Point,
    p2: Point,
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
    let lines: Vec<Line> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut point_iter = line.split(" -> ").map(|p_str| {
                let mut coord_iter = p_str.split(',').map(|coord_s| coord_s.parse().unwrap());
                Point {
                    x: coord_iter.next().unwrap(),
                    y: coord_iter.next().unwrap(),
                }
            });
            Line {
                p1: point_iter.next().unwrap(),
                p2: point_iter.next().unwrap(),
            }
        })
        .collect();

    println!("Day 05");

    let start_time = Instant::now();
    let p1_overlaps = solve(&lines, false);
    let duration = start_time.elapsed();
    println!(
        "Part 1: {}, time: {:.3e} s",
        p1_overlaps,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let p2_overlaps = solve(&lines, true);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        p2_overlaps,
        duration.as_secs_f32()
    );
    Ok(())
}

fn solve(lines: &[Line], include_diagonals: bool) -> usize {
    let mut point_counts: HashMap<Point, bool> = HashMap::new();
    lines
        .iter()
        .filter(|Line { p1, p2 }| include_diagonals || (p1.x == p2.x) || (p1.y == p2.y))
        .for_each(|Line { p1, p2 }| {
            let x_adder = match p2.x.cmp(&p1.x) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            let y_adder = match p2.y.cmp(&p1.y) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            let mut cur_point = *p1;
            while cur_point != *p2 {
                point_counts
                    .entry(cur_point)
                    .and_modify(|multiple| *multiple = true)
                    .or_insert_with(|| false);
                cur_point.x += x_adder;
                cur_point.y += y_adder;
            }
            point_counts
                .entry(cur_point)
                .and_modify(|multiple| *multiple = true)
                .or_insert_with(|| false);
        });
    point_counts
        .iter()
        .filter(|(_, &multiple)| multiple)
        .count()
}
