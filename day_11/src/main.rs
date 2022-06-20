use std::{collections::HashSet, fs, time::Instant};

const SIZE: usize = 10;
type Grid = [[u32; SIZE]; SIZE];

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();

    let (part_one, part_two) = game_of_octopus(&mut parse_input(&contents));
    let time = now.elapsed().as_micros(); // 897μs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold([[0; SIZE]; SIZE], |mut grid, (x, line)| {
            line.chars()
                .enumerate()
                .for_each(|(y, c)| grid[x][y] = c.to_digit(10).unwrap());
            grid
        })
}

/// Returns the score of both parts, the part one score is the total flash count
/// after 100 turns. The part two score is the turn at which all octopi flash.
fn game_of_octopus(grid: &mut Grid) -> (usize, usize) {
    let mut flashes = 0;
    let mut part_one_score = 0;

    let mut i = 0;
    loop {
        let mut flashed_coords: HashSet<(usize, usize)> = HashSet::with_capacity(100);

        for x in 0..SIZE {
            for y in 0..SIZE {
                if !flashed_coords.contains(&(x, y)) {
                    grid[x][y] += 1;
                }

                if grid[x][y] > 9 {
                    flashes += 1;
                    grid[x][y] = 0;
                    flashed_coords.insert((x, y));

                    let mut stack = find_neighbours(x, y);
                    loop {
                        if stack.is_empty() {
                            break;
                        }
                        let (neighbour_x, neighbour_y) = stack.pop().unwrap();

                        if !flashed_coords.contains(&(neighbour_x, neighbour_y)) {
                            grid[neighbour_x][neighbour_y] += 1;
                        }

                        if grid[neighbour_x][neighbour_y] > 9 {
                            flashes += 1;
                            grid[neighbour_x][neighbour_y] = 0;
                            flashed_coords.insert((neighbour_x, neighbour_y));
                            stack.extend(find_neighbours(neighbour_x, neighbour_y));
                        }
                    }
                }
            }
        }

        i += 1;
        if i == 100 {
            part_one_score = flashes;
        }
        if flashed_coords.len() == 100 {
            break (part_one_score, i);
        }
    }
}

fn find_neighbours(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(8);
    //top
    if row > 0 {
        neighbours.push((row - 1, col));
    }
    //top left
    if row > 0 && col > 0 {
        neighbours.push((row - 1, col - 1));
    }
    //top right
    if row > 0 && col + 1 < SIZE {
        neighbours.push((row - 1, col + 1));
    }
    //left
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    //right
    if col + 1 < SIZE {
        neighbours.push((row, col + 1));
    }
    //bottom left
    if row + 1 < SIZE && col > 0 {
        neighbours.push((row + 1, col - 1));
    }
    //bottom right
    if row + 1 < SIZE && col + 1 < SIZE {
        neighbours.push((row + 1, col + 1));
    }
    //bottom
    if row + 1 < SIZE {
        neighbours.push((row + 1, col));
    }
    neighbours
}
