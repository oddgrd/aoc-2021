use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let now = Instant::now();

    let parsed = parse_input(&contents);
    println!("Part one: {:?}", part_one(&parsed));
    println!("Part two: {:?}", part_two(&parsed)); // part two 500ms

    let time = now.elapsed().as_millis();
    println!("time: {}", time);
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Elem {
    value: u32,
    depth: u8,
}
impl Elem {
    fn new(value: u32, depth: u8) -> Self {
        Elem { value, depth }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Elem>> {
    input.lines().fold(Vec::new(), |mut fishes, line| {
        let mut chars = line.chars();
        let mut depth = 0;
        let mut fish = Vec::new();
        while let Some(c) = chars.next() {
            if c == ',' {
                continue;
            }
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {
                    fish.push(Elem {
                        value: c.to_digit(10).unwrap(),
                        depth: depth - 1,
                    });
                }
            }
        }
        fishes.push(fish);
        fishes
    })
}

fn will_explode(fish: &[Elem]) -> bool {
    fish.iter().map(|f| f.depth).filter(|d| *d >= 4).count() > 1
}

fn reduce_fish(fish: &[Elem]) -> Vec<Elem> {
    let mut updated: Vec<Elem> = fish.to_vec();
    if will_explode(&updated) {
        updated = explode_fish(&updated);
    } else {
        updated = split_fish(&updated);
    }
    updated
}

fn explode_fish(fish: &[Elem]) -> Vec<Elem> {
    let mut exploded: Vec<Elem> = Vec::new();
    let mut iter = fish.iter().enumerate();

    let mut reduced = false;
    while let Some((i, Elem { value, depth })) = iter.next() {
        if reduced {
            exploded.extend(fish[i..].to_owned());
            break;
        }
        if *depth >= 4 {
            let (_, next) = iter.next().unwrap();
            if i == 0 {
                exploded.push(Elem::new(0, depth - 1));
            };
            if i > 0 {
                let left = exploded.pop().unwrap();
                exploded.push(Elem::new(left.value + value, left.depth));
                exploded.push(Elem::new(0, depth - 1));
            }
            if (i + 2) < fish.len() {
                let right = &fish[i + 2];
                exploded.push(Elem::new(right.value + next.value, right.depth));
            }
            iter.next();
            reduced = true;
        } else {
            exploded.push(Elem::new(*value, *depth));
        }
    }
    exploded
}
fn split_fish(fish: &[Elem]) -> Vec<Elem> {
    let mut updated: Vec<Elem> = Vec::new();
    let mut iter = fish.iter();

    let mut reduced = false;
    while let Some(Elem { value, depth }) = iter.next() {
        // if reduced {
        //     updated.extend(fish[i..].to_owned());
        //     break;
        // }
        if *value > 9 && !reduced {
            updated.push(Elem::new(value / 2, depth + 1));
            updated.push(Elem::new(
                (((*value as f32) / 2f32).ceil()) as u32,
                depth + 1,
            ));
            reduced = true;
        } else {
            updated.push(Elem::new(*value, *depth));
        }
    }
    updated
}

fn add_fish(a: &[Elem], b: &[Elem]) -> Vec<Elem> {
    a.iter()
        .chain(b.iter())
        .map(|e| Elem::new(e.value, e.depth + 1))
        .collect()
}

fn magnify(fish: &[Elem]) -> Vec<Elem> {
    let mut iter = fish.iter().peekable();
    let mut fish = Vec::new();

    let mut magnified = false;
    while let Some(cur) = iter.next() {
        fish.push(*cur);
        if let Some(next) = iter.peek() {
            if cur.depth == next.depth {
                let sum_pair = (3 * cur.value) + (2 * next.value);
                fish.pop();
                fish.push(Elem::new(
                    sum_pair,
                    if cur.depth > 0 { cur.depth - 1 } else { 0 },
                ));
                magnified = true;
                iter.next();
            }
        }
    }
    if !magnified {
        fish.sort_by(|a, b| a.depth.cmp(&b.depth))
    }
    fish
}
fn part_one(input: &[Vec<Elem>]) -> u32 {
    let mut iter = input.iter();
    let mut fish = iter.next().unwrap().to_owned();

    while let Some(next) = iter.next() {
        fish = add_fish(&fish, next);
        let mut reduced = reduce_fish(&fish);
        loop {
            if reduce_fish(&reduced) == reduced {
                break;
            }
            reduced = reduce_fish(&reduced);
        }
        fish = reduced;
    }

    let mut magnified = magnify(&fish);
    loop {
        magnified = magnify(&magnified);
        if magnified.len() == 1 {
            break magnified[0].value;
        }
    }
}

fn part_two(input: &[Vec<Elem>]) -> u32 {
    let mut max_pair = 0;

    for i in 0..input.len() {
        for j in i + 1..input.len() - i {
            let a = part_one(&[input[i].to_vec(), input[j].to_vec()]);
            if a > max_pair {
                max_pair = a;
            }
            let b = part_one(&[input[j].to_vec(), input[i].to_vec()]);
            if b > max_pair {
                max_pair = b;
            }
        }
    }
    max_pair
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(
            parse_input("[9,[8,7]]"),
            [[
                Elem { value: 9, depth: 0 },
                Elem { value: 8, depth: 1 },
                Elem { value: 7, depth: 1 },
            ]]
        );
    }
    #[test]
    fn explode_0() {
        let parsed = parse_input("[[[[[9,8],1],2],3],4]");
        assert_eq!(
            explode_fish(&parsed[0]),
            [
                Elem { value: 0, depth: 3 },
                Elem { value: 9, depth: 3 },
                Elem { value: 2, depth: 2 },
                Elem { value: 3, depth: 1 },
                Elem { value: 4, depth: 0 },
            ]
        );
    }
    #[test]
    fn explode_1() {
        let parsed = parse_input("[7,[6,[5,[4,[3,2]]]]]");
        assert_eq!(
            explode_fish(&parsed[0]),
            [
                Elem { value: 7, depth: 0 },
                Elem { value: 6, depth: 1 },
                Elem { value: 5, depth: 2 },
                Elem { value: 7, depth: 3 },
                Elem { value: 0, depth: 3 },
            ]
        );
    }
    #[test]
    fn explode_2() {
        let parsed = parse_input("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(
            explode_fish(&parsed[0]),
            [
                Elem { value: 6, depth: 1 },
                Elem { value: 5, depth: 2 },
                Elem { value: 7, depth: 3 },
                Elem { value: 0, depth: 3 },
                Elem { value: 3, depth: 0 },
            ]
        );
    }
    #[test]
    fn explode_3() {
        let parsed = parse_input("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert_eq!(
            explode_fish(&parsed[0]),
            [
                Elem { value: 3, depth: 1 },
                Elem { value: 2, depth: 2 },
                Elem { value: 8, depth: 3 },
                Elem { value: 0, depth: 3 },
                Elem { value: 9, depth: 1 },
                Elem { value: 5, depth: 2 },
                Elem { value: 4, depth: 3 },
                Elem { value: 3, depth: 4 },
                Elem { value: 2, depth: 4 },
            ]
        );
    }
    #[test]
    fn explode_4() {
        let parsed = parse_input("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(
            explode_fish(&parsed[0]),
            [
                Elem { value: 3, depth: 1 },
                Elem { value: 2, depth: 2 },
                Elem { value: 8, depth: 3 },
                Elem { value: 0, depth: 3 },
                Elem { value: 9, depth: 1 },
                Elem { value: 5, depth: 2 },
                Elem { value: 7, depth: 3 },
                Elem { value: 0, depth: 3 },
            ]
        );
    }
    #[test]
    fn split_0() {
        let parsed = [[
            Elem { value: 0, depth: 3 },
            Elem { value: 7, depth: 3 },
            Elem { value: 4, depth: 2 },
            Elem {
                value: 15,
                depth: 2,
            },
            Elem { value: 0, depth: 3 },
            Elem {
                value: 13,
                depth: 3,
            },
            Elem { value: 1, depth: 1 },
            Elem { value: 1, depth: 1 },
        ]];
        assert_eq!(
            split_fish(&parsed[0]),
            [
                Elem { value: 0, depth: 3 },
                Elem { value: 7, depth: 3 },
                Elem { value: 4, depth: 2 },
                Elem { value: 7, depth: 3 },
                Elem { value: 8, depth: 3 },
                Elem { value: 0, depth: 3 },
                Elem {
                    value: 13,
                    depth: 3
                },
                Elem { value: 1, depth: 1 },
                Elem { value: 1, depth: 1 },
            ]
        );
    }
    #[test]
    fn add_fish_0() {
        let parsed = [
            vec![
                Elem { value: 0, depth: 3 },
                Elem { value: 7, depth: 3 },
                Elem { value: 4, depth: 2 },
                Elem {
                    value: 15,
                    depth: 2,
                },
            ],
            vec![
                Elem { value: 0, depth: 3 },
                Elem {
                    value: 13,
                    depth: 3,
                },
                Elem { value: 1, depth: 1 },
                Elem { value: 1, depth: 1 },
            ],
        ];
        assert_eq!(
            add_fish(&parsed[0], &parsed[1]),
            [
                Elem { value: 0, depth: 4 },
                Elem { value: 7, depth: 4 },
                Elem { value: 4, depth: 3 },
                Elem {
                    value: 15,
                    depth: 3,
                },
                Elem { value: 0, depth: 4 },
                Elem {
                    value: 13,
                    depth: 4,
                },
                Elem { value: 1, depth: 2 },
                Elem { value: 1, depth: 2 },
            ]
        );
    }
}
