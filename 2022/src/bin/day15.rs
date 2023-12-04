// use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use adventofcode_2022::Position;

fn main() -> Result<(), anyhow::Error> {
    let fl = File::open("resources/input15")?;
    let reader = BufReader::new(fl);
    let sensor_beacon_pairs = parse_input(reader);
    let impossible_positions = part1(&sensor_beacon_pairs, 2_000_000);
    println!("Part 1: {}", impossible_positions.len());
    // let beacon_position = part2(&sensor_beacon_pairs, 4_000_000).unwrap();
    // println!(
    //     "Part 2: {}",
    //     4_000_000 * beacon_position.col + beacon_position.row
    // );
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beacon {
    position: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Sensor {
    position: Position,
}

fn part1(sensor_beacon_pairs: &HashMap<(Sensor, Beacon), u64>, row: i64) -> HashSet<Position> {
    let mut impossible_positions = HashSet::new();
    for ((sensor, beacon), sensor_beacon_distance) in sensor_beacon_pairs {
        let sensor_row_min_distance = sensor.position.row.abs_diff(row);
        if sensor_row_min_distance <= *sensor_beacon_distance {
            let spread_distance = (sensor_beacon_distance - sensor_row_min_distance) as i64;
            for col_diff in -spread_distance..=spread_distance {
                let impossible_position = Position::new(row, sensor.position.col + col_diff);
                impossible_positions.insert(impossible_position);
            }
        }
        impossible_positions.remove(&beacon.position);
    }
    impossible_positions
}

// fn part2(sensor_beacon_pairs: &HashSet<(Sensor, Beacon)>, limit: u64) -> Option<Position> {
//     (0..=limit).into_par_iter().find_map_any(|row| {
//         let mut impossible_positions = part1(sensor_beacon_pairs, row as i64);
//         sensor_beacon_pairs.iter().for_each(|(_, beacon)| {
//             if beacon.position.row == row as i64 {
//                 impossible_positions.insert(beacon.position);
//             }
//         });
//         let impossible_cols: HashSet<i64> = impossible_positions
//             .into_par_iter()
//             .map(|position| position.col)
//             .collect();
//         (0..=limit as i64).into_par_iter().find_map_any(|col| {
//             if !impossible_cols.contains(&col) {
//                 Some(Position::new(row as i64, col))
//             } else {
//                 None
//             }
//         })
//     })
// }

fn parse_input<R: BufRead>(input: R) -> HashMap<(Sensor, Beacon), u64> {
    let mut sensor_beacon_pairs = HashMap::new();
    for maybe_line in input.lines() {
        let (sensor, beacon) = parse_input_line(&maybe_line.unwrap());
        sensor_beacon_pairs.insert(
            (sensor, beacon),
            sensor.position.manhattan_distance(beacon.position),
        );
    }
    sensor_beacon_pairs
}

fn parse_input_line(line: &str) -> (Sensor, Beacon) {
    // expected line format
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let mut tokens = line.split_whitespace().skip(2);
    let sensor_x: i64 = tokens
        .next()
        .unwrap()
        .trim_start_matches("x=")
        .trim_end_matches(',')
        .parse()
        .unwrap();
    let sensor_y: i64 = tokens
        .next()
        .unwrap()
        .trim_start_matches("y=")
        .trim_end_matches(':')
        .parse()
        .unwrap();
    let mut tokens = tokens.skip(4);
    let beacon_x: i64 = tokens
        .next()
        .unwrap()
        .trim_start_matches("x=")
        .trim_end_matches(',')
        .parse()
        .unwrap();
    let beacon_y: i64 = tokens
        .next()
        .unwrap()
        .trim_start_matches("y=")
        .parse()
        .unwrap();
    (
        Sensor {
            position: Position::new(sensor_y, sensor_x),
        },
        Beacon {
            position: Position::new(beacon_y, beacon_x),
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    fn test_input_1() -> String {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3\n"
            .to_string()
    }

    #[test]
    fn part1_test_1() {
        let sensor_beacon_pairs = parse_input(test_input_1().as_bytes());
        let impossible_positions = part1(&sensor_beacon_pairs, 10);
        assert_eq!(impossible_positions.len(), 26);
    }
}
