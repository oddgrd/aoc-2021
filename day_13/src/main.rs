use std::fs;

enum Fold {
    Left(usize),
    Up(usize),
}
type Dot = (i32, i32);
fn parse_input(input: &str) -> (Vec<Dot>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\r").unwrap();
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
                let (ch, num) = line[11..line.len()].split_once('=').unwrap();

                if ch == "x" {
                    Fold::Left(num.parse().unwrap())
                } else {
                    Fold::Up(num.parse().unwrap())
                }
            })
            .collect(),
    )
}

// This is needed for part one
fn initial_size(folds: &[Fold]) -> (usize, usize) {
    folds.iter().fold((0, 0), |sizes, fold| match fold {
        Fold::Left(x) => {
            if x > &sizes.0 {
                (x * 2, sizes.1)
            } else {
                sizes
            }
        }
        Fold::Up(y) => {
            if y > &sizes.1 {
                (sizes.0, y * 2)
            } else {
                sizes
            }
        }
    })
}

fn calculate_folds(
    input: (Vec<Dot>, Vec<Fold>),
    limit: Option<usize>,
) -> (Vec<Dot>, (usize, usize)) {
    let (mut dots, folds) = input;
    let (mut width, mut height) = initial_size(&folds);

    let mut i = 0;
    loop {
        match folds[i] {
            Fold::Left(line) => {
                width = line;
                for (x, _) in dots.iter_mut() {
                    if *x > line as i32 {
                        *x = (*x - (line as i32 * 2)).abs();
                    }
                }
            }
            Fold::Up(line) => {
                height = line;
                for (_, y) in dots.iter_mut() {
                    if *y > line as i32 {
                        *y = (*y - (line as i32 * 2)).abs();
                    }
                }
            }
        }

        i += 1;
        match limit {
            Some(limit) => {
                if i == limit {
                    break;
                }
            }
            None => {
                if i == folds.len() {
                    break;
                }
            }
        }
    }

    (dots, (width, height))
}

fn build_board((dots, (width, height)): (Vec<Dot>, (usize, usize))) -> Vec<Vec<char>> {
    dots.into_iter()
        .fold(vec![vec!['.'; width]; height], |mut board, (x, y)| {
            board[y as usize][x as usize] = '#';
            board
        })
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!(
        "Part one: \n{:?}",
        build_board(calculate_folds(parse_input(&contents), Some(1)))
            .iter()
            .flatten()
            .filter(|p| **p == '#')
            .count()
    );

    println!("Part two:");
    build_board(calculate_folds(parse_input(&contents), None))
        .iter()
        .for_each(|line| println!("{:?}", line));
}
