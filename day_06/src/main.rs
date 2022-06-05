use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = simulate_lanternfish(&parsed_input, 80);
    let part_two = simulate_lanternfish(&parsed_input, 256);
    let time = now.elapsed().as_micros(); // 6μs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(contents: &str) -> Vec<usize> {
    contents.split(',').map(|n| n.parse().unwrap()).collect()
}

fn simulate_lanternfish(seed: &[usize], days: u16) -> u64 {
    // Track the number of fish in each state
    let mut fish = [0u64; 9];

    seed.iter().for_each(|&state| fish[state] += 1);

    let mut day = 0;
    // each day the fish in one state shift right to the next state,
    // spawning new fish when state is 0
    while day < days {
        let will_spawn = fish[0];
        for i in 0..fish.len() {
            if i < 8 {
                fish[i] = fish[i + 1];
            }
        }
        // fish with state 0 create a new fish with state 8
        fish[8] = will_spawn;
        // fish with state 0 becomes a fish with state 6
        fish[6] += will_spawn;
        day += 1;
    }

    fish.iter().sum()
}
