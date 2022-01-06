use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let now = Instant::now();

    let parsed = parse_input(&contents);
    println!("Part one: {:?}", part_one(&parsed)); //200μs

    let time = now.elapsed().as_micros();
    println!("{}μs", time);
}

struct Player {
    pos: u32,
    score: u32,
}
impl Player {
    fn forward(&mut self, moves: u32) {
        self.pos = (self.pos + moves - 1) % 10 + 1;
        self.score += self.pos;
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().fold(Vec::new(), |mut starts, line| {
        starts.push(line.chars().last().unwrap().to_digit(10).unwrap());
        starts
    })
}

fn roll_dice(turn: u32) -> u32 {
    (turn * 3) + (turn * 3 + 1) + (turn * 3 + 2) + 3
}

fn part_one(input: &[u32]) -> u32 {
    let mut p1 = Player {
        pos: input[0],
        score: 0,
    };
    let mut p2 = Player {
        pos: input[1],
        score: 0,
    };

    let mut turn = 0;
    loop {
        if p1.score >= 1000 {
            break p2.score * turn * 3;
        }
        if p2.score >= 1000 {
            break p1.score * turn * 3;
        }

        let moves = roll_dice(turn);
        match turn % 2 {
            0 => {
                p1.forward(moves);
            }
            _ => {
                p2.forward(moves);
            }
        }
        turn += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(
            parse_input(
                "Player 1 starting position: 4
                 Player 2 starting position: 8"
            ),
            [4, 8]
        );
    }
    #[test]
    fn part_one_test() {
        let parsed = parse_input(
            "Player 1 starting position: 4
             Player 2 starting position: 8",
        );
        assert_eq!(part_one(&parsed), 739785);
    }
}
