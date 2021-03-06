use std::{fs, time::Instant};

const OPEN: [char; 4] = ['(', '[', '{', '<'];
const CLOSE: [char; 4] = [')', ']', '}', '>'];

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = part_one(&parsed_input);
    let part_two = part_two(&part_one.1);
    let time = now.elapsed().as_micros(); // 110μs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one.0, part_two, time
    );
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Returns part one score and incomplete lines for part two
fn part_one(input: &[Vec<char>]) -> (u32, Vec<Vec<char>>) {
    let mut incomplete = Vec::new();
    let score = input.iter().fold(0, |mut total, line| {
        let mut i = 0;
        let mut stack: Vec<char> = vec![line[i]];
        loop {
            let close_idx = CLOSE.iter().position(|c| *c == line[i + 1]);
            let open_idx = OPEN.iter().position(|c| *c == stack[stack.len() - 1]);

            if close_idx == None {
                stack.push(line[i + 1]);
            } else if close_idx == open_idx {
                stack.pop();
            } else {
                total += match line[i + 1] {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                break;
            }

            i += 1;
            if i == line.len() - 1 {
                incomplete.push(stack);
                break;
            }
        }
        total
    });

    (score, incomplete)
}

/// Close the incomplete lines from part one and return middle score
fn part_two(incomplete: &[Vec<char>]) -> u64 {
    let mut scores = incomplete.iter().fold(Vec::new(), |mut scores, line| {
        scores.push(line.iter().rev().fold(0, |mut score: u64, sym| {
            score *= 5;
            match sym {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => (),
            };
            score
        }));
        scores
    });
    scores.sort_unstable();
    scores[scores.len() / 2]
}
