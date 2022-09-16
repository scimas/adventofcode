use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug)]
struct Num(u8);

impl Num {
    const fn left(&self) -> Num {
        match self.0 {
            1 => Num(1),
            2 => Num(1),
            3 => Num(2),
            4 => Num(4),
            5 => Num(4),
            6 => Num(5),
            7 => Num(7),
            8 => Num(7),
            9 => Num(8),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn left2(&self) -> Num {
        match self.0 {
            1 => Num(1),
            2 => Num(2),
            3 => Num(2),
            4 => Num(3),
            5 => Num(5),
            6 => Num(5),
            7 => Num(6),
            8 => Num(7),
            9 => Num(8),
            0xa => Num(0xa),
            0xb => Num(0xa),
            0xc => Num(0xb),
            0xd => Num(0xd),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn right(&self) -> Num {
        match self.0 {
            1 => Num(2),
            2 => Num(3),
            3 => Num(3),
            4 => Num(5),
            5 => Num(6),
            6 => Num(6),
            7 => Num(8),
            8 => Num(9),
            9 => Num(9),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn right2(&self) -> Num {
        match self.0 {
            1 => Num(1),
            2 => Num(3),
            3 => Num(4),
            4 => Num(4),
            5 => Num(6),
            6 => Num(7),
            7 => Num(8),
            8 => Num(9),
            9 => Num(9),
            0xa => Num(0xb),
            0xb => Num(0xc),
            0xc => Num(0xc),
            0xd => Num(0xd),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn up(&self) -> Num {
        match self.0 {
            1 => Num(1),
            2 => Num(2),
            3 => Num(3),
            4 => Num(1),
            5 => Num(2),
            6 => Num(3),
            7 => Num(4),
            8 => Num(5),
            9 => Num(6),
            _ => panic!("Not a valid numpad number"),
        }
    }
    
    const fn up2(&self) -> Num {
        match self.0 {
            1 => Num(1),
            2 => Num(2),
            3 => Num(1),
            4 => Num(4),
            5 => Num(5),
            6 => Num(2),
            7 => Num(3),
            8 => Num(4),
            9 => Num(9),
            0xa => Num(6),
            0xb => Num(7),
            0xc => Num(8),
            0xd => Num(0xb),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn down(&self) -> Num {
        match self.0 {
            1 => Num(4),
            2 => Num(5),
            3 => Num(6),
            4 => Num(7),
            5 => Num(8),
            6 => Num(9),
            7 => Num(7),
            8 => Num(8),
            9 => Num(9),
            _ => panic!("Not a valid numpad number"),
        }
    }

    const fn down2(&self) -> Num {
        match self.0 {
            1 => Num(3),
            2 => Num(6),
            3 => Num(7),
            4 => Num(8),
            5 => Num(5),
            6 => Num(0xa),
            7 => Num(0xb),
            8 => Num(0xc),
            9 => Num(9),
            0xa => Num(0xa),
            0xb => Num(0xd),
            0xc => Num(0xc),
            0xd => Num(0xd),
            _ => panic!("Not a valid numpad number"),
        }
    }
}

fn part1(directions: &[String]) -> Vec<Num> {
    directions.iter().map(|s| s.chars().fold(Num(5), |a, d| {
        match d {
            'L' => a.left(),
            'R' => a.right(),
            'U' => a.up(),
            'D' => a.down(),
            _ => panic!("Unknown direction"),
        }
    })).collect()
}

fn part2(directions: &[String]) -> Vec<Num> {
    directions.iter().map(|s| s.chars().fold(Num(5), |a, d| {
        match d {
            'L' => a.left2(),
            'R' => a.right2(),
            'U' => a.up2(),
            'D' => a.down2(),
            _ => panic!("Unknown direction"),
        }
    })).collect()
}

fn main() {
    let fl = File::open("res/day02/input").unwrap();
    let reader = BufReader::new(fl);
    let directions: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    println!("Part 1: {:?}", part1(&directions));
    println!("Part 2: {:?}", part2(&directions));
}
