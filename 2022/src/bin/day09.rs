use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input09")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let moves = parse_input(&input);
    println!("Day 9");
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
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

fn execute_moves(moves: &[Point], knots: usize) -> HashSet<Point> {
    let mut knot_positions = vec![(0, 0); knots];
    let mut t_positions = HashSet::new();
    for m in moves {
        match m {
            (x, 0) => {
                for _ in 0..x.abs() {
                    knot_positions[0] = add(knot_positions[0], (x.signum(), 0));
                    for i in 1..knots {
                        knot_positions[i] =
                            adjust_knot_pair(knot_positions[i - 1], knot_positions[i]);
                    }
                    t_positions.insert(knot_positions[knots - 1]);
                }
            }
            (0, y) => {
                for _ in 0..y.abs() {
                    knot_positions[0] = add(knot_positions[0], (0, y.signum()));
                    for i in 1..knots {
                        knot_positions[i] =
                            adjust_knot_pair(knot_positions[i - 1], knot_positions[i]);
                    }
                    t_positions.insert(knot_positions[knots - 1]);
                }
            }
            _ => unreachable!("move should not be in any other pattern"),
        }
    }
    t_positions
}

fn adjust_knot_pair(head_position: Point, tail_position: Point) -> Point {
    let mut new_tail = tail_position;
    if distance(head_position, tail_position) >= 2 {
        let diff = sub(head_position, tail_position);
        match diff {
            (x, 0) => new_tail = add(tail_position, ((x.abs() - 1) * x.signum(), 0)),
            (0, y) => new_tail = add(tail_position, (0, (y.abs() - 1) * y.signum())),
            (x, y) if x.abs() != y.abs() => {
                if x.abs() > y.abs() {
                    new_tail = add(tail_position, ((x.abs() - 1) * x.signum(), y))
                } else {
                    new_tail = add(tail_position, (x, (y.abs() - 1) * y.signum()))
                }
            }
            (x, y) => {
                new_tail = add(
                    tail_position,
                    ((x.abs() - 1) * x.signum(), (y.abs() - 1) * y.signum()),
                )
            }
        }
    }
    new_tail
}

fn part1(moves: &[Point]) -> usize {
    execute_moves(moves, 2).len()
}

fn part2(moves: &[Point]) -> usize {
    execute_moves(moves, 10).len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{execute_moves, parse_input, part1, part2};

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

    fn test_input_2() -> String {
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
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
    fn tail_positions_test_1() {
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
        assert_eq!(execute_moves(&moves, 2), expected);
    }

    #[test]
    fn tail_positions_test_2() {
        let input = test_input_2();
        let moves = parse_input(&input);
        let expected = HashSet::from([
            (0, 0),
            (1, 1),
            (2, 2),
            (1, 3),
            (2, 4),
            (3, 5),
            (4, 5),
            (5, 5),
            (6, 4),
            (7, 3),
            (8, 2),
            (9, 1),
            (10, 0),
            (9, -1),
            (8, -2),
            (7, -3),
            (6, -4),
            (5, -5),
            (4, -5),
            (3, -5),
            (2, -5),
            (1, -5),
            (0, -5),
            (-1, -5),
            (-2, -5),
            (-3, -4),
            (-4, -3),
            (-5, -2),
            (-6, -1),
            (-7, 0),
            (-8, 1),
            (-9, 2),
            (-10, 3),
            (-11, 4),
            (-11, 5),
            (-11, 6),
        ]);
        assert_eq!(execute_moves(&moves, 10), expected);
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let moves = parse_input(&input);
        assert_eq!(part1(&moves), 13);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_2();
        let moves = parse_input(&input);
        assert_eq!(part2(&moves), 36);
    }
}
