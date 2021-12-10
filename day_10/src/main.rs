use std::fs;

fn parse_input(input: &String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: Vec<Vec<char>>) -> i32 {
    const OPEN: [char; 4] = ['(', '[', '{', '<'];
    const CLOSE: [char; 4] = [')', ']', '}', '>'];

    input.iter().fold(0, |mut total, line| {
        let mut i = 0;
        let mut stack: Vec<char> = vec![line[i]];
        loop {
            let close_idx = CLOSE.iter().position(|c| *c == line[i + 1]);
            let open_idx = OPEN.iter().position(|c| *c == stack[stack.len() - 1]);

            if close_idx == None {
                stack.push(line[i + 1]);
            } else {
                if close_idx == open_idx {
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
            }

            i += 1;
            if i == line.len() - 1 {
                break;
            }
        }
        total
    })
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{:?}", part_one(parse_input(&contents)));
}
