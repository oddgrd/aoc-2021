use std::fs;

fn parse_input(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn binary_to_decimal(binary: String) -> u32 {
    u32::from_str_radix(&binary, 2).unwrap()
}

fn find_most_common_bit(column: &str) -> char {
    let count_0 = column.matches('0').count();
    let count_1 = column.matches('1').count();
    if count_0 > count_1 {
        '0'
    } else {
        '1'
    }
}

fn find_most_common_bits(input: &[Vec<char>]) -> Vec<char> {
    let mut most_common_bits = vec!['0'; input[0].len()];
    let mut columns = vec![String::new(); input[0].len()];
    for row in input {
        for i in 0..row.len() {
            columns[i].push(row[i]);
        }
    }
    for i in 0..columns.len() {
        most_common_bits[i] = find_most_common_bit(&columns[i]);
    }
    most_common_bits
}

fn power_consumption(input: Vec<Vec<char>>) -> u32 {
    let most_common_bits = find_most_common_bits(&input);

    let most_common = binary_to_decimal(most_common_bits.iter().collect());
    let least_common = binary_to_decimal(
        most_common_bits
            .iter()
            .map(|&bit| if bit == '0' { '1' } else { '0' })
            .collect(),
    );

    most_common * least_common
}

enum LifeSupport {
    O2,
    CO2,
}
fn life_support_rating(input: Vec<Vec<char>>, mut i: usize, system: LifeSupport) -> u32 {
    if input.len() == 1 {
        return binary_to_decimal(input[0].iter().collect());
    }

    let most_common_bits = find_most_common_bits(&input);
    let mut selected: Vec<Vec<char>> = Vec::new();

    match system {
        LifeSupport::O2 => {
            for row in input {
                if row[i] == most_common_bits[i] {
                    selected.push(row);
                }
            }
        }
        LifeSupport::CO2 => {
            for row in input {
                if row[i] != most_common_bits[i] {
                    selected.push(row);
                }
            }
        }
    }
    i += 1;
    life_support_rating(selected, i, system)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!(
        "Power consumption: {:?}",
        (power_consumption(parse_input(&contents)))
    );
    println!(
        "Life support rating: {}",
        (life_support_rating(parse_input(&contents), 0, LifeSupport::O2)
            * life_support_rating(parse_input(&contents), 0, LifeSupport::CO2))
    );
}
