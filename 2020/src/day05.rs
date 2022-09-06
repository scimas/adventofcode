use std::fs::File;
use std::io::{BufRead, BufReader};

fn string_to_seat(s: &str) -> (usize, usize, usize) {
    let mut row: usize = 0;
    let mut column: usize = 0;
    for (idx, ch) in s.char_indices() {
        if idx < 7 {
            if ch == 'B' {
                row += 2usize.pow(6 - idx as u32);
            }
        } else if ch == 'R' {
            column += 2usize.pow(9 - idx as u32);
        }
    }
    (row, column, row * 8 + column)
}

fn load_data() -> Vec<(usize, usize, usize)> {
    let f = File::open("res/input05.txt").expect("Couldn't read day 5 input");
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|line| string_to_seat(&line.expect("Couldn't read line from day 5 input")))
        .collect()
}

pub fn part1() -> usize {
    let seats = load_data();
    match seats.iter().max_by_key(|seat| seat.2) {
        Some(seat) => seat.2,
        None => panic!("Couldn't find highest seat id"),
    }
}

pub fn part2() -> usize {
    let mut seats = load_data();
    seats.sort_by_key(|(_, _, seat_id)| *seat_id);
    let want_seats = seats[1..]
        .iter()
        .zip(seats.iter())
        .find(|((_, _, sidx1), (_, _, sidx2))| *sidx1 - *sidx2 != 1);
    match want_seats {
        Some(seat) => seat.0 .2 - 1,
        None => panic!("Couldn't find my seat id"),
    }
}

#[test]
fn correct_str_seat_conversion() {
    let examples: Vec<String> = ["FBBBFFFRRL", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let answers: Vec<(usize, usize, usize)> =
        vec![(56, 6, 454), (70, 7, 567), (14, 7, 119), (102, 4, 820)];
    examples
        .iter()
        .zip(answers.iter())
        .for_each(|(ex, ans)| assert_eq!(string_to_seat(ex), *ans));
}
