use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = final_position(&parsed_input);
    let part_two = final_position_with_aim(&parsed_input);
    let time = now.elapsed().as_micros();

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(contents: &str) -> Vec<Direction> {
    contents
        .lines()
        .map(|line| line.try_into().expect("Invalid input"))
        .collect()
}

enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}
impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let distance = (&line[line.len() - 1..]).parse().expect("Not an integer");

        match &line[0..line.len() - 2] {
            "forward" => Ok(Direction::Forward(distance)),
            "down" => Ok(Direction::Down(distance)),
            "up" => Ok(Direction::Up(distance)),
            _ => Err(()),
        }
    }
}

fn final_position(input: &[Direction]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for direction in input {
        match direction {
            Direction::Forward(dist) => horizontal += dist,
            Direction::Down(dist) => depth += dist,
            Direction::Up(dist) => depth -= dist,
        }
    }
    horizontal * depth
}

fn final_position_with_aim(input: &[Direction]) -> u32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for direction in input {
        match direction {
            Direction::Forward(dist) => {
                horizontal += dist;
                depth += dist * aim;
            }
            Direction::Down(dist) => aim += dist,
            Direction::Up(dist) => aim -= dist,
        }
    }
    horizontal * depth
}
