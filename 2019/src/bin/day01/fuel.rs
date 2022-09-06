pub fn module_fuel(masses: &[u32]) -> Vec<u32> {
    masses.iter().map(|m| *m / 3 - 2).collect()
}

pub fn fuel_fuel(mut fuel: Vec<u32>) -> Vec<u32> {
    let mut extra_fuel: Vec<u32> = fuel
        .iter()
        .map(|&f| if f > 6 { f / 3 - 2 } else { 0 })
        .collect();
    fuel = fuel
        .iter()
        .zip(extra_fuel.iter())
        .map(|(&f1, &f2)| f1 + f2)
        .collect();
    loop {
        extra_fuel = extra_fuel
            .iter()
            .map(|&f| if f > 6 { f / 3 - 2 } else { 0 })
            .collect();
        if let Some(&0) = extra_fuel.iter().max() {
            break
        }
        fuel = fuel
            .iter()
            .zip(extra_fuel.iter())
            .map(|(&f1, &f2)| f1 + f2)
            .collect();
    }
    fuel
}
