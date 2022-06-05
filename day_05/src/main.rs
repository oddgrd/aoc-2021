use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let parsed_input = parse_input(&contents);

    let part_one = find_overlaps(&parsed_input, false);
    let part_two = find_overlaps(&parsed_input, true);
    let time = now.elapsed().as_micros(); // 1.6ms

    println!(
        "Part one: {}\nPart two: {}\nTime: {} μs",
        part_one, part_two, time
    );
}

#[derive(Debug)]
struct Line {
    /// (x, y)
    start: (usize, usize),
    /// (x, y)
    end: (usize, usize),
}

fn parse_input(contents: &str) -> Vec<Line> {
    contents
        .lines()
        .map(|line| line.replace(" -> ", " "))
        .map(|line| line.replace(',', " "))
        .fold(Vec::new(), |mut coords, line| {
            coords.push({
                let line: Vec<usize> = line.split(' ').map(|s| s.parse().unwrap()).collect();

                Line {
                    start: (line[0], line[1]),
                    end: (line[2], line[3]),
                }
            });
            coords
        })
}

fn find_overlaps(lines: &[Line], include_diagonal_lines: bool) -> usize {
    let mut grid = vec![vec![0u8; 1000]; 1000];

    lines.iter().for_each(|line| {
        if line.start.0 == line.end.0 {
            let mut y = vec![line.start.1, line.end.1];
            y.sort_unstable();
            for i in y[0]..=y[1] {
                grid[i][line.start.0] += 1;
            }
        } else if line.start.1 == line.end.1 {
            let mut x = vec![line.start.0, line.end.0];
            x.sort_unstable();
            for i in x[0]..=x[1] {
                grid[line.start.1][i] += 1;
            }
        } else if include_diagonal_lines {
            let x: Vec<usize> = if line.start.0 > line.end.0 {
                (line.end.0..=line.start.0).rev().collect()
            } else {
                (line.start.0..=line.end.0).collect()
            };

            let y: Vec<usize> = if line.start.1 > line.end.1 {
                (line.end.1..=line.start.1).rev().collect()
            } else {
                (line.start.1..=line.end.1).collect()
            };

            for i in 0..x.len() {
                grid[y[i]][x[i]] += 1;
            }
        }
    });
    grid.into_iter()
        .map(|row| row.into_iter().filter(|&x| x > 1).count())
        .sum()
}
