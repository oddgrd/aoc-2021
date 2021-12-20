use std::fs;

fn parse_input(contents: &str) -> Vec<Vec<usize>> {
    contents
        .lines()
        .map(|line| line.replace(" -> ", " "))
        .map(|line| line.replace(",", " "))
        .fold(Vec::new(), |mut coords, line| {
            coords.push(line.split(' ').map(|s| s.parse().unwrap()).collect());
            coords
        })
}

fn sum_overlaps(grid: Vec<Vec<usize>>) -> usize {
    grid.into_iter().flatten().filter(|n| *n > 1).count()
}

fn find_overlaps(coordinates: Vec<Vec<usize>>) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    coordinates.iter().for_each(|coord| {
        if coord[0] == coord[2] {
            let mut x = vec![coord[1], coord[3]];
            x.sort_unstable();
            for i in x[0]..=x[1] {
                grid[i][coord[0]] += 1;
            }
        } else if coord[1] == coord[3] {
            let mut y = vec![coord[0], coord[2]];
            y.sort_unstable();
            for i in y[0]..=y[1] {
                grid[coord[1]][i] += 1;
            }
        } else {
            let x = vec![coord[0], coord[2]];

            let x: Vec<usize> = if x[0] > x[1] {
                (x[1]..=x[0]).rev().collect()
            } else {
                (x[0]..=x[1]).collect()
            };

            let y = vec![coord[1], coord[3]];

            let y: Vec<usize> = if y[0] > y[1] {
                (y[1]..=y[0]).rev().collect()
            } else {
                (y[0]..=y[1]).collect()
            };

            for i in 0..x.len() {
                grid[y[i]][x[i]] += 1;
            }
        }
    });
    sum_overlaps(grid)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!(
        "Vertical, horizontal and diagonal overlaps: \n{:?}",
        find_overlaps(parse_input(&contents))
    );
}
