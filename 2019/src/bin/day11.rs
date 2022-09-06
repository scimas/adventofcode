use std::collections::HashMap;
use std::fs;
use std::ops;

use adventofcode2019::intcode;
use intcode::{Computer, State};

fn main() {
    part1(0);
    part2();
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        return Point { x, y };
    }

    fn rotate(&mut self, left: bool) {
        let nx: i64;
        let ny: i64;
        if left {
            nx = -self.y;
            ny = self.x;
        } else {
            nx = self.y;
            ny = -self.x;
        }
        self.x = nx;
        self.y = ny;
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

fn load_data(fname: &str) -> HashMap<usize, i64> {
    let program_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    let program_text = program_text.trim();
    let program_vec: Vec<i64> = program_text
        .split(",")
        .map(|x| x.parse::<i64>().expect("Couldn't parse instruction!"))
        .collect();
    let program: HashMap<usize, i64> = (0..program_vec.len())
        .zip(program_vec.iter())
        .map(|(k, v)| (k, *v))
        .collect();
    return program;
}

fn part1(first_inp: i64) -> HashMap<Point, i64> {
    let program = load_data("data/Day11_input.txt");
    let mut hull: HashMap<Point, i64> = HashMap::new();
    hull.insert(Point::new(0, 0), first_inp);
    let mut current_pos = Point::new(0, 0);
    let mut current_facing = Point::new(0, 1);

    let mut my_robot = Computer::new();
    my_robot.load_program(&program);
    my_robot.add_input(*hull.entry(current_pos).or_insert(0));
    my_robot.initialize();
    let mut painting = true;

    loop {
        match my_robot.get_state() {
            State::HALT => break,
            State::WAIT => {
                let out = my_robot.get_output().unwrap();
                if painting {
                    hull.insert(current_pos, out);
                    painting = false;
                } else {
                    current_facing.rotate(out == 0);
                    current_pos = current_pos + current_facing;
                    let inp = hull.entry(current_pos).or_insert(-1);
                    if *inp == -1 {
                        my_robot.add_input(0);
                    } else {
                        my_robot.add_input(*inp);
                    }
                    painting = true;
                }
                my_robot.initialize();
            }
            _ => my_robot.run(),
        }
    }

    let mut painted_panels = hull.len();
    if *hull.get(&current_pos).unwrap() == -1 {
        painted_panels -= 1;
    }
    println!("{}", painted_panels);
    return hull;
}

fn part2() {
    let hull = part1(1);

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for &k in hull.keys() {
        if k.x < min_x {
            min_x = k.x;
        }
        if k.y < min_y {
            min_y = k.y;
        }
        if k.x > max_x {
            max_x = k.x;
        }
        if k.y > max_y {
            max_y = k.y;
        }
    }

    max_x -= min_x;
    max_y -= min_y;

    let mut grid: Vec<Vec<i64>> = Vec::new();

    for row in 0..(max_y + 1) {
        grid.push(Vec::new());
        for _col in 0..(max_x + 1) {
            grid[row as usize].push(0);
        }
    }

    for (k, v) in &hull {
        let x = (k.x - min_x) as usize;
        let y = (k.y - min_y) as usize;
        grid[y][x] = *v;
    }
    grid.reverse();
    for row in 0..(max_y + 1) {
        println!("{:?}", grid[row as usize]);
    }
}
