use std::{cmp::Ordering, fs, time::Instant};

struct Target {
    x: i32,
    x1: i32,
    y: i32,
    y1: i32,
}
impl Target {
    fn new(input: &[i32]) -> Self {
        Target {
            x: input[0],
            x1: input[1],
            y: input[2],
            y1: input[3],
        }
    }
    fn probe_on_target(&self, p: &Point) -> bool {
        (p.x >= self.x && p.x <= self.x1) && (p.y >= self.y && p.y <= self.y1)
    }
    fn probe_off_target(&self, p: &Point) -> bool {
        p.x > self.x1 || p.y < self.y
    }
}
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_ascii_whitespace()
        .skip(2)
        .fold(Vec::new(), |mut values, s| {
            values.extend(
                s.trim_matches(|c: char| !c.is_numeric() && c != '-')
                    .split("..")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>(),
            );
            values
        })
}

fn min_x_velocity(goal: i32) -> i32 {
    let mut val = 1;
    loop {
        let mut sum = 0;
        for i in 0..val {
            sum += val - i;
        }
        if sum > goal {
            break val - 1;
        } else {
            val += 1;
        }
    }
}

fn part_one(input: &[i32]) -> i32 {
    let target = Target::new(input);
    let mut max_y = 0;

    // x needs to be able to reach target, y needs to go as high as possible without jumping over target
    let mut velocity = Point {
        x: min_x_velocity(target.x),
        y: target.y.abs() - 1,
    };

    let mut probe = Point { x: 0, y: 0 };

    loop {
        if target.probe_off_target(&probe) {
            break max_y;
        }
        if probe.y > max_y {
            max_y = probe.y
        }

        probe.y += velocity.y;
        velocity.y -= 1;

        match velocity.x.cmp(&0) {
            Ordering::Less => {
                probe.x += velocity.x;
                velocity.x += 1;
            }
            Ordering::Equal => {
                probe.x += velocity.x;
            }
            Ordering::Greater => {
                probe.x += velocity.x;
                velocity.x -= 1;
            }
        };
    }
}

fn part_two(input: &[i32]) -> usize {
    let target = Target::new(input);
    let mut valid_velocities: Vec<Point> = Vec::new();

    for x in min_x_velocity(target.x)..=target.x1 {
        for y in target.y..target.y.abs() {
            let mut probe = Point { x: 0, y: 0 };
            let mut velocity = Point { x, y };

            while !target.probe_off_target(&probe) {
                if target.probe_on_target(&probe) {
                    valid_velocities.push(Point { x, y });
                    break;
                }

                probe.y += velocity.y;
                velocity.y -= 1;

                match velocity.x.cmp(&0) {
                    Ordering::Less => {
                        probe.x += velocity.x;
                        velocity.x += 1;
                    }
                    Ordering::Equal => {
                        probe.x += velocity.x;
                    }
                    Ordering::Greater => {
                        probe.x += velocity.x;
                        velocity.x -= 1;
                    }
                };
            }
        }
    }
    valid_velocities.len()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let parsed = parse_input(&contents);

    let now = Instant::now();
    println!("Part one: {:?}", part_one(&parsed));

    println!("Part two: {:?}", part_two(&parsed));
    let time = now.elapsed().as_millis();

    println!("time: {}", time); // 3ms
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(
            parse_input("target area: x=20..30, y=-10..-5"),
            [20, 30, -10, -5]
        );
    }
    #[test]
    fn part_one_0() {
        let parsed = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(part_one(&parsed), 45);
    }
    #[test]
    fn part_two_0() {
        let parsed = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(part_two(&parsed), 112);
    }
}
