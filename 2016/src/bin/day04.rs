use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let fl = File::open("res/day04/input").expect("couldn't open file");
    let reader = BufReader::new(fl);
    let maybe_rooms: Result<Vec<String>, _> = reader.lines().collect();
    let rooms = maybe_rooms.expect("couldn't read lines from file");
    println!("Day 04");
    let real_rooms = get_real_rooms(&rooms);
    println!("Part 1: {}", part1(&real_rooms));
    println!("Part 2:");
    part2(&real_rooms);
}

fn get_real_rooms(rooms: &[String]) -> Vec<String> {
    let mut real_rooms: Vec<String> = Vec::with_capacity(rooms.len() / 2);
    for room in rooms {
        let mut letter_counts: HashMap<char, usize> = HashMap::new();
        let mut parts = room.split('-').rev();
        let id_checksum = parts.next().unwrap();
        for part in parts {
            part.chars().for_each(|ch| {
                letter_counts
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
        }
        let (_, checksum) = id_checksum.split_once('[').unwrap();
        let checksum = &checksum[..(checksum.len() - 1)];
        let mut prevalence: Vec<usize> = letter_counts.values().copied().collect();
        prevalence.sort_unstable();
        prevalence.reverse();
        let mut is_decoy = false;
        for (idx, ch) in checksum.chars().enumerate() {
            if !letter_counts.contains_key(&ch) {
                is_decoy = true;
                break;
            }
            if letter_counts[&ch] != prevalence[idx] {
                is_decoy = true;
                break;
            }
        }
        if !is_decoy {
            real_rooms.push(room.clone());
        }
    }
    real_rooms
}

fn part1(rooms: &[String]) -> u64 {
    let mut sector_sum = 0;
    for room in rooms {
        let id_checksum = room.split('-').last().unwrap();
        let (sector_id, _) = id_checksum.split_once('[').unwrap();
        let sector_id: u64 = sector_id.parse().expect("couldn't parse sector id as u64");
        sector_sum += sector_id;
    }
    sector_sum
}

fn reverse_shift_cipher(ch: char, shift: u64) -> char {
    let i_a = 'a' as u64;
    let i_ch = ch as u64;
    let shift = shift % 26;
    let i_ch = ((i_ch - i_a) + shift) % 26 + i_a;
    char::from_u32(i_ch as u32).unwrap()
}

fn part2(rooms: &[String]) {
    for room in rooms {
        let mut parts = room.split('-').rev();
        let id_checksum = parts.next().unwrap();
        let (sector_id, _) = id_checksum.split_once('[').unwrap();
        let sector_id: u64 = sector_id.parse().expect("couldn't parse sector id as u64");
        let mut words = Vec::new();
        let mut to_be_printed = false;
        for part in parts {
            let word: String = part
                .chars()
                .map(|ch| reverse_shift_cipher(ch, sector_id))
                .collect();
            to_be_printed = word == "northpole";
            words.push(word);
        }
        if to_be_printed {
            println!("{words:?} id: {sector_id}");
            break;
        }
    }
}
