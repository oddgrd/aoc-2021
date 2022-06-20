use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();

    let (part_one, part_two) = extend_polymer(parse_input(&contents));
    let time = now.elapsed().as_micros(); // 669µs

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, char>) {
    let (template, insertion_rules) = input.split_once("\n\n").unwrap();

    (
        template.chars().collect(),
        insertion_rules
            .trim()
            .lines()
            .fold(HashMap::new(), |mut rules, line| {
                let (pair, insertion) = line.split_once(" -> ").unwrap();
                rules.insert(pair.to_string(), insertion.chars().next().unwrap());
                rules
            }),
    )
}

fn count_initial_pairs(template: Vec<char>) -> HashMap<String, u64> {
    let mut initial_pairs: HashMap<String, u64> = HashMap::new();
    for i in 0..template.len() - 1 {
        let counter = initial_pairs
            .entry(format!("{}{}", template[i], template[i + 1]))
            .or_insert(0);
        *counter += 1;
    }
    initial_pairs
}

fn count_elements(pairs: &HashMap<String, u64>) -> u64 {
    let mut counts = HashMap::new();
    pairs.iter().for_each(|(k, v)| {
        let counter = counts.entry(k.chars().next().unwrap()).or_insert(0u64);
        *counter += *v;
    });

    counts.values().max().unwrap() - counts.values().min().unwrap() + 1
}

fn extend_polymer((template, rules): (Vec<char>, HashMap<String, char>)) -> (u64, u64) {
    let mut pairs: HashMap<String, u64> = count_initial_pairs(template);
    let mut part_one_score = 0;

    let mut steps = 0;
    loop {
        let mut updated_pairs: HashMap<String, u64> = HashMap::new();
        pairs
            .keys()
            .map(|key| pairs.get_key_value(key).unwrap())
            .for_each(|(k, v)| {
                let left_pair = updated_pairs
                    .entry(format!(
                        "{}{}",
                        k.chars().nth(0).unwrap(),
                        *rules.get(k).unwrap()
                    ))
                    .or_insert(0);
                *left_pair += v;

                let right_pair = updated_pairs
                    .entry(format!(
                        "{}{}",
                        *rules.get(k).unwrap(),
                        k.chars().nth(1).unwrap()
                    ))
                    .or_insert(0);
                *right_pair += v;
            });

        pairs = updated_pairs;

        steps += 1;
        if steps == 10 {
            part_one_score = count_elements(&pairs);
        }
        if steps == 40 {
            break (part_one_score, count_elements(&pairs));
        }
    }
}
