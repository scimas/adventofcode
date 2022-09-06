use std::fs;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Planet {
    rx: i64,
    ry: i64,
    rz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Planet {
    fn new() -> Planet {
        return Planet {
            rx: 0,
            ry: 0,
            rz: 0,
            vx: 0,
            vy: 0,
            vz: 0,
        };
    }

    fn set_pos(&mut self, x: i64, y: i64, z: i64) {
        self.rx = x;
        self.ry = y;
        self.rz = z;
    }
}

fn load_data(fname: &str) -> Vec<Planet> {
    let contents = fs::read_to_string(fname).expect("Couldn't read the file!");
    let contents = contents.trim();
    let mut planets: Vec<Planet> = Vec::new();

    for line in contents.lines() {
        let filt_line = line.replace(">", "");
        let mut pos: Vec<i64> = Vec::new();
        for substr in filt_line.split(",") {
            pos.push(
                substr[3..]
                    .parse::<i64>()
                    .expect("Couldn't parse position!"),
            );
        }
        planets.push(Planet::new());
        planets.last_mut().unwrap().set_pos(pos[0], pos[1], pos[2]);
    }

    return planets;
}

fn apply_gravity(planets: &mut Vec<Planet>) {
    for i in 0..(planets.len() - 1) {
        for j in (i + 1)..planets.len() {
            // X Velocity
            if planets[i].rx > planets[j].rx {
                planets[i].vx -= 1;
                planets[j].vx += 1;
            } else if planets[i].rx < planets[j].rx {
                planets[i].vx += 1;
                planets[j].vx -= 1;
            }
            // Y Velocity
            if planets[i].ry > planets[j].ry {
                planets[i].vy -= 1;
                planets[j].vy += 1;
            } else if planets[i].ry < planets[j].ry {
                planets[i].vy += 1;
                planets[j].vy -= 1;
            }
            // Z Velocity
            if planets[i].rz > planets[j].rz {
                planets[i].vz -= 1;
                planets[j].vz += 1;
            } else if planets[i].rz < planets[j].rz {
                planets[i].vz += 1;
                planets[j].vz -= 1;
            }
        }
    }
}

fn update_pos(planets: &mut Vec<Planet>) {
    for planet in planets.iter_mut() {
        planet.rx += planet.vx;
        planet.ry += planet.vy;
        planet.rz += planet.vz;
    }
}

fn sample_energy(planets: &Vec<Planet>) -> i64 {
    let mut potential_energy: i64;
    let mut kinetic_energy: i64;
    let mut total_energy: i64 = 0;
    for planet in planets.iter() {
        potential_energy = planet.rx.abs() + planet.ry.abs() + planet.rz.abs();
        kinetic_energy = planet.vx.abs() + planet.vy.abs() + planet.vz.abs();
        total_energy += potential_energy * kinetic_energy;
    }
    return total_energy;
}

fn check_state(planets: &Vec<Planet>, original_conf: &Vec<Planet>, axis: char) -> bool {
    let mut same = true;
    if axis == 'x' {
        for i in 0..planets.len() {
            if planets[i].rx != original_conf[i].rx {
                same = false;
                break;
            }
            if planets[i].vx != original_conf[i].vx {
                same = false;
                break;
            }
        }
    } else if axis == 'y' {
        for i in 0..planets.len() {
            if planets[i].ry != original_conf[i].ry {
                same = false;
                break;
            }
            if planets[i].vy != original_conf[i].vy {
                same = false;
                break;
            }
        }
    } else if axis == 'z' {
        for i in 0..planets.len() {
            if planets[i].rz != original_conf[i].rz {
                same = false;
                break;
            }
            if planets[i].vz != original_conf[i].vz {
                same = false;
                break;
            }
        }
    }
    return same;
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    let mut t: i64;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return (a * b) / gcd(a, b);
}

fn part1() {
    let mut planets = load_data("data/Day12_input.txt");
    for _ in 0..1000 {
        apply_gravity(&mut planets);
        update_pos(&mut planets);
    }
    println!("{}", sample_energy(&planets));
}

fn part2() {
    let mut planets = load_data("data/Day12_input.txt");
    let original_conf = planets.clone();
    let mut stepx: u32 = 0;
    let mut stepy: u32 = 0;
    let mut stepz: u32 = 0;
    let mut steps: u32 = 0;
    let mut foundx = false;
    let mut foundy = false;
    let mut foundz = false;
    loop {
        steps += 1;
        apply_gravity(&mut planets);
        update_pos(&mut planets);
        if !foundx && check_state(&planets, &original_conf, 'x') {
            stepx = steps;
            foundx = true;
        }
        if !foundy && check_state(&planets, &original_conf, 'y') {
            stepy = steps;
            foundy = true;
        }
        if !foundz && check_state(&planets, &original_conf, 'z') {
            stepz = steps;
            foundz = true;
        }
        if foundx && foundy && foundz {
            break;
        }
    }
    let a = lcm(stepx as i64, stepy as i64);
    let b = lcm(stepz as i64, a);
    println!("{}, {}, {}", stepx, stepy, stepz);
    println!("{}", b);
}
