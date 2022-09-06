use std::fs;

fn main() {
    let mass_text = fs::read_to_string("../data/Day01_input.csv").expect("Error reading file!");
    let mut masses: Vec<u32> = Vec::new();
    for m in mass_text.split_whitespace() {
        masses.push(m.parse::<u32>().expect("Couldn't parse mass."));
    }
    let mut fuel: Vec<u32> = Vec::with_capacity(masses.len());
    let mut total_fuel: u32 = 0;
    for m in masses.iter() {
        fuel.push(m / 3 - 2);
    }

    // Part 1
    for f in fuel.iter() {
        total_fuel += f;
    }
    println!("{}", total_fuel);

    // Part 2
    let mut extra_fuel: Vec<u32> = Vec::with_capacity(masses.len());
    for i in 0..fuel.len() {
        extra_fuel.push(if fuel[i] / 3 > 2 { fuel[i] / 3 - 2 } else { 0 });
        fuel[i] += extra_fuel[i];
    }
    loop {
        let mut should_continue = false;
        for i in 0..fuel.len() {
            extra_fuel[i] = if extra_fuel[i] / 3 > 2 { extra_fuel[i] / 3 - 2 } else { 0 };
            fuel[i] += extra_fuel[i];
            if extra_fuel[i] > 0 {
                should_continue = true;
            }
        }
        if should_continue == false {
            total_fuel = 0;
            for f in fuel.iter() {
                total_fuel += f;
            }
            break;
        }
    }
    println!("{}", total_fuel);
}
