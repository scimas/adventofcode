use std::{fs::File, io::Read, time::Instant};

type Board = Vec<Vec<u16>>;

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let mut f;
    if let Some(fpath) = args.nth(1) {
        let base_path = std::env::current_dir()
            .map_err(|_| "could not resolve current directory".to_string())?;
        f = File::open(base_path.join(&fpath))
            .map_err(|_| format!(r#"could not open file "{fpath}""#))?;
    } else {
        return Err("incorrect number of arguments".into());
    }
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let (draw_seq_s, boards_s) = contents.trim().split_once("\n\n").unwrap();
    let draw_seq: Vec<u16> = draw_seq_s.split(',').map(|s| s.parse().unwrap()).collect();

    let boards: Vec<Board> = boards_s
        .split("\n\n")
        .map(|board_s| {
            board_s
                .split('\n')
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    println!("Day 04");

    let start_time = Instant::now();

    let mut winning_number: u16 = 0;
    let mut winning_board: Board = Vec::new();
    let mut part1_boards = boards.clone();
    for num in &draw_seq {
        let mut win_found = false;
        for board in &mut part1_boards {
            let mut num_found = false;
            for row in &mut *board {
                for tile in row {
                    if tile == num {
                        *tile = 100;
                        num_found = true;
                        break;
                    }
                }
                if num_found {
                    break;
                }
            }
            for row in &*board {
                if row.iter().all(|num| *num == 100) {
                    winning_board = board.clone();
                    winning_number = *num;
                    win_found = true;
                    break;
                }
            }
            if win_found {
                break;
            }
            for j in 0..5 {
                let mut all = true;
                for i in 0..5 {
                    if board[i][j] != 100 {
                        all = false;
                        break;
                    }
                }
                if all {
                    winning_board = board.clone();
                    winning_number = *num;
                    win_found = true;
                    break;
                }
            }
            if win_found {
                break;
            }
        }
        if win_found {
            break;
        }
    }
    let score = winning_board
        .iter()
        .flat_map(|row| row.iter().filter(|&&num| num != 100).copied())
        .sum::<u16>()
        * winning_number;
    let duration = start_time.elapsed();
    println!("Part 1: {}, time: {:.3e} s", score, duration.as_secs_f32());

    let start_time = Instant::now();
    let mut part2_boards = boards.clone();
    let mut won_boards = Vec::with_capacity(boards.len());
    for num in &draw_seq {
        for (idx, board) in part2_boards.iter_mut().enumerate() {
            let mut win_found = false;
            if won_boards.contains(&idx) {
                continue;
            }
            let mut num_found = false;
            for row in &mut *board {
                for tile in row {
                    if tile == num {
                        *tile = 100;
                        num_found = true;
                        break;
                    }
                }
                if num_found {
                    break;
                }
            }
            for row in &*board {
                if row.iter().all(|num| *num == 100) {
                    winning_number = *num;
                    won_boards.push(idx);
                    win_found = true;
                    break;
                }
            }
            if win_found {
                continue;
            }
            for j in 0..5 {
                let mut all = true;
                for i in 0..5 {
                    if board[i][j] != 100 {
                        all = false;
                        break;
                    }
                }
                if all {
                    winning_number = *num;
                    won_boards.push(idx);
                    break;
                }
            }
        }
        if won_boards.len() == boards.len() {
            break;
        }
    }
    let score = part2_boards[*won_boards.last().unwrap()]
        .iter()
        .flat_map(|row| row.iter().filter(|&&num| num != 100).copied())
        .sum::<u16>()
        * winning_number;
    let duration = start_time.elapsed();
    println!("Part 2: {}, time: {:.3e} s", score, duration.as_secs_f32());
    Ok(())
}
