use std::fs;

fn parse_input(input: &String) -> Vec<(&str, &str)> {
    input.lines().fold(Vec::new(), |mut patterns, line| {
        patterns.push(line.split_once(" | ").unwrap());
        patterns
    })
}

fn part_one(input: Vec<(&str, &str)>) -> u32 {
    input.iter().fold(0, |mut total, pattern| {
        total += pattern.1.split(" ").fold(0, |mut count, segment| {
            match segment.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            };
            count
        });

        total
    })
}

struct Known {
    one: String,
    four: String,
    seven: String,
    eight: String,
}
impl Known {
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
                return '5';
            } else {
                return '2';
            };
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
                return '9';
            } else {
                return '0';
            }
        }
    }
}
fn part_two(input: Vec<(&str, &str)>) -> u32 {
    input.iter().fold(0, |mut sum, tup| {
        let mut known = Known {
            one: String::new(),
            four: String::new(),
            seven: String::new(),
            eight: String::new(),
        };
        let signal_patterns: Vec<&str> = tup.0.split(" ").collect();
        let output_patterns: Vec<&str> = tup.1.split(" ").collect();

        signal_patterns.iter().for_each(|s| match s.len() {
            2 => known.one = s.to_string(),
            3 => known.seven = s.to_string(),
            4 => known.four = s.to_string(),
            7 => known.eight = s.to_string(),
            _ => (),
        });

        sum += output_patterns
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
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{:?}", part_one(parse_input(&contents)));
    println!("{}", part_two(parse_input(&contents)));
}
