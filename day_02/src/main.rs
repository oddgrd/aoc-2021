use std::fs;

fn parse_input(contents: &String) -> Vec<(&str, i32)> {
    contents
        .lines()
        .map(|line| {
            (
                &line[0..line.len() - 2],
                (&line[line.len() - 1..]).parse().unwrap(),
            )
        })
        .collect()
}

fn final_position(input: Vec<(&str, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for (a, b) in input {
        match a {
            "forward" => horizontal += b,
            "down" => depth += b,
            "up" => depth -= b,
            _ => (),
        }
    }
    horizontal * depth
}

fn final_position_two(input: Vec<(&str, i32)>) -> i32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for (a, b) in input {
        match a {
            "forward" => {
                horizontal += b;
                depth += b * aim;
            }
            "down" => aim += b,
            "up" => aim -= b,
            _ => (),
        }
    }
    horizontal * depth
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!("Final position {}", final_position(parse_input(&contents)));
    println!(
        "Final position with aim {}",
        final_position_two(parse_input(&contents))
    );
}
