use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {}

fn load_data() -> Vec<Vec<u8>> {
    let f = File::open("res/input17.txt").expect("Couldn't open day 17 input");
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| {
            line.expect("Couldn't read line from day 17 input")
                .chars()
                .map(|ch| {
                    if ch == '#' {
                        1
                    } else if ch == '.' {
                        0
                    } else {
                        unreachable!()
                    }
                })
                .collect()
        })
        .collect()
}

fn sparse_cube_from_grid(grid: &[Vec<u8>]) -> HashMap<(isize, isize, isize), u8> {
    let mut sprs_cube: HashMap<(isize, isize, isize), u8> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 1 {
                sprs_cube.insert((i as isize, j as isize, 0), 1);
            }
        }
    }
    sprs_cube
}

fn sparse_cube_simulate_cycles(
    cycles: usize,
    sprs_cube: &HashMap<(isize, isize, isize), u8>,
    gridlen: usize,
) -> HashMap<(isize, isize, isize), u8> {
    let mut sprs_cube = sprs_cube.clone();
    for c in 1..=(cycles as isize) {
        let mut next_cube: HashMap<(isize, isize, isize), u8> = HashMap::new();
        for k in -c..=c {
            for i in -c..(gridlen as isize + c) {
                for j in -c..(gridlen as isize + c) {
                    let mut neighbors = 0u8;
                    for kk in (k - 1)..=(k + 1) {
                        for ii in (i - 1)..=(i + 1) {
                            for jj in (j - 1)..=(j + 1) {
                                if (ii == i) && (jj == j) && (kk == k) {
                                    continue;
                                }
                                neighbors += sprs_cube.get(&(ii, jj, kk)).unwrap_or(&0);
                            }
                        }
                    }
                    if sprs_cube.get(&(i, j, k)).is_some() {
                        if neighbors == 2 || neighbors == 3 {
                            next_cube.insert((i, j, k), 1);
                        }
                    } else if neighbors == 3 {
                        next_cube.insert((i, j, k), 1);
                    }
                }
            }
        }
        sprs_cube = next_cube.clone();
    }
    sprs_cube
}

// fn cube_from_grid(cycles: usize, grid: &[Vec<u8>]) -> Vec<Vec<Vec<u8>>> {
//     let cube_side = grid.len() + 2 * (cycles + 1);
//     let mut cube = vec![vec![vec![0u8; cube_side]; cube_side]; 2 * (cycles + 1) + 1];
//     for i in (cycles + 1)..(cycles + 1 + grid.len()) {
//         for j in (cycles + 1)..(cycles + 1 + grid.len()) {
//             cube[cycles + 1][i][j] = grid[i - (cycles + 1)][j - (cycles + 1)];
//         }
//     }
//     cube
// }

// fn cube_simulate_cycles(cycles: usize, grid: &[Vec<u8>]) -> Vec<Vec<Vec<u8>>> {
//     let cube_side = grid.len() + 2 * (cycles + 1);
//     let mut cube = cube_from_grid(cycles, grid);
//     for c in 1..=cycles {
//         let mut next_cube = vec![vec![vec![0u8; cube_side]; cube_side]; 2 * (cycles + 1) + 1];
//         for k in (cycles + 1 - c)..(cycles + 2 + c) {
//             for i in (cycles + 1 - c)..(cycles + 1 + grid.len() + c) {
//                 for j in (cycles + 1 - c)..(cycles + 1 + grid.len() + c) {
//                     let mut neighbors = 0u8;
//                     for kk in (k - 1)..=(k + 1) {
//                         for ii in (i - 1)..=(i + 1) {
//                             for jj in (j - 1)..=(j + 1) {
//                                 if (ii == i) && (jj == j) && (kk == k) {
//                                     continue;
//                                 }
//                                 neighbors += cube[kk][ii][jj];
//                             }
//                         }
//                     }
//                     if cube[k][i][j] == 1 {
//                         if neighbors == 2 || neighbors == 3 {
//                             next_cube[k][i][j] = 1;
//                         } else {
//                             next_cube[k][i][j] = 0;
//                         }
//                     } else if cube[k][i][j] == 0 {
//                         if neighbors == 3 {
//                             next_cube[k][i][j] = 1;
//                         } else {
//                             next_cube[k][i][j] = 0;
//                         }
//                     } else {
//                         unreachable!();
//                     }
//                 }
//             }
//         }
//         cube = next_cube.clone();
//     }
//     cube
// }

