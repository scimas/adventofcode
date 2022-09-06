use std::{collections::HashMap, fs::File, io::Read, time::Instant};

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let mut f;
    if let Some(fpath) = args.nth(1) {
        let base_path = std::env::current_dir()
            .map_err(|_| "could not resolve current directory".to_string())?;
        f = File::open(base_path.join(&fpath))
            .map_err(|_| format!(r#"could not open file "{fpath}""#))?;
    } else {
        return Err("incorrect number of arguments".into());
    }
    let mut ages_s = String::new();
    f.read_to_string(&mut ages_s).unwrap();
    let mut age_map: HashMap<u8, usize> = HashMap::new();
    ages_s.trim().split(',').for_each(|num| {
        let age = num.parse().unwrap();
        age_map
            .entry(age)
            .and_modify(|cnt| *cnt += 1)
            .or_insert_with(|| 1);
    });
    for age in 0..9 {
        age_map.entry(age).or_insert(0);
    }
    println!("Day 06");

    let start_time = Instant::now();
    let p1_fish_count = solve(age_map.clone(), 80, 7, 9);
    let duration = start_time.elapsed();
    println!(
        "Part 1: {}, time: {:.3e} s",
        p1_fish_count,
        duration.as_secs_f32()
    );

    let start_time = Instant::now();
    let p2_fish_count = solve(age_map.clone(), 256, 7, 9);
    let duration = start_time.elapsed();
    println!(
        "Part 2: {}, time: {:.3e} s",
        p2_fish_count,
        duration.as_secs_f32()
    );
    Ok(())
}

fn solve(mut age_map: HashMap<u8, usize>, days: usize, generation: u8, growth: u8) -> usize {
    let mut cur_day = 0;
    while cur_day < days {
        cur_day += 1;
        let mut new_ages = HashMap::with_capacity(growth as usize);
        new_ages.insert(growth - 1, age_map[&0]);
        for age in (generation..growth - 1).rev() {
            new_ages.insert(age, age_map[&(age + 1)]);
        }
        new_ages.insert(generation - 1, age_map[&generation] + age_map[&0]);
        for age in (0..generation - 1).rev() {
            new_ages.insert(age, age_map[&(age + 1)]);
        }
        age_map = new_ages;
    }
    age_map.values().sum()
}
