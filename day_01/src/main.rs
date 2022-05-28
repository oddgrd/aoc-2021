use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = count_increases(&parsed_input);
    let part_two = count_increases_triplets(&parsed_input);
    let time = now.elapsed().as_micros();

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increases(input: &[u32]) -> u32 {
    let mut counter = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            counter += 1;
        }
    }
    counter
}

/// The last two numbers of the first triple and the first two of the next triplet overlap
/// Only compare the non-overlapping numbers
fn count_increases_triplets(input: &[u32]) -> u32 {
    let mut counter = 0;

    for i in 3..input.len() {
        if input[i] > input[i - 3] {
            counter += 1;
        }
    }
    counter
}
