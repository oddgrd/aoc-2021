use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = part_one(&parsed_input);
    let part_two = part_two(&parsed_input);
    let time = now.elapsed().as_micros(); // 150μs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

struct Patterns<'a> {
    signal: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn parse_input(input: &str) -> Vec<Patterns> {
    input.lines().fold(Vec::new(), |mut patterns, line| {
        let p = line.split_once(" | ").unwrap();
        patterns.push(Patterns {
            signal: p.0.split(' ').collect(),
            output: p.1.split(' ').collect(),
        });
        patterns
    })
}

/// The digits 1, 4, 7 and 8 can be decoded simply by segment length
fn part_one(input: &[Patterns]) -> u32 {
    input.iter().fold(0, |mut total, pattern| {
        total += pattern.output.iter().fold(0, |mut count, segment| {
            match segment.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            };
            count
        });

        total
    })
}

/// Use the patterns of the easily decoded segments to decode the rest
fn part_two(patterns: &[Patterns]) -> u32 {
    patterns.iter().fold(0, |mut sum, pattern| {
        let mut known = Known {
            one: "",
            four: "",
            seven: "",
        };

        pattern.signal.iter().for_each(|signal| match signal.len() {
            2 => known.one = signal,
            3 => known.seven = signal,
            4 => known.four = signal,
            _ => (),
        });

        sum += pattern
            .output
            .iter()
            .fold(String::new(), |mut num, code| {
                match code.len() {
                    2 => num.push('1'),
                    3 => num.push('7'),
                    4 => num.push('4'),
                    5 => num.push(known.decode(code)),
                    6 => num.push(known.decode(code)),
                    7 => num.push('8'),
                    _ => (),
                }
                num
            })
            .parse::<u32>()
            .unwrap();
        sum
    })
}

struct Known<'a> {
    one: &'a str,
    four: &'a str,
    seven: &'a str,
}
impl<'a> Known<'a> {
    /// Use the known digits to decode the rest
    fn decode(&self, code: &str) -> char {
        let mut matches = 0;
        if code.len() == 5 {
            for c in code.chars() {
                if self.seven.contains(c) {
                    matches += 1;
                }
            }
            if matches == 3 {
                return '3';
            }
            matches = 0;
            for c in code.chars() {
                if self.four.contains(c) {
                    matches += 1;
                }
            }
            if matches == 3 {
                '5'
            } else {
                '2'
            }
        } else {
            for c in code.chars() {
                if self.one.contains(c) {
                    matches += 1;
                }
            }
            if matches == 1 {
                return '6';
            }
            matches = 0;
            for c in code.chars() {
                if self.four.contains(c) {
                    matches += 1;
                }
            }
            if matches == 4 {
                '9'
            } else {
                '0'
            }
        }
    }
}
