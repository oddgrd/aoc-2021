use std::fs;

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increases(input: Vec<u32>) -> u32 {
    let mut counter = 0;
    let mut prev = input[0];
    for num in input {
        if num > prev {
            counter += 1;
        }
        prev = num;
    }
    counter
}

fn count_increases_triplets(input: Vec<u32>) -> u32 {
    let mut counter = 0;
    let mut prev: u32 = input[0..=2].iter().sum();
    for i in 0..input.len() - 2 {
        let sum = (input[i..=i + 2]).iter().sum();
        if sum > prev {
            counter += 1;
        }
        prev = sum;
    }
    counter
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!(
        "Single increases count:\n{}",
        count_increases(parse_input(&contents))
    );
    println!(
        "Sum triplets increases count:\n{}",
        count_increases_triplets(parse_input(&contents))
    );
}
