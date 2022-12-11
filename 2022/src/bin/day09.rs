use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input09")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let moves = parse_input(&input);
    println!("Day 9");
    println!("Part 1: {}", part1(&moves));
    Ok(())
}

type Point = (i64, i64);

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn sub(p1: Point, p2: Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1)
}

fn distance(p1: Point, p2: Point) -> i64 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (direction, magnitude) = line.split_once(' ').unwrap();
            let magnitude: i64 = magnitude.parse().unwrap();
            match direction {
                "R" => (magnitude, 0),
                "L" => (-magnitude, 0),
                "U" => (0, magnitude),
                "D" => (0, -magnitude),
                _ => panic!("unexpected direction"),
            }
        })
        .collect()
}

fn execute_moves(moves: &[Point]) -> HashSet<Point> {
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut t_positions = HashSet::new();
    for m in moves {
        match m {
            (x, 0) => {
                for _ in 0..x.abs() {
                    head_position = add(head_position, (x.signum(), 0));
                    tail_position = adjust_tail(head_position, tail_position);
                    t_positions.insert(tail_position);
                }
            }
            (0, y) => {
                for _ in 0..y.abs() {
                    head_position = add(head_position, (0, y.signum()));
                    tail_position = adjust_tail(head_position, tail_position);
                    t_positions.insert(tail_position);
                }
            }
            _ => unreachable!("move should not be in any other pattern"),
        }
    }
    t_positions
}

fn adjust_tail(head_position: Point, mut tail_position: Point) -> Point {
    if distance(head_position, tail_position) >= 2 {
        let diff = sub(head_position, tail_position);
        match diff {
            (x, 0) => tail_position = add(tail_position, ((x.abs() - 1) * x.signum(), 0)),
            (0, y) => tail_position = add(tail_position, (0, (y.abs() - 1) * y.signum())),
            (x, y) if x.abs() != y.abs() => {
                if x.abs() > y.abs() {
                    tail_position = add(tail_position, ((x.abs() - 1) * x.signum(), y))
                } else {
                    tail_position = add(tail_position, (x, (y.abs() - 1) * y.signum()))
                }
            }
            _ => (),
        }
    }
    tail_position
}

fn part1(moves: &[Point]) -> usize {
    execute_moves(moves).len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{execute_moves, parse_input, part1};

    fn test_input_1() -> String {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            (4, 0),
            (0, 4),
            (-3, 0),
            (0, -1),
            (4, 0),
            (0, -1),
            (-5, 0),
            (2, 0),
        ];
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn tail_positions_test() {
        let input = test_input_1();
        let moves = parse_input(&input);
        let expected = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (3, 3),
            (4, 3),
            (2, 4),
            (3, 4),
        ]);
        assert_eq!(execute_moves(&moves), expected);
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let moves = parse_input(&input);
        assert_eq!(part1(&moves), 13);
    }
}
