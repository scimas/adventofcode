use std::fs;

// struct Passport {
//     byr: String,
//     iyr: String,
//     eyr: String,
//     hgt: String,
//     hcl: String,
//     ecl: String,
//     pid: String,
//     cid: String,
// }

pub fn part1() -> usize {
    let pports = fs::read_to_string("res/input04.txt").expect("Couldn't read day 4 input");
    let req_fields: Vec<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut count: usize = 0;
    for pport in pports.split("\n\n") {
        let mut keys: Vec<String> = Vec::new();
        for item in pport.split_whitespace() {
            keys.push(item.split(':').next().unwrap().to_string());
        }
        count += req_fields.iter().all(|key| keys.contains(key)) as usize;
    }
    count
}

fn is_year_valid(v: &str, min: i32, max: i32) -> bool {
    let vval = v.parse::<i32>();
    match vval {
        Err(_) => false,
        Ok(n) => (min <= n) && (n <= max),
    }
}

fn is_height_valid(v: &str) -> bool {
    if v.len() == 5 {
        if &v[3..] != "cm" {
            false
        } else {
            match v[..3].parse::<u32>() {
                Err(_) => false,
                Ok(n) => (150 <= n) && (n <= 193),
            }
        }
    } else if v.len() == 4 {
        if &v[2..] != "in" {
            false
        } else {
            match v[..2].parse::<u32>() {
                Err(_) => false,
                Ok(n) => (59 <= n) && (n <= 76),
            }
        }
    } else {
        false
    }
}

fn is_hair_color_valid(v: &str) -> bool {
    v.len() == 7 && v[1..].chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_eye_color_valid(v: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)
}

fn is_passport_id_valid(v: &str) -> bool {
    v.len() == 9 && v.chars().all(|ch| ch.is_numeric())
}

pub fn part2() -> usize {
    let pports = fs::read_to_string("res/input04.txt").expect("Couldn't read day 4 input");
    let req_fields: Vec<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut count: usize = 0;
    for pport in pports.split("\n\n") {
        let mut keys: Vec<String> = Vec::new();
        let mut valid = true;
        for item in pport.split_whitespace() {
            let mut kv = item.split(':');
            let k = kv.next().unwrap();
            let v = kv.next().unwrap();
            keys.push(k.to_string());
            match k {
                "byr" => {
                    if !is_year_valid(v, 1920, 2002) {
                        valid = false;
                        break;
                    }
                }
                "iyr" => {
                    if !is_year_valid(v, 2010, 2020) {
                        valid = false;
                        break;
                    }
                }
                "eyr" => {
                    if !is_year_valid(v, 2020, 2030) {
                        valid = false;
                        break;
                    }
                }
                "hgt" => {
                    if !is_height_valid(v) {
                        valid = false;
                        break;
                    }
                }
                "hcl" => {
                    if !is_hair_color_valid(v) {
                        valid = false;
                        break;
                    }
                }
                "ecl" => {
                    if !is_eye_color_valid(v) {
                        valid = false;
                        break;
                    }
                }
                "pid" => {
                    if !is_passport_id_valid(v) {
                        valid = false;
                        break;
                    }
                }
                _ => (),
            }
        }
        if valid {
            count += req_fields.iter().all(|key| keys.contains(key)) as usize;
        }
    }
    count
}

#[test]
fn year_validation_test() {
    assert!(is_year_valid(&"1921", 1920, 2002));
    assert!(is_year_valid(&"2002", 1920, 2002));
    assert!(!is_year_valid(&"2020", 1920, 2002));
    assert!(!is_year_valid(&"abcd", 1920, 2002));
}

#[test]
fn height_validation() {
    assert!(is_height_valid(&"155cm"));
    assert!(is_height_valid(&"60in"));
    assert!(!is_height_valid(&"149cm"));
    assert!(!is_height_valid(&"77in"));
    assert!(!is_height_valid(&"abdcm"));
}

#[test]
fn hair_color_validation() {
    assert!(is_hair_color_valid(&"#a53489"));
    assert!(!is_hair_color_valid(&"#a5348"));
    assert!(!is_hair_color_valid(&"a53489"));
    assert!(!is_hair_color_valid(&"#a5348g"));
}

#[test]
fn eye_color_validation() {
    assert!(is_eye_color_valid(&"brn"));
    assert!(!is_eye_color_valid(&"blk"));
}

#[test]
fn passport_id_validation() {
    assert!(is_passport_id_valid(&"234893450"));
    assert!(is_passport_id_valid(&"034893450"));
    assert!(!is_passport_id_valid(&"23489345"));
    assert!(!is_passport_id_valid(&"23489a450"));
}
