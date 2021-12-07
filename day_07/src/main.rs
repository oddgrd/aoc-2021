use std::fs;

fn parse_input(input: &String) -> Vec<u32> {
    let mut sorted: Vec<u32> = input.split(",").map(|n| n.parse().unwrap()).collect();
    sorted.sort();
    sorted
}

fn part_one(positions: Vec<u32>) -> u32 {
    let mid = positions[positions.len() / 2];

    positions.iter().fold(0, |mut fuel_cost: u32, crab| {
        if crab > &mid {
            fuel_cost += crab - mid;
        } else {
            fuel_cost += mid - crab;
        }
        fuel_cost
    })
}

fn sum_range(end: u32) -> u32 {
    (0..=end).collect::<Vec<u32>>().iter().sum()
}
fn calc_fuel_cost(positions: &Vec<u32>, mid: u32) -> u32 {
    positions.iter().fold(0, |mut fuel_cost: u32, crab| {
        if crab > &mid {
            fuel_cost += sum_range(crab - mid);
        } else {
            fuel_cost += sum_range(mid - crab);
        }
        fuel_cost
    })
}

fn part_two(positions: Vec<u32>) -> u32 {
    let mid: u32 = positions[positions.len() / 2];
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
    // println!(
    //     "Last winner: {:?}",
    //     bingo_last_winner(
    //         &parse_input(&contents).1,
    //         parse_boards(parse_input(&contents).0)
    //     )
    // );
}
