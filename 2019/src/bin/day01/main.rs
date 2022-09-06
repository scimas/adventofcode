use std::fs;
mod fuel;

fn main() {
    part1();
    part2();
}

fn load_data(fname: &str) -> Vec<u32> {
    let mass_text = fs::read_to_string(fname).expect("Error reading file!");
    let masses: Vec<u32> = mass_text
        .split_whitespace()
        .map(|m| m.parse::<u32>().expect("Couldn't parse mass from text!"))
        .collect();
    masses
}

fn part1() {
    let masses: Vec<u32> = load_data("data/Day01_input.txt");
    let fuel: Vec<u32> = fuel::module_fuel(&masses);
    println!("{}", fuel.iter().sum::<u32>());
}

fn part2() {
    let masses: Vec<u32> = load_data("data/Day01_input.txt");
    let my_fuel: Vec<u32> = fuel::module_fuel(&masses);
    let extra_fuel: Vec<u32> = fuel::fuel_fuel(my_fuel);
    println!("{}", extra_fuel.iter().sum::<u32>());
}
