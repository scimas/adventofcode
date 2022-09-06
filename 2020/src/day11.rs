use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_seats() -> Vec<Vec<i32>> {
    let f = File::open("res/input11.txt").expect("Couldn't open day 11 input");
    let reader = BufReader::new(f);
    let mut grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 11 input")
                .chars()
                .map(|ch| if ch == 'L' { 0 } else { -1 })
                .collect()
        })
        .collect();
    let mut dummy_row: Vec<i32> = Vec::with_capacity(grid[0].len() + 2);
    for _ in 0..grid[0].len() + 2 {
        dummy_row.push(-1);
    }
    grid.insert(0, dummy_row.to_vec());
    grid.push(dummy_row.to_vec());
    for i in 1..grid.len() - 1 {
        grid[i].insert(0, -1);
        grid[i].push(-1);
    }
    grid
}

fn update_grid_part1(grid: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, bool) {
    let mut new_grid = grid.clone();
    let mut changed = false;
    let cols = grid[0].len() - 1;
    for row in 1..grid.len() - 1 {
        for col in 1..cols {
            if grid[row][col] == -1 {
                continue;
            }
            let mut neighbors: i32 = 0;
            for i in row - 1..=row + 1 {
                for j in col - 1..=col + 1 {
                    if i != row || j != col {
                        neighbors += (grid[i][j] == 1) as i32;
                    }
                }
            }
            if grid[row][col] == 0 {
                if neighbors == 0 {
                    new_grid[row][col] = 1;
                    changed = true;
                }
            } else if grid[row][col] == 1 && neighbors >= 4 {
                new_grid[row][col] = 0;
                changed = true
            }
        }
    }
    (new_grid, changed)
}

pub fn part1() -> usize {
    let mut grid = load_seats();
    let mut changed = true;
    while changed {
        let res = update_grid_part1(grid);
        grid = res.0;
        changed = res.1;
    }
    grid.iter()
        .fold(0, |acc, x| acc + x.iter().filter(|&s| *s == 1).count())
}

fn update_grid_part2(grid: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, bool) {
    let mut new_grid = grid.clone();
    let mut changed = false;
    let cols = grid[0].len() - 1;
    let rows = grid.len() - 1;
    for row in 1..rows {
        for col in 1..cols {
            if grid[row][col] == -1 {
                continue;
            }
            let mut neighbors: i32 = 0;
            // North
            for i in (1..row).rev() {
                if grid[i][col] == 0 {
                    break;
                } else if grid[i][col] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // South
            for i in row + 1..rows {
                if grid[i][col] == 0 {
                    break;
                } else if grid[i][col] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // West
            for j in (1..col).rev() {
                if grid[row][j] == 0 {
                    break;
                } else if grid[row][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // East
            for j in col + 1..cols {
                if grid[row][j] == 0 {
                    break;
                } else if grid[row][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // North West
            for (i, j) in (1..row).rev().zip((1..col).rev()) {
                if grid[i][j] == 0 {
                    break;
                } else if grid[i][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // South West
            for (i, j) in (row + 1..rows).zip((1..col).rev()) {
                if grid[i][j] == 0 {
                    break;
                } else if grid[i][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // North East
            for (i, j) in (1..row).rev().zip(col + 1..cols) {
                if grid[i][j] == 0 {
                    break;
                } else if grid[i][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            // South East
            for (i, j) in (row + 1..rows).zip(col + 1..cols) {
                if grid[i][j] == 0 {
                    break;
                } else if grid[i][j] == 1 {
                    neighbors += 1;
                    break;
                }
            }
            if grid[row][col] == 0 {
                if neighbors == 0 {
                    new_grid[row][col] = 1;
                    changed = true;
                }
            } else if grid[row][col] == 1 && neighbors >= 5 {
                new_grid[row][col] = 0;
                changed = true;
            }
        }
    }
    (new_grid, changed)
}

pub fn part2() -> usize {
    let mut grid = load_seats();
    let mut changed = true;
    while changed {
        let res = update_grid_part2(grid);
        grid = res.0;
        changed = res.1;
    }
    grid.iter()
        .fold(0, |acc, x| acc + x.iter().filter(|&s| *s == 1).count())
}
