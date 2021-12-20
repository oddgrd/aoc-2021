use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Debug, Default)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
    neighbours: Vec<u32>,
}
impl Point {
    fn is_lowest(&self) -> bool {
        self.value < *self.neighbours.iter().min().unwrap()
    }
    fn risk_level(&self) -> u32 {
        self.value + 1
    }
}

fn find_neigbours(row: usize, col: usize, matrix: &[Vec<u32>]) -> Vec<u32> {
    let mut neighbours = Vec::new();
    //top
    if row > 0 {
        neighbours.push(matrix[row - 1][col]);
    }
    //top left
    if row > 0 && col > 0 {
        neighbours.push(matrix[row - 1][col - 1]);
    }
    //top right
    if row > 0 && col + 1 < matrix[row].len() {
        neighbours.push(matrix[row - 1][col + 1]);
    }
    //left
    if col > 0 {
        neighbours.push(matrix[row][col - 1]);
    }
    //right
    if col + 1 < matrix[row].len() {
        neighbours.push(matrix[row][col + 1]);
    }
    //bottom left
    if row + 1 < matrix.len() && col > 0 {
        neighbours.push(matrix[row + 1][col - 1]);
    }
    //bottom right
    if row + 1 < matrix.len() && col + 1 < matrix[row].len() {
        neighbours.push(matrix[row + 1][col + 1]);
    }
    //bottom
    if row + 1 < matrix.len() {
        neighbours.push(matrix[row + 1][col]);
    }
    neighbours
}

fn part_one(input: Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let point = Point {
                value: input[x][y],
                neighbours: find_neigbours(x, y, &input),
                ..Default::default()
            };
            if point.is_lowest() {
                total += point.risk_level();
            }
        }
    }
    total
}

fn expand_basin(row: usize, col: usize, matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut basin: Vec<(usize, usize)> = Vec::new();
    //top
    if row > 0 {
        let mut i: usize = 1;
        while row >= i && matrix[row - i][col] < 9 {
            basin.push((row - i, col));
            i += 1;
        }
    }
    //left
    if col > 0 {
        let mut i: usize = 1;
        while col >= i && matrix[row][col - i] < 9 {
            basin.push((row, col - i));
            i += 1;
        }
    }
    //right
    if col + 1 < matrix[row].len() {
        let mut i: usize = 1;
        while col + i < matrix[row].len() && matrix[row][col + i] < 9 {
            basin.push((row, col + i));
            i += 1;
        }
    }
    //bottom
    if row + 1 < matrix.len() {
        let mut i: usize = 1;
        while row + i < matrix.len() && matrix[row + i][col] < 9 {
            basin.push((row + i, col));
            i += 1;
        }
    }
    basin
}

fn go_deeper(seed: &[(usize, usize)], matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut copy = seed.to_owned();
    for (x, y) in seed {
        for z in expand_basin(*x, *y, matrix) {
            if !copy.contains(&z) {
                copy.push(z);
            }
        }
    }
    copy
}

fn multiply_biggest_basins(lengths: Vec<usize>) -> usize {
    let mut sorted = lengths;
    sorted.sort_unstable();
    let len = sorted.len();
    sorted[len - 1] * sorted[len - 2] * sorted[len - 3]
}

fn part_two(input: Vec<Vec<u32>>) -> usize {
    let mut lowest_points: Vec<Point> = Vec::new();
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let point = Point {
                x,
                y,
                value: input[x][y],
                neighbours: find_neigbours(x, y, &input),
            };
            if point.is_lowest() {
                lowest_points.push(point);
            }
        }
    }

    let seed: Vec<Vec<(usize, usize)>> = lowest_points.iter().map(|p| vec![(p.x, p.y)]).collect();
    let basins: Vec<Vec<(usize, usize)>> = seed
        .iter()
        .map(|basin| {
            let mut prev = go_deeper(basin, &input);

            loop {
                if go_deeper(&prev, &input).len() == prev.len() {
                    break prev;
                } else {
                    prev = go_deeper(&prev, &input);
                }
            }
        })
        .collect();

    multiply_biggest_basins(basins.iter().map(|b| b.len()).collect::<Vec<usize>>())
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!("Sum of risk levels: {:?}", part_one(parse_input(&contents)));
    println!(
        "Product of three largest basin sizes {:?}",
        part_two(parse_input(&contents))
    );
}
