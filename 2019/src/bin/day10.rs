use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let p1_result = part1();
    part2(p1_result);
}

fn load_data(fname: &str) -> (Vec<Vec<u32>>, Vec<(i32, i32)>) {
    let forest_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    // let forest_text = ".#..#\n.....\n#####\n....#\n...##";
    let forest_text = forest_text.trim();
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let mut trees: Vec<(i32, i32)> = Vec::new();
    let mut row: usize = 0;
    let mut col: usize;
    for line in forest_text.lines() {
        forest.push(Vec::new());
        col = 0;
        for tree in line.chars() {
            if tree == '#' {
                forest[row].push(1);
                trees.push((row as i32, col as i32));
            } else {
                forest[row].push(0);
            }
            col += 1;
        }
        row += 1;
    }
    return (forest, trees);
}

fn transform_coords(origin: &(i32, i32), points: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut new_points: Vec<(i32, i32)> = Vec::new();
    for p in points {
        new_points.push((p.0 - origin.0, p.1 - origin.1));
    }
    return new_points;
}

fn to_polar(point: &(i32, i32)) -> (f32, f32) {
    let r = (point.0.pow(2) + point.1.pow(2)) as f32;
    let theta = (-1f32 * point.0 as f32).atan2(point.1 as f32);

    return (r.sqrt(), theta);
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut t: i32;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn reduce(point: &(i32, i32)) -> (i32, i32) {
    let denom = gcd(point.0.abs(), point.1.abs());
    return (point.0 / denom, point.1 / denom);
}

fn part1() -> HashSet<(i32, i32)> {
    let (field, asteroids) = load_data("data/Day10_input.txt");
    let mut vision_field: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let colmax = field[0].len() as i32;
    let rowmax = field.len() as i32;

    for ast in asteroids.iter() {
        let mut found_up = false;
        let mut found_down = false;
        let mut found_left = false;
        let mut found_right = false;
        let new_asteroids = transform_coords(ast, &asteroids);
        for other_ast in new_asteroids.iter() {
            if other_ast.0 == 0 && other_ast.1 == 0 {
                continue;
            } else if !found_up && other_ast.1 == 0 && other_ast.0 < 0 {
                let my_vision = vision_field.entry(*ast).or_insert(HashSet::new());
                my_vision.insert((0, -rowmax - 1));
                found_up = true;
            } else if !found_down && other_ast.1 == 0 && other_ast.0 > 0 {
                let my_vision = vision_field.entry(*ast).or_insert(HashSet::new());
                my_vision.insert((0, rowmax + 1));
                found_down = true;
            } else if !found_left && other_ast.0 == 0 && other_ast.1 < 0 {
                let my_vision = vision_field.entry(*ast).or_insert(HashSet::new());
                my_vision.insert((-colmax - 1, 0));
                found_left = true;
            } else if !found_right && other_ast.0 == 0 && other_ast.1 > 0 {
                let my_vision = vision_field.entry(*ast).or_insert(HashSet::new());
                my_vision.insert((colmax + 1, 0));
                found_right = true;
            } else if found_up && found_down && found_left && found_right {
                break;
            }
        }
        for other_ast in new_asteroids.iter() {
            if other_ast.0 == 0 || other_ast.1 == 0 {
                continue;
            } else {
                let my_vision = vision_field.entry(*ast).or_insert(HashSet::new());
                my_vision.insert(reduce(other_ast));
            }
        }
    }
    let mut vision_counts: HashMap<(i32, i32), usize> = HashMap::new();
    for &k in vision_field.keys() {
        vision_counts
            .entry(k)
            .or_insert(vision_field.get(&k).unwrap().len());
    }
    let mut max_key: (i32, i32) = (0, 0);
    let mut last_count: usize = 0;
    for &k in vision_counts.keys() {
        let &this_count = vision_counts.get(&k).unwrap();
        if this_count > last_count {
            last_count = this_count;
            max_key = k;
        }
    }
    // println!("{:?}", max_key);
    // println!("{:?}", vision_field.get(&max_key).unwrap());
    println!("{:?}", max_key);
    println!("{}", vision_counts.get(&max_key).unwrap());
    return vision_field.get(&max_key).unwrap().clone();
}

fn part2(vision_field: HashSet<(i32, i32)>) {
    let mut vision_field: Vec<(f32, f32)> = vision_field.iter().map(|k| to_polar(k)).collect();
    vision_field.sort_by(|k1, k2| k1.1.partial_cmp(&k2.1).unwrap());
    vision_field.reverse();
    let mut cycle_it = vision_field.iter().cycle();
    loop {
        match cycle_it.next().unwrap().1 <= std::f32::consts::PI / 2.0 {
            true => break,
            false => (),
        }
    }
    for _ in 0..198 {
        cycle_it.next();
    }
    let two_hundredth = cycle_it.next().unwrap();
    println!(
        "{:?}",
        (
            two_hundredth.0 * two_hundredth.1.cos(),
            two_hundredth.0 * two_hundredth.1.sin()
        )
    );
}
