use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let part_one = build_board(calculate_folds(parse_input(&contents), Some(1)))
        .iter()
        .flatten()
        .filter(|&p| *p == '#')
        .count();

    let part_two = build_board(calculate_folds(parse_input(&contents), None));
    let time = now.elapsed().as_micros(); // 732µs

    println!("Part one: {}", part_one);
    println!("Part two:");
    part_two.iter().for_each(|line| println!("{:?}", line));
    println!("Time: {}µs", time);
}

type DotCoordinate = (i32, i32);
type BoardDimensions = (usize, usize);

enum Fold {
    Left(usize),
    Up(usize),
}

fn parse_input(input: &str) -> (Vec<DotCoordinate>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    (
        dots.lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
        folds
            .trim()
            .lines()
            .map(|line| {
                let (char, num) = line[11..line.len()].split_once('=').unwrap();

                if char == "x" {
                    Fold::Left(num.parse().unwrap())
                } else {
                    Fold::Up(num.parse().unwrap())
                }
            })
            .collect(),
    )
}

/// The initial size is twice as big as the first fold instruction
fn initial_size(folds: &[Fold]) -> BoardDimensions {
    folds.iter().fold((0, 0), |sizes, fold| match fold {
        Fold::Left(x) => (x * 2, sizes.1),
        Fold::Up(y) => (sizes.0, y * 2),
    })
}

fn calculate_folds(
    input: (Vec<DotCoordinate>, Vec<Fold>),
    limit: Option<usize>,
) -> (Vec<DotCoordinate>, BoardDimensions) {
    let (mut dots, folds) = input;
    let (mut width, mut height) = initial_size(&folds[0..2]);

    let mut i = 0;
    loop {
        match folds[i] {
            Fold::Left(fold_line) => {
                width = fold_line;
                for (x, _) in &mut dots {
                    if *x > fold_line as i32 {
                        *x = (*x - (fold_line as i32 * 2)).abs();
                    }
                }
            }
            Fold::Up(fold_line) => {
                height = fold_line;
                for (_, y) in &mut dots {
                    if *y > fold_line as i32 {
                        *y = (*y - (fold_line as i32 * 2)).abs();
                    }
                }
            }
        }

        i += 1;
        if i == limit.unwrap_or(folds.len()) {
            break;
        }
    }

    (dots, (width, height))
}

fn build_board((dots, (width, height)): (Vec<DotCoordinate>, BoardDimensions)) -> Vec<Vec<char>> {
    dots.into_iter()
        .fold(vec![vec!['.'; width]; height], |mut board, (x, y)| {
            board[y as usize][x as usize] = '#';
            board
        })
}
