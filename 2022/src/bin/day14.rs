use std::{collections::HashMap, fs::File, io::Read};

use adventofcode_2022::Position;

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input14")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let mut cave = parse_input(&input);
    println!("Day 14");
    println!("Part 1: {}", part1(&mut cave.clone()));
    println!("Part 2: {}", part2(&mut cave));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cave {
    tiles: HashMap<Position, Tile>,
    bounds: Bounds,
    floor: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    bottom: i64,
    left: i64,
    right: i64,
}

impl Cave {
    fn drop_sand(&mut self, from: Position) {
        self.tiles.insert(from, Tile::Sand);
    }

    fn move_sand(&mut self, from: &Position) -> Position {
        let down = Position::new(from.row + 1, from.col);
        let down_left = Position::new(from.row + 1, from.col - 1);
        let down_right = Position::new(from.row + 1, from.col + 1);
        if self.get(&down).is_none() {
            self.tiles.insert(down, Tile::Sand);
            self.tiles.remove(from);
            down
        } else if self.get(&down_left).is_none() {
            self.tiles.insert(down_left, Tile::Sand);
            self.tiles.remove(from);
            down_left
        } else if self.get(&down_right).is_none() {
            self.tiles.insert(down_right, Tile::Sand);
            self.tiles.remove(from);
            down_right
        } else {
            *from
        }
    }

    fn is_out_of_bounds(&self, position: &Position) -> bool {
        self.bounds.bottom < position.row
    }

    fn add_floor(&mut self, at_row: i64) {
        self.floor = Some(at_row);
    }

    fn get(&mut self, position: &Position) -> Option<&Tile> {
        if let Some(floor) = self.floor {
            if position.row == floor {
                self.tiles.entry(*position).or_insert(Tile::Rock);
            }
        }
        self.tiles.get(position)
    }
}

fn parse_input(input: &str) -> Cave {
    let mut tiles = HashMap::new();
    let mut bottom = i64::MIN;
    let mut left = i64::MAX;
    let mut right = i64::MIN;
    for line in input.lines() {
        let mut points = line.split(" -> ");
        let mut p1 = parse_position(points.next().unwrap());
        for point in points {
            let p2 = parse_position(point);
            bottom = bottom.max(p1.row).max(p2.row);
            left = left.min(p1.col).min(p2.col);
            right = right.max(p1.col).max(p2.col);
            if p1.row == p2.row {
                tiles.extend(
                    (p1.col.min(p2.col)..=p1.col.max(p2.col))
                        .map(|col| (Position::new(p1.row, col), Tile::Rock)),
                );
            } else {
                tiles.extend(
                    (p1.row.min(p2.row)..=p1.row.max(p2.row))
                        .map(|row| (Position::new(row, p1.col), Tile::Rock)),
                );
            }
            p1 = p2;
        }
    }
    Cave {
        tiles,
        bounds: Bounds {
            bottom,
            left,
            right,
        },
        floor: None,
    }
}

fn parse_position(s: &str) -> Position {
    let (col_s, row_s) = s.split_once(',').unwrap();
    Position::new(row_s.parse().unwrap(), col_s.parse().unwrap())
}

fn part1(cave: &mut Cave) -> usize {
    let mut count = 0;
    let mut from_pos = Position::new(0, 500);
    while !cave.is_out_of_bounds(&from_pos) {
        let start_position = Position::new(0, 500);
        from_pos = start_position;
        cave.drop_sand(start_position);
        count += 1;
        loop {
            let to_pos = cave.move_sand(&from_pos);
            if to_pos == from_pos || cave.is_out_of_bounds(&to_pos) {
                from_pos = to_pos;
                break;
            }
            from_pos = to_pos;
        }
    }
    count - 1
}

fn part2(cave: &mut Cave) -> usize {
    cave.add_floor(cave.bounds.bottom + 2);
    let mut count = 0;
    let mut from_pos = Position::new(1, 500);
    while from_pos != Position::new(0, 500) {
        let start_position = Position::new(0, 500);
        from_pos = start_position;
        cave.drop_sand(start_position);
        count += 1;
        loop {
            let to_pos = cave.move_sand(&from_pos);
            if to_pos == from_pos {
                from_pos = to_pos;
                break;
            }
            from_pos = to_pos;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use adventofcode_2022::Position;

    use crate::{parse_input, part1, part2, Bounds, Cave, Tile};

    fn test_input_1() -> String {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = Cave {
            tiles: HashMap::from([
                (Position::new(4, 498), Tile::Rock),
                (Position::new(5, 498), Tile::Rock),
                (Position::new(6, 498), Tile::Rock),
                (Position::new(6, 497), Tile::Rock),
                (Position::new(6, 496), Tile::Rock),
                (Position::new(4, 503), Tile::Rock),
                (Position::new(4, 502), Tile::Rock),
                (Position::new(5, 502), Tile::Rock),
                (Position::new(6, 502), Tile::Rock),
                (Position::new(7, 502), Tile::Rock),
                (Position::new(8, 502), Tile::Rock),
                (Position::new(9, 502), Tile::Rock),
                (Position::new(9, 501), Tile::Rock),
                (Position::new(9, 500), Tile::Rock),
                (Position::new(9, 499), Tile::Rock),
                (Position::new(9, 498), Tile::Rock),
                (Position::new(9, 497), Tile::Rock),
                (Position::new(9, 496), Tile::Rock),
                (Position::new(9, 495), Tile::Rock),
                (Position::new(9, 494), Tile::Rock),
            ]),
            bounds: Bounds {
                bottom: 9,
                left: 494,
                right: 503,
            },
            floor: None,
        };
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let mut cave = parse_input(&input);
        assert_eq!(part1(&mut cave), 24);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let mut cave = parse_input(&input);
        assert_eq!(part2(&mut cave), 93);
    }
}
