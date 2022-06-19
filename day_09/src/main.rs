use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = part_one(&parsed_input);
    let part_two = part_two(&parsed_input);
    let time = now.elapsed().as_micros(); // 8ms

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| Point {
                    x: i,
                    y: j,
                    value: char.to_digit(10).unwrap(),
                })
                .collect()
        })
        .collect()
}

/// A point on the heightmap of the cave
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

/// The sum of the risk levels of all low points on the map
fn part_one(input: &[Vec<Point>]) -> u32 {
    let mut total = 0;
    for row in input {
        for point in row {
            let lowest_neighbor = find_neighbors(point, input).into_iter().min().unwrap();

            if point.value < lowest_neighbor {
                total += point.value + 1
            }
        }
    }
    total
}

/// Find the product of the three biggest basins
fn part_two(input: &[Vec<Point>]) -> usize {
    let mut lowest_points: Vec<Vec<Point>> = Vec::new();
    for row in input {
        for point in row {
            let lowest_neighbor = find_neighbors(point, input).into_iter().min().unwrap();

            if point.value < lowest_neighbor {
                lowest_points.push(vec![*point])
            }
        }
    }

    // Search for higher neighbors until there are none, collect basin sizes
    let mut basin_sizes: Vec<usize> = lowest_points.iter().fold(
        Vec::with_capacity(lowest_points.len()),
        |mut basins, basin| {
            let mut current_basin = search_higher(basin, input);

            basins.push(loop {
                if search_higher(&current_basin, input).len() == current_basin.len() {
                    // no higher neighbors, return basin size
                    break current_basin.len();
                } else {
                    current_basin = search_higher(&current_basin, input);
                }
            });

            basins
        },
    );

    // return product of three biggest basins
    basin_sizes.sort_unstable();
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

fn find_neighbors(p: &Point, grid: &[Vec<Point>]) -> Vec<u32> {
    let mut neighbor_values = Vec::with_capacity(4);
    //top
    if p.x > 0 {
        neighbor_values.push(grid[p.x - 1][p.y].value);
    }
    //left
    if p.y > 0 {
        neighbor_values.push(grid[p.x][p.y - 1].value);
    }
    //right
    if p.y + 1 < grid[p.x].len() {
        neighbor_values.push(grid[p.x][p.y + 1].value);
    }
    //bottom
    if p.x + 1 < grid.len() {
        neighbor_values.push(grid[p.x + 1][p.y].value);
    }
    neighbor_values
}

fn search_higher(basin: &[Point], grid: &[Vec<Point>]) -> Vec<Point> {
    let mut copy = basin.to_vec();
    for point in basin {
        for neighbor in find_higher_neighbors(point.x, point.y, grid) {
            if !copy.contains(&neighbor) {
                copy.push(neighbor);
            }
        }
    }
    copy
}

fn find_higher_neighbors(row: usize, col: usize, grid: &[Vec<Point>]) -> Vec<Point> {
    let mut basin: Vec<Point> = Vec::with_capacity(4);
    //top
    if row > 0 {
        let mut i: usize = 1;
        while row >= i && grid[row - i][col].value < 9 {
            basin.push(grid[row - i][col]);
            i += 1;
        }
    }
    //left
    if col > 0 {
        let mut i: usize = 1;
        while col >= i && grid[row][col - i].value < 9 {
            basin.push(grid[row][col - i]);
            i += 1;
        }
    }
    //right
    if col + 1 < grid[row].len() {
        let mut i: usize = 1;
        while col + i < grid[row].len() && grid[row][col + i].value < 9 {
            basin.push(grid[row][col + i]);
            i += 1;
        }
    }
    //bottom
    if row + 1 < grid.len() {
        let mut i: usize = 1;
        while row + i < grid.len() && grid[row + i][col].value < 9 {
            basin.push(grid[row + i][col]);
            i += 1;
        }
    }
    basin
}
