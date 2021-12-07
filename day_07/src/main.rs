use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    let mut sorted: Vec<i32> = input.split(",").map(|n| n.parse().unwrap()).collect();
    sorted.sort();
    sorted
}

fn part_one(positions: Vec<i32>) -> i32 {
    let mid = positions[positions.len() / 2];

    positions.iter().fold(0, |mut fuel_cost: i32, crab| {
        fuel_cost += (mid - crab).abs();
        fuel_cost
    })
}

fn triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
fn calc_fuel_cost(positions: &Vec<i32>, mid: i32) -> i32 {
    positions.iter().fold(0, |mut fuel_cost: i32, crab| {
        fuel_cost += triangle((mid - crab).abs());
        fuel_cost
    })
}
fn part_two(positions: Vec<i32>) -> i32 {
    let mid: i32 = positions[positions.len() / 2];
    let mut fuel_cost = calc_fuel_cost(&positions, mid);
    let mut i = 0;
    loop {
        if calc_fuel_cost(&positions, mid + i) > fuel_cost {
            break fuel_cost;
        }
        fuel_cost = calc_fuel_cost(&positions, mid + i);
        i += 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("First winner: {:?}", part_one(parse_input(&contents)));
    println!("Second winner: {:?}", part_two(parse_input(&contents)));
}
