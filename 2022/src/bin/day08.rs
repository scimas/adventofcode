use std::{fs::File, io::Read, ops::ControlFlow};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input08")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let forest = parse_input(&input);
    println!("Day 8");
    println!("Part 1: {}", part1(&forest));
    println!("Part 2: {}", part2(&forest));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Forest {
    columns: usize,
    rows: usize,
    trees: Vec<u8>,
}

impl Forest {
    fn new(columns: usize, rows: usize, trees: Vec<u8>) -> Self {
        Self {
            columns,
            rows,
            trees,
        }
    }

    fn row(&self, row: usize) -> impl Iterator<Item = &u8> {
        let first = self.columns * row;
        self.trees.iter().skip(first).take(self.columns)
    }

    fn column(&self, column: usize) -> impl Iterator<Item = &u8> {
        let step = self.columns;
        self.trees.iter().skip(column).step_by(step)
    }

    fn is_visible(&self, row: usize, column: usize) -> bool {
        assert!(row < self.rows && column < self.columns);
        if row == 0 || column == 0 || row == self.rows - 1 || column == self.columns - 1 {
            return true;
        }
        let tree_height = self.trees[self.columns * row + column];
        #[rustfmt::skip]
        let visible = self.row(row).take(column).all(|h| h < &tree_height)       ||  // visible from left
                            self.row(row).skip(column + 1).all(|h| h < &tree_height)   ||  // visible from right
                            self.column(column).take(row).all(|h| h < &tree_height)    ||  // visible from top
                            self.column(column).skip(row + 1).all(|h| h < &tree_height); // visible from bottom
        visible
    }

    fn scenic_score(&self, row: usize, column: usize) -> usize {
        assert!(row < self.rows && column < self.columns);
        if row == 0 || column == 0 || row == self.rows - 1 || column == self.columns - 1 {
            return 0;
        }
        let tree_height = self.trees[self.columns * row + column];
        let left_score =
            self.row(row)
                .take(column)
                .fold(0, |count, h| if h < &tree_height { count + 1 } else { 1 });
        let right_score = match self.row(row).skip(column + 1).try_fold(0, |count, h| {
            if h < &tree_height {
                ControlFlow::Continue(count + 1)
            } else {
                ControlFlow::Break(count + 1)
            }
        }) {
            ControlFlow::Continue(count) => count,
            ControlFlow::Break(count) => count,
        };
        let up_score =
            self.column(column)
                .take(row)
                .fold(0, |count, h| if h < &tree_height { count + 1 } else { 1 });
        let down_score = match self.column(column).skip(row + 1).try_fold(0, |count, h| {
            if h < &tree_height {
                ControlFlow::Continue(count + 1)
            } else {
                ControlFlow::Break(count + 1)
            }
        }) {
            ControlFlow::Continue(count) => count,
            ControlFlow::Break(count) => count,
        };
        left_score * right_score * up_score * down_score
    }
}

fn parse_input(input: &str) -> Forest {
    let columns = input.chars().take_while(|ch| !ch.is_whitespace()).count();
    let trees: Vec<u8> = input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();
    let rows = trees.len() / columns;
    Forest::new(columns, rows, trees)
}

fn part1(forest: &Forest) -> usize {
    let mut count = 0;
    for i in 0..forest.rows {
        for j in 0..forest.columns {
            count += usize::from(forest.is_visible(i, j));
        }
    }
    count
}

fn part2(forest: &Forest) -> usize {
    let mut scenic_score = 0;
    for i in 0..forest.rows {
        for j in 0..forest.columns {
            scenic_score = scenic_score.max(forest.scenic_score(i, j));
        }
    }
    scenic_score
}
#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2, Forest};

    fn test_input_1() -> String {
        "30373
25512
65332
33549
35390
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        #[rustfmt::skip]
        let expected = Forest::new(
            5,
            5,
            vec![
                3, 0, 3, 7, 3,
                2, 5, 5, 1, 2,
                6, 5, 3, 3, 2,
                3, 3, 5, 4, 9,
                3, 5, 3, 9, 0,
            ],
        );
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn visibility_test() {
        let input = test_input_1();
        let forest = parse_input(&input);
        #[rustfmt::skip]
        let expected = vec![
            true, true, true, true, true,
            true, true, true, false, true,
            true, true, false, true, true,
            true, false, true, false, true,
            true, true, true, true, true,
        ];
        for i in 0..forest.rows {
            for j in 0..forest.columns {
                assert_eq!(
                    forest.is_visible(i, j),
                    expected[forest.columns * i + j],
                    "problem at ({i}, {j})"
                );
            }
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let forest = parse_input(&input);
        assert_eq!(part1(&forest), 21);
    }

    #[test]
    fn scenic_score_test() {
        let input = test_input_1();
        let forest = parse_input(&input);
        #[rustfmt::skip]
        let expected = vec![
            0, 0, 0, 0, 0,
            0, 1, 4, 1, 0,
            0, 6, 1, 2, 0,
            0, 1, 8, 3, 0,
            0, 0, 0, 0, 0,
        ];
        for i in 3..forest.rows {
            for j in 2..forest.columns {
                assert_eq!(
                    forest.scenic_score(i, j),
                    expected[forest.columns * i + j],
                    "problem at ({i}, {j})"
                );
            }
        }
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let forest = parse_input(&input);
        assert_eq!(part2(&forest), 8);
    }
}