pub fn part1() -> usize {
    let grid = load_data();
    // let cube = cube_simulate_cycles(6, &grid);
    // cube.iter().fold(0, |acc, x| {
    //     acc + x.iter().fold(0, |accx, y| accx + y.iter().sum::<u8>())
    // })
    let sprs_cube = sparse_cube_from_grid(&grid);
    let sprs_cube = sparse_cube_simulate_cycles(6, &sprs_cube, grid.len());
    sprs_cube.len()
}

fn sparse_hypercube_from_grid(grid: &[Vec<u8>]) -> HashMap<(isize, isize, isize, isize), u8> {
    let mut sprs_cube: HashMap<(isize, isize, isize, isize), u8> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 1 {
                sprs_cube.insert((i as isize, j as isize, 0, 0), 1);
            }
        }
    }
    sprs_cube
}

fn sparse_hypercube_simulate_cycles(
    cycles: usize,
    sprs_hcube: &HashMap<(isize, isize, isize, isize), u8>,
    gridlen: usize,
) -> HashMap<(isize, isize, isize, isize), u8> {
    let mut sprs_hcube = sprs_hcube.clone();
    for c in 1..=(cycles as isize) {
        let mut next_hcube: HashMap<(isize, isize, isize, isize), u8> = HashMap::new();
        for l in -c..=c {
            for k in -c..=c {
                for i in -c..(gridlen as isize + c) {
                    for j in -c..(gridlen as isize + c) {
                        let mut neighbors = 0u8;
                        for ll in (l - 1)..=(l + 1) {
                            for kk in (k - 1)..=(k + 1) {
                                for ii in (i - 1)..=(i + 1) {
                                    for jj in (j - 1)..=(j + 1) {
                                        if (ii == i) && (jj == j) && (kk == k) && (ll == l) {
                                            continue;
                                        }
                                        neighbors +=
                                            sprs_hcube.get(&(ii, jj, kk, ll)).unwrap_or(&0);
                                    }
                                }
                            }
                        }
                        if sprs_hcube.get(&(i, j, k, l)).is_none() {
                            if neighbors == 2 || neighbors == 3 {
                                next_hcube.insert((i, j, k, l), 1);
                            }
                        } else if neighbors == 3 {
                            next_hcube.insert((i, j, k, l), 1);
                        }
                    }
                }
            }
        }
        sprs_hcube = next_hcube.clone();
    }
    sprs_hcube
}

pub fn part2() -> usize {
    let grid = load_data();
    let sprs_hcube = sparse_hypercube_from_grid(&grid);
    let sprs_hcube = sparse_hypercube_simulate_cycles(6, &sprs_hcube, grid.len());
    sprs_hcube.len()
}

#[cfg(test)]
mod test {
    use super::*;

    // fn print_cube(cube: &[Vec<Vec<u8>>]) {
    //     for plane in cube {
    //         for row in plane {
    //             println!("{:?}", row);
    //         }
    //         println!();
    //     }
    // }

    // #[test]
    // fn post_dense_sim_count() {
    //     let s = String::from(".#.\n..#\n###");
    //     let grid: Vec<Vec<u8>> = s
    //         .lines()
    //         .map(|line| {
    //             line.chars()
    //                 .map(|ch| {
    //                     if ch == '#' {
    //                         1
    //                     } else if ch == '.' {
    //                         0
    //                     } else {
    //                         unreachable!()
    //                     }
    //                 })
    //                 .collect()
    //         })
    //         .collect();
    //     let cube = cube_simulate_cycles(1, &grid);
    //     let count = cube.iter().fold(0, |acc, x| {
    //         acc + x.iter().fold(0, |accx, y| accx + y.iter().sum::<u8>())
    //     });
    //     assert_eq!(count, 11);
    // }

    #[test]
    fn post_sparse_sim_count() {
        let s = String::from(".#.\n..#\n###");
        let grid: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        if ch == '#' {
                            1
                        } else if ch == '.' {
                            0
                        } else {
                            unreachable!()
                        }
                    })
                    .collect()
            })
            .collect();
        let sprs_cube = sparse_cube_from_grid(&grid);
        let sprs_cube = sparse_cube_simulate_cycles(1, &sprs_cube, grid.len());
        assert_eq!(sprs_cube.len(), 11);
    }
}
