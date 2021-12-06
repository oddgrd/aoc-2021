use std::fs;

fn parse_input(contents: &String) -> Vec<usize> {
    contents.split(",").map(|n| n.parse().unwrap()).collect()
}

fn simulate_lanternfish(seed: Vec<usize>, days: i32) -> usize {
    let mut fish: Vec<usize> = vec![0; 9];

    seed.iter().for_each(|f| fish[*f] += 1);

    let mut day = 0;
    while day < days {
        let tmp = fish[0];
        for i in 0..fish.len() {
            if i < 8 {
                fish[i] = fish[i + 1];
            }
        }
        fish[8] = tmp;
        fish[6] += tmp;
        day += 1;
    }

    fish.iter().sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!(
        "Total fish after 80 days: \n{:?}",
        simulate_lanternfish(parse_input(&contents), 80)
    );
    println!(
        "Total fish after 256 days: \n{:?}",
        simulate_lanternfish(parse_input(&contents), 256)
    );
}
