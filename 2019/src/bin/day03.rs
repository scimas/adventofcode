extern crate regex;
use regex::Regex;
use std::fs;

fn main() {
    part1();
    part2();
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, p: &Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

fn load_data(fname: &str) -> (Vec<Point>, Vec<Point>) {
    let moves = fs::read_to_string(fname).expect("Couldn't read the file!");
    let mut moves = moves.trim().split_whitespace();
    let wire1 = moves.next().unwrap();
    let wire2 = moves.next().unwrap();

    let move_pattern = Regex::new(r"([A-Z])([0-9]+)").unwrap();

    let mut wire1_vec: Vec<Point> = vec![Point::new(0, 0)];
    for cap in move_pattern.captures_iter(wire1) {
        let last_point: Point = *wire1_vec.last().unwrap();
        let m: i32 = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "U" => wire1_vec.push(Point::new(last_point.x, last_point.y + m)),
            "D" => wire1_vec.push(Point::new(last_point.x, last_point.y - m)),
            "L" => wire1_vec.push(Point::new(last_point.x - m, last_point.y)),
            "R" => wire1_vec.push(Point::new(last_point.x + m, last_point.y)),
            _ => println!("You probably read the file incorrectly!"),
        }
    }

    let mut wire2_vec: Vec<Point> = vec![Point::new(0, 0)];
    for cap in move_pattern.captures_iter(wire2) {
        let last_point: Point = *wire2_vec.last().unwrap();
        let m: i32 = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "U" => wire2_vec.push(Point::new(last_point.x, last_point.y + m)),
            "D" => wire2_vec.push(Point::new(last_point.x, last_point.y - m)),
            "L" => wire2_vec.push(Point::new(last_point.x - m, last_point.y)),
            "R" => wire2_vec.push(Point::new(last_point.x + m, last_point.y)),
            _ => println!("You probably read the file incorrectly!"),
        }
    }
    (wire1_vec, wire2_vec)
}

fn get_crossings(wire1: &[Point], wire2: &[Point]) -> (Vec<Point>, Vec<(usize, usize)>) {
    let mut possible_crossings: Vec<Point> = Vec::new();
    let mut crossing_indices: Vec<(usize, usize)> = Vec::new();
    for i in 0..(wire1.len() - 1) {
        for j in 0..(wire2.len() - 1) {
            let x_diff = (wire1[i].x - wire2[j].x) * (wire1[i + 1].x - wire2[j + 1].x);
            let y_diff = (wire1[i].y - wire2[j].y) * (wire1[i + 1].y - wire2[j + 1].y);
            if (x_diff > 0) || (y_diff > 0) {
                continue;
            }
            crossing_indices.push((i, j));
            if x_diff == 0 {
                let mut ys = [wire1[i].y, wire1[i + 1].y, wire2[j].y, wire2[j + 1].y];
                ys.sort_unstable();
                possible_crossings.push(Point::new(wire1[i].x, ys[1]));
                possible_crossings.push(Point::new(wire1[i].x, ys[2]));
            } else if y_diff == 0 {
                let mut xs = [wire1[i].x, wire1[i + 1].x, wire2[j].x, wire2[j + 1].x];
                xs.sort_unstable();
                possible_crossings.push(Point::new(xs[1], wire1[i].y));
                possible_crossings.push(Point::new(xs[2], wire1[i].y));
            } else {
                let mut xs = [wire1[i].x, wire1[i + 1].x, wire2[j].x, wire2[j + 1].x];
                let mut ys = [wire1[i].y, wire1[i + 1].y, wire2[j].y, wire2[j + 1].y];
                xs.sort_unstable();
                ys.sort_unstable();
                possible_crossings.push(Point::new(xs[1], ys[1]));
            }
        }
    }
    (possible_crossings, crossing_indices)
}

fn part1() {
    let (wire1, wire2) = load_data("data/Day03_input.txt");
    let (possible_crossings, _crossing_indices) = get_crossings(&wire1, &wire2);
    let origin: Point = Point::new(0, 0);
    let mut distances: Vec<i32> = possible_crossings
        .iter()
        .map(|p| p.manhattan_distance(&origin))
        .collect();
    distances.sort_unstable();
    println!("{}", distances.iter().find(|&&d| d != 0).unwrap());
}

fn part2() {
    let (wire1, wire2) = load_data("data/Day03_input.txt");
    let (_possible_crossings, crossing_indices) = get_crossings(&wire1, &wire2);
    let mut crossing_steps: Vec<i32> = Vec::new();
    for cross in crossing_indices {
        if cross.0 == 0 && cross.1 == 0 {
            continue;
        }
        let mut steps: i32 = 0;
        for i in 0..cross.0 {
            steps += wire1[i].manhattan_distance(&wire2[i + 1]);
        }
        for j in 0..cross.1 {
            steps += wire2[j].manhattan_distance(&wire2[j + 1]);
        }
        let mut xs = [
            wire1[cross.0].x,
            wire1[cross.0 + 1].x,
            wire2[cross.1].x,
            wire2[cross.1 + 1].x,
        ];
        let mut ys = [
            wire1[cross.0].y,
            wire1[cross.0 + 1].y,
            wire2[cross.1].y,
            wire2[cross.1 + 1].y,
        ];
        xs.sort_unstable();
        ys.sort_unstable();
        steps += wire1[cross.0].manhattan_distance(&Point::new(xs[2], ys[2]));
        steps += wire2[cross.1].manhattan_distance(&Point::new(xs[2], ys[2]));
        crossing_steps.push(steps);
    }
    println!("{}", crossing_steps.iter().min().unwrap());
}
