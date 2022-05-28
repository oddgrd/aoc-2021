use std::str;
use std::{fs, time::Instant};

const BITS: usize = 12;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = power_consumption(&parsed_input);
    let part_two = life_support_rating(&parsed_input);

    let time = now.elapsed().as_micros();

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

fn parse_input(contents: &str) -> Vec<u32> {
    contents.lines().map(binary_to_decimal).collect()
}

fn binary_to_decimal(binary: &str) -> u32 {
    u32::from_str_radix(binary, 2).unwrap()
}

/// Counts set bits at given index of each number in given input
/// ```
/// // Returns 1 if bit is set and 0 if not
/// (num >> index) & 1
/// ```
fn count_set_bits_in_column(input: &[u32], index: usize) -> usize {
    input.iter().filter(|&num| (num >> index) & 1 == 1).count()
}

fn power_consumption(input: &[u32]) -> u32 {
    let mut most_common_bits = [0; BITS];
    for (i, bit) in most_common_bits.iter_mut().rev().enumerate() {
        // Cast bool to int (0 or 1)
        *bit = (count_set_bits_in_column(input, i) * 2 < input.len()) as u8;
    }

    let most_common = binary_to_decimal(
        &most_common_bits
            .iter()
            .map(|bit| bit.to_string())
            .collect::<String>(),
    );
    let least_common = binary_to_decimal(
        &most_common_bits
            .iter()
            .map(|bit| (1 - bit).to_string())
            .collect::<String>(),
    );

    most_common * least_common
}

fn life_support_rating(input: &[u32]) -> u32 {
    let (mut o2, mut co2) = (0, 0);

    let mut selected_rows = input.to_vec();
    for i in (0..BITS).rev() {
        let most_common_bit =
            (count_set_bits_in_column(&selected_rows, i) * 2 >= selected_rows.len()) as u32;

        selected_rows.retain(|num| (num >> i) & 1 == most_common_bit);

        if selected_rows.len() == 1 {
            o2 = selected_rows[0];
        }
    }

    let mut selected_rows = input.to_vec();
    for i in (0..BITS).rev() {
        let most_common_bit =
            (count_set_bits_in_column(&selected_rows, i) * 2 < selected_rows.len()) as u32;

        selected_rows.retain(|num| (num >> i) & 1 == most_common_bit);

        if selected_rows.len() == 1 {
            co2 = selected_rows[0];
        }
    }

    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::binary_to_decimal;

    #[test]
    fn binary_to_decimal_works() {
        assert_eq!(binary_to_decimal("110110101101"), 3501);
    }
}
