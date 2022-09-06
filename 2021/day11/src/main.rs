use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Octopus {
    energy: u32,
    brightness: Light,
}

impl Octopus {
    fn new(energy: u32) -> Self {
        Self {
            energy,
            brightness: Light::Dim,
        }
    }

    fn ready_to_flash(&self) -> bool {
        self.energy > 9 && self.brightness != Light::Flash
    }

    fn flashed(&self) -> bool {
        self.brightness == Light::Flash
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Light {
    Flash,
    Dim,
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
    let mut octopuses = Vec::new();
    let mut lines = reader.lines().peekable();
    let width = lines
        .peek()
        .ok_or_else(|| "no line in file".to_string())?
        .as_ref()
        .map_err(|_| "could not read line from file".to_string())?
        .len();

    for line in lines {
        for ch in line
            .map_err(|_| "could not read line from file".to_string())?
            .chars()
        {
            let energy = ch
                .to_digit(10)
                .ok_or_else(|| "character in file not a digit".to_string())?;
            octopuses.push(Octopus::new(energy));
        }
    }

    println!("Day 11");

    let start_time = Instant::now();
    let total_flashes = part1(octopuses.clone(), width);
    let duration = start_time.elapsed();

    println!(
        "Part 1: {}, time: {:.3e} s",
        total_flashes,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let sync_step = part2(octopuses.clone(), width);
    let duration = start_time.elapsed();

    println!(
        "Part 2: {}, time: {:.3e} s",
        sync_step,
        duration.as_secs_f32()
    );
    Ok(())
}

fn part1(mut octopuses: Vec<Octopus>, width: usize) -> usize {
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += octopuses_life_step(&mut octopuses, width);
    }
    total_flashes
}

fn part2(mut octopuses: Vec<Octopus>, width: usize) -> usize {
    let mut step = 0;
    loop {
        step += 1;
        octopuses_life_step(&mut octopuses, width);
        if octopuses.iter().all(|oct| oct.energy == 0) {
            break step;
        }
    }
}

fn octopuses_life_step(octopuses: &mut Vec<Octopus>, width: usize) -> usize {
    octopuses.iter_mut().for_each(|oct| oct.energy += 1);
    let mut total_flashes = 0;
    loop {
        let mut octopus_flashed = false;
        for i in 0..octopuses.len() {
            if octopuses[i].ready_to_flash() {
                octopuses[i].brightness = Light::Flash;
                octopus_flashed = true;
                total_flashes += 1;
                let column = i % width;
                let row = i / width;
                if row != 0 {
                    // top
                    octopuses[i - width].energy += 1;
                    if column != 0 {
                        // top left
                        octopuses[i - width - 1].energy += 1;
                    }
                    if column + 1 != width {
                        // top right
                        octopuses[i + 1 - width].energy += 1;
                    }
                }
                if column != 0 {
                    // left
                    octopuses[i - 1].energy += 1;
                    if row != octopuses.len() / width - 1 {
                        // bottom left
                        octopuses[i + width - 1].energy += 1;
                    }
                }
                if column + 1 != width {
                    // right
                    octopuses[i + 1].energy += 1;
                    if row != octopuses.len() / width - 1 {
                        // bottom right
                        octopuses[i + width + 1].energy += 1;
                    }
                }
                if row != octopuses.len() / width - 1 {
                    // bottom
                    octopuses[i + width].energy += 1;
                }
            }
        }
        if !octopus_flashed {
            break;
        }
    }
    for oct in octopuses.iter_mut() {
        if oct.flashed() {
            oct.brightness = Light::Dim;
            oct.energy = 0;
        }
    }
    total_flashes
}

#[test]
fn life_step_test() {
    let energies = vec![
        1u32, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
    ];
    let mut octopuses: Vec<Octopus> = energies.iter().map(|en| Octopus::new(*en)).collect();

    let first_step_energies = vec![
        3u32, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3,
    ];

    let second_step_energies = vec![
        4u32, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4,
    ];
    let width = 5;
    octopuses_life_step(&mut octopuses, width);
    for i in 0..energies.len() {
        assert_eq!(first_step_energies[i], octopuses[i].energy);
    }
    octopuses_life_step(&mut octopuses, width);
    for i in 0..energies.len() {
        assert_eq!(second_step_energies[i], octopuses[i].energy);
    }
}

#[test]
fn life_step_test2() {
    let energies = vec![
        5u32, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
        3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6, 4,
        5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8, 5, 5,
        4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
    ];
    let mut octopuses: Vec<Octopus> = energies.iter().map(|en| Octopus::new(*en)).collect();

    let first_step_energies = vec![
        6u32, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2, 8,
        4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3, 5, 7, 5,
        6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7, 9, 5, 9, 6, 6,
        5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
    ];

    let second_step_energies = vec![
        8u32, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6, 0,
        8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8, 8, 9, 8,
        9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0, 0, 0, 0, 8, 7,
        6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
    ];
    let width = 10;
    octopuses_life_step(&mut octopuses, width);
    for i in 0..energies.len() {
        assert_eq!(first_step_energies[i], octopuses[i].energy);
    }
    octopuses_life_step(&mut octopuses, width);
    for i in 0..energies.len() {
        assert_eq!(second_step_energies[i], octopuses[i].energy);
    }
}
