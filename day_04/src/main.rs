use std::{fs, time::Instant};

/// Used to mark drawn numbers, all board numbers are < 100.
const DRAWN: u32 = 100;
type Board = [[u32; 5]; 5];

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = bingo_first_winner(&parsed_input.0, &parsed_input.1);
    let part_two = bingo_last_winner(&parsed_input.0, &parsed_input.1);
    let time = now.elapsed().as_micros();

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(contents: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = contents.lines();
    let selection: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut board: Board = [[0; 5]; 5];
    let mut i = 0;
    let boards = lines.fold(Vec::new(), |mut boards, line| {
        if !line.is_empty() {
            line.split_whitespace()
                .enumerate()
                .for_each(|(j, num)| board[i][j] = num.parse().unwrap());
            i += 1;
        }
        if i == 5 {
            boards.push(board);
            board = [[0; 5]; 5];
            i = 0;
        }
        boards
    });
    (selection, boards)
}

fn check_column(board: &Board, index: usize) -> bool {
    let column = board
        .iter()
        .enumerate()
        .fold([0; 5], |mut column, (i, row)| {
            column[i] = row[index];
            column
        });
    column.iter().all(|&num| num == DRAWN)
}

fn bingo_first_winner(selection: &[u32], boards: &[Board]) -> u32 {
    let mut b = boards.to_vec();

    let winning_board_idx: usize;

    let mut idx = 0;
    'outer: loop {
        for i in 0..boards.len() {
            for j in 0..5 {
                for k in 0..5 {
                    if boards[i][j][k] == selection[idx] {
                        b[i][j][k] = DRAWN;
                        if b[i][j].iter().all(|&num| num == DRAWN) || check_column(&b[i], k) {
                            // Bingo
                            winning_board_idx = i;
                            break 'outer;
                        }
                    }
                }
            }
        }
        idx += 1;
    }

    let sum_undrawn: u32 = b[winning_board_idx]
        .iter()
        .flatten()
        .filter(|&n| *n != DRAWN)
        .sum();

    selection[idx] * sum_undrawn
}

fn bingo_last_winner(selection: &[u32], boards: &[Board]) -> u32 {
    let mut b = boards.to_vec();
    let mut last_winning_board = b[0];

    let mut idx = 0;
    loop {
        b.retain_mut(|board| {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == selection[idx] {
                        board[i][j] = DRAWN;
                        if board[i].iter().all(|&num| num == DRAWN) || check_column(board, j) {
                            // Bingo, drop board
                            last_winning_board = *board;
                            return false;
                        }
                    }
                }
            }
            // Not bingo, keep board
            true
        });

        if b.is_empty() {
            break;
        }
        idx += 1;
    }

    let sum_undrawn: u32 = last_winning_board
        .iter()
        .flatten()
        .filter(|&n| *n != DRAWN)
        .sum();

    selection[idx] * (sum_undrawn)
}
