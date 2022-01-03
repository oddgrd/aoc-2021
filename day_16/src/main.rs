use std::fs;
use std::time::Instant;

fn parse_input(input: &str) -> String {
    input.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn binary_to_decimal(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).unwrap()
}

#[derive(Clone)]
enum Packet {
    O(Operator),
    L(Literal),
}

#[derive(Clone)]
struct Literal {
    version: usize,
    value: usize,
}
impl Literal {
    fn new(version: usize, value: usize) -> Self {
        Literal { version, value }
    }
}

#[derive(Clone)]
struct Operator {
    version: usize,
    type_id: usize,
    sub_packets: Vec<Packet>,
}
impl Operator {
    fn new(version: usize, type_id: usize, sub_packets: Vec<Packet>) -> Self {
        Operator {
            version,
            type_id,
            sub_packets,
        }
    }
    fn sum_versions(&self) -> usize {
        self.sub_packets.iter().fold(self.version, |mut sum, p| {
            sum += match p {
                Packet::L(literal) => literal.version,
                Packet::O(operator) => operator.sum_versions(),
            };
            sum
        })
    }
}

fn decode_literal(bits: &mut dyn Iterator<Item = char>) -> usize {
    let mut literal_bits = String::new();

    while let Some('1') = bits.next() {
        bits.take(4).for_each(|b| literal_bits.push(b));
    }
    bits.take(4).for_each(|b| literal_bits.push(b));

    binary_to_decimal(&literal_bits)
}

fn decode_operator(bits: &mut dyn Iterator<Item = char>) -> Vec<Packet> {
    let length_type = &bits.next().unwrap();
    let mut sub_packets = Vec::new();

    match length_type {
        '0' => {
            let len_sub_bits = binary_to_decimal(&bits.take(15).collect::<String>());
            let mut sub_bits = &mut bits.take(len_sub_bits).peekable();

            while sub_bits.peek().is_some() {
                sub_packets.push(decode(&mut sub_bits));
            }
        }
        '1' => {
            let num_sub_packets = binary_to_decimal(&bits.take(11).collect::<String>());

            for _ in 0..num_sub_packets {
                sub_packets.push(decode(bits));
            }
        }
        _ => (),
    }

    sub_packets
}

// thanks to reddit for the mutable iterator strategy
fn decode(bits: &mut dyn Iterator<Item = char>) -> Packet {
    let version = binary_to_decimal(&bits.take(3).collect::<String>());
    let type_id = binary_to_decimal(&bits.take(3).collect::<String>());

    match type_id {
        4 => Packet::L(Literal::new(version, decode_literal(bits))),
        _ => Packet::O(Operator::new(version, type_id, decode_operator(bits))),
    }
}

fn evaluate_packet(packet: &Packet) -> usize {
    match packet {
        Packet::L(literal) => literal.value,
        Packet::O(Operator {
            type_id,
            sub_packets,
            ..
        }) => match type_id {
            0 => sub_packets
                .iter()
                .map(|p| evaluate_packet(p))
                .sum::<usize>(),
            1 => sub_packets
                .iter()
                .map(|p| evaluate_packet(p))
                .fold(1, |mut val, p| {
                    val *= p;
                    val
                }),
            2 => sub_packets
                .iter()
                .map(|p| evaluate_packet(p))
                .min()
                .unwrap(),
            3 => sub_packets
                .iter()
                .map(|p| evaluate_packet(p))
                .max()
                .unwrap(),
            5 => {
                let values: Vec<usize> = sub_packets.iter().map(|p| evaluate_packet(p)).collect();
                match values[0] > values[1] {
                    true => 1,
                    false => 0,
                }
            }
            6 => {
                let values: Vec<usize> = sub_packets.iter().map(|p| evaluate_packet(p)).collect();
                match values[0] < values[1] {
                    true => 1,
                    false => 0,
                }
            }
            7 => {
                let values: Vec<usize> = sub_packets.iter().map(|p| evaluate_packet(p)).collect();
                match values[0] == values[1] {
                    true => 1,
                    false => 0,
                }
            }
            _ => 0,
        },
    }
}

fn part_one(transmission: &str) -> usize {
    let bits = &mut transmission.chars();
    let decoded = decode(bits);

    match decoded {
        Packet::L(literal) => literal.version,
        Packet::O(operator) => operator.sum_versions(),
    }
}
fn part_two(transmission: &str) -> usize {
    let bits = &mut transmission.chars();
    let decoded = decode(bits);

    evaluate_packet(&decoded)
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let now = Instant::now();

    let parsed = parse_input(&contents);
    println!("Part one: {:?}", part_one(&parsed));
    println!("Part two: {:?}", part_two(&parsed));
    let time = now.elapsed().as_micros();

    println!("time: {}", time); // 2.4ms
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(parse_input("D2FE28"), "110100101111111000101000");
    }
    #[test]
    fn part_one_0() {
        let parsed = parse_input("8A004A801A8002F478");
        assert_eq!(part_one(&parsed), 16);
    }
    #[test]
    fn part_one_1() {
        let parsed = parse_input("620080001611562C8802118E34");
        assert_eq!(part_one(&parsed), 12);
    }
    #[test]
    fn part_one_2() {
        let parsed = parse_input("C0015000016115A2E0802F182340");
        assert_eq!(part_one(&parsed), 23);
    }
    #[test]
    fn part_one_3() {
        let parsed = parse_input("A0016C880162017C3686B18A3D4780");
        assert_eq!(part_one(&parsed), 31);
    }
    #[test]
    fn part_two_0() {
        let parsed = parse_input("C200B40A82");
        assert_eq!(part_two(&parsed), 3);
    }
    #[test]
    fn part_two_1() {
        let parsed = parse_input("04005AC33890");
        assert_eq!(part_two(&parsed), 54);
    }
    #[test]
    fn part_two_2() {
        let parsed = parse_input("880086C3E88112");
        assert_eq!(part_two(&parsed), 7);
    }
    #[test]
    fn part_two_3() {
        let parsed = parse_input("9C0141080250320F1802104A08");
        assert_eq!(part_two(&parsed), 1);
    }
}
