use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {}

fn load_data() -> Vec<Vec<bool>> {
    let f = File::open("res/input03.txt").expect("Couldn't read day 3 input");
    let reader = BufReader::new(f);
    let grid: Vec<Vec<bool>> = reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 3 input")
                .chars()
                .map(|ch| ch == '#')
                .collect()
        })
        .collect();
    grid
}

fn count_trees(slope_right: usize, slope_down: usize, grid: &[Vec<bool>]) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let (mut i, mut j): (usize, usize) = (0, 0);
    let mut count: usize = 0;
    loop {
        i += slope_down;
        j = (j + slope_right) % cols;
        if i >= rows {
            break count;
        }
        count += grid[i][j] as usize;
    }
}

pub fn part1() -> usize {
    let grid = load_data();
    count_trees(3, 1, &grid)
}

pub fn part2() -> usize {
    let grid = load_data();
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tree_product = 1;
    for slope in slopes {
        tree_product *= count_trees(slope.0, slope.1, &grid);
    }
    tree_product
}

#[test]
fn data_loading() {
    let grid = load_data();
    assert_eq!(grid.len(), 323);
}
