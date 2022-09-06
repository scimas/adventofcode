use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Vector<T: Add + Copy> {
    x: T,
    y: T,
}

impl<T: Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

type Instruction = (String, i32);

fn load_instructions() -> Vec<Instruction> {
    let f = File::open("res/input12.txt").expect("Couldn't open day 12 input");
    let reader = BufReader::new(f);
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in reader.lines() {
        let s = line.expect("Couldn't read line from day 12 input");
        let instr = s[..1].to_string();
        let movement: i32 = s[1..].parse().unwrap();
        instructions.push((instr, movement));
    }
    instructions
}

fn execute_part1(
    ins: Instruction,
    pos: Vector<i32>,
    facing: Vector<i32>,
) -> (Vector<i32>, Vector<i32>) {
    if &ins.0 == "N" {
        (pos + Vector { x: 0, y: ins.1 }, facing)
    } else if &ins.0 == "S" {
        (pos + Vector { x: 0, y: -ins.1 }, facing)
    } else if &ins.0 == "E" {
        (pos + Vector { x: ins.1, y: 0 }, facing)
    } else if &ins.0 == "W" {
        (pos + Vector { x: -ins.1, y: 0 }, facing)
    } else if &ins.0 == "F" {
        (pos + (facing * ins.1), facing)
    } else {
        let mut xx = 0;
        let mut xy = 0;
        let mut yx = 0;
        let mut yy = 0;
        if ins.1 == 90 {
            xy = -1;
            yx = 1;
        } else if ins.1 == 180 {
            xx = -1;
            yy = -1;
        } else if ins.1 == 270 {
            xy = 1;
            yx = -1;
        }
        if &ins.0 == "L" {
            (
                pos,
                Vector {
                    x: xx * facing.x + xy * facing.y,
                    y: yx * facing.x + yy * facing.y,
                },
            )
        } else {
            (
                pos,
                Vector {
                    x: xx * facing.x - xy * facing.y,
                    y: -yx * facing.x + yy * facing.y,
                },
            )
        }
    }
}

pub fn part1() -> i32 {
    let instructions = load_instructions();
    let mut position = Vector { x: 0, y: 0 };
    let mut facing = Vector { x: 1, y: 0 };
    for ins in instructions {
        let res = execute_part1(ins, position, facing);
        position = res.0;
        facing = res.1;
    }
    position.x.abs() + position.y.abs()
}

fn execute_part2(
    ins: Instruction,
    waypoint: Vector<i32>,
    position: Vector<i32>,
) -> (Vector<i32>, Vector<i32>) {
    if &ins.0 == "N" {
        (waypoint + Vector { x: 0, y: ins.1 }, position)
    } else if &ins.0 == "S" {
        (waypoint + Vector { x: 0, y: -ins.1 }, position)
    } else if &ins.0 == "E" {
        (waypoint + Vector { x: ins.1, y: 0 }, position)
    } else if &ins.0 == "W" {
        (waypoint + Vector { x: -ins.1, y: 0 }, position)
    } else if &ins.0 == "F" {
        (waypoint, position + (waypoint * ins.1))
    } else {
        let mut xx = 0;
        let mut xy = 0;
        let mut yx = 0;
        let mut yy = 0;
        if ins.1 == 90 {
            xy = -1;
            yx = 1;
        } else if ins.1 == 180 {
            xx = -1;
            yy = -1;
        } else if ins.1 == 270 {
            xy = 1;
            yx = -1;
        }
        if &ins.0 == "L" {
            (
                Vector {
                    x: xx * waypoint.x + xy * waypoint.y,
                    y: yx * waypoint.x + yy * waypoint.y,
                },
                position,
            )
        } else {
            (
                Vector {
                    x: xx * waypoint.x - xy * waypoint.y,
                    y: -yx * waypoint.x + yy * waypoint.y,
                },
                position,
            )
        }
    }
}

pub fn part2() -> i32 {
    let instructions = load_instructions();
    let mut position = Vector { x: 0, y: 0 };
    let mut waypoint = Vector { x: 10, y: 1 };
    for ins in instructions {
        let res = execute_part2(ins, waypoint, position);
        waypoint = res.0;
        position = res.1;
    }
    position.x.abs() + position.y.abs()
}

#[test]
fn rotation_part1() {
    let position = Vector { x: 0, y: 0 };
    let facing = Vector { x: 1, y: 0 };
    let res = execute_part1(("L".to_string(), 90), position, facing);
    assert_eq!(res.1, Vector { x: 0, y: 1 });
    let res = execute_part1(("R".to_string(), 90), position, facing);
    assert_eq!(res.1, Vector { x: 0, y: -1 });
    let res = execute_part1(("L".to_string(), 180), position, facing);
    assert_eq!(res.1, Vector { x: -1, y: 0 });
    let res = execute_part1(("R".to_string(), 180), position, facing);
    assert_eq!(res.1, Vector { x: -1, y: 0 });
}
