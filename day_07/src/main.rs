use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = part_one(&parsed_input);
    let part_two = part_two(&parsed_input);
    let time = now.elapsed().as_micros(); // 135μs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut positions: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();
    positions.sort_unstable();
    positions
}

/// The optimal position the crabs can align to is the median of the sorted positions
fn part_one(positions: &[i32]) -> i32 {
    let target = positions[positions.len() / 2];

    positions.iter().map(|crab| (crab - target).abs()).sum()
}

fn part_two(positions: &[i32]) -> i32 {
    // Start calculating from the middle to reduce iterations
    let median: i32 = positions[positions.len() / 2];

    let mut current_cost = calc_fuel_cost(positions, median);

    let mut i = 0;
    // Loop until the fuel cost of the next position is greater
    loop {
        if calc_fuel_cost(positions, median + i) > current_cost {
            break current_cost;
        }
        current_cost = calc_fuel_cost(positions, median + i);
        i += 1;
    }
}

fn calc_fuel_cost(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|crab| triangle((crab - target).abs()))
        .sum()
}

/// Use triangular number formula to calculate fuel cost for part two
fn triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
