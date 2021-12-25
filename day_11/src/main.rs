use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_neighbours(row: usize, col: usize, matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    //top
    if row > 0 {
        neighbours.push((row - 1, col));
    }
    //top left
    if row > 0 && col > 0 {
        neighbours.push((row - 1, col - 1));
    }
    //top right
    if row > 0 && col + 1 < matrix[row].len() {
        neighbours.push((row - 1, col + 1));
    }
    //left
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    //right
    if col + 1 < matrix[row].len() {
        neighbours.push((row, col + 1));
    }
    //bottom left
    if row + 1 < matrix.len() && col > 0 {
        neighbours.push((row + 1, col - 1));
    }
    //bottom right
    if row + 1 < matrix.len() && col + 1 < matrix[row].len() {
        neighbours.push((row + 1, col + 1));
    }
    //bottom
    if row + 1 < matrix.len() {
        neighbours.push((row + 1, col));
    }
    neighbours
}

fn game_of_octopus(input: Vec<Vec<u32>>, part_one_limit: Option<usize>) -> usize {
    let mut octopi = input;
    let mut i = 0;
    let mut flashes = 0;
    loop {
        let mut flashed_coords: Vec<(usize, usize)> = Vec::new();

        for x in 0..octopi.len() {
            for y in 0..octopi[x].len() {
                if !flashed_coords.contains(&(x, y)) {
                    octopi[x][y] += 1;
                }
                if octopi[x][y] > 9 {
                    flashes += 1;
                    octopi[x][y] = 0;
                    flashed_coords.push((x, y));

                    let mut stack = find_neighbours(x, y, &octopi);
                    loop {
                        if stack.is_empty() {
                            break;
                        }
                        let oct = stack.pop().unwrap();

                        if !flashed_coords.contains(&(oct.0, oct.1)) {
                            octopi[oct.0][oct.1] += 1;
                        }
                        if octopi[oct.0][oct.1] > 9 {
                            flashes += 1;
                            octopi[oct.0][oct.1] = 0;
                            flashed_coords.push((oct.0, oct.1));
                            stack.append(&mut find_neighbours(oct.0, oct.1, &octopi));
                        }
                    }
                }
            }
        }

        i += 1;
        match part_one_limit {
            None => {
                if flashed_coords.len() == octopi.len() * octopi[0].len() {
                    break i;
                }
            }
            Some(part_one_limit) => {
                if i == part_one_limit {
                    break flashes;
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!(
        "Part one: {:?}",
        game_of_octopus(parse_input(&contents), Some(100))
    );
    println!(
        "Part two: {:?}",
        game_of_octopus(parse_input(&contents), None)
    );
}
