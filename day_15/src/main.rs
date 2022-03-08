use arrayvec::ArrayVec;
use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;

#[derive(PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

const GRID_SIZE: usize = 100;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_neighbours(p: Point, size: usize) -> ArrayVec<Point, 4> {
    let mut neighbours = ArrayVec::<Point, 4>::new();

    //top
    if p.x > 0 {
        neighbours.push(Point::new(p.x - 1, p.y));
    }
    //left
    if p.y > 0 {
        neighbours.push(Point::new(p.x, p.y - 1));
    }
    //right
    if p.y + 1 < size {
        neighbours.push(Point::new(p.x, p.y + 1));
    }
    //bottom
    if p.x + 1 < size {
        neighbours.push(Point::new(p.x + 1, p.y));
    }
    neighbours
}

fn dijkstra<const SIZE: usize>(matrix: &Vec<Vec<u8>>) -> Option<u16> {
    let start = Point::new(0, 0);
    let end = Point::new(matrix[0].len() - 1, matrix.len() - 1);

    let mut dist = [[u16::MAX; SIZE]; SIZE];
    let mut fringe: BTreeMap<u16, Vec<Point>> = BTreeMap::new();

    dist[start.x][start.y] = 0;
    fringe.insert(0, vec![start]);

    while let Some(&cost) = fringe.keys().next() {
        let nodes = fringe.remove(&cost).unwrap();

        for current in nodes.into_iter() {
            if current == end {
                return Some(cost);
            }

            if cost > dist[current.x][current.y] {
                continue;
            }

            for p in find_neighbours(current, SIZE) {
                let cost = cost + matrix[p.x][p.y] as u16;

                if cost < dist[p.x][p.y] {
                    dist[p.x][p.y] = cost;
                    let path = fringe
                        .entry(cost)
                        .or_insert_with(|| Vec::with_capacity(SIZE / 2));
                    path.push(p);
                }
            }
        }
    }

    None
}

fn increment_tile(tile: &[Vec<u8>], i: u8) -> Vec<Vec<u8>> {
    tile.iter()
        .map(|row| {
            row.iter()
                .map(|v| if *v + i > 9 { (v + i) - 9 } else { v + i })
                .collect()
        })
        .collect()
}

fn expand_down(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut expanded = matrix.to_vec();
    let mut i = 0;

    loop {
        expanded.extend(increment_tile(matrix, i + 1));

        i += 1;
        if i == 4 {
            break expanded;
        }
    }
}

fn expand_right(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut expanded = matrix.to_vec();
    let mut i = 0;

    loop {
        let incremented = increment_tile(matrix, i + 1);

        for j in 0..expanded.len() {
            expanded[j].extend(&incremented[j]);
        }

        i += 1;
        if i == 4 {
            break expanded;
        }
    }
}

fn expand_matrix(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    expand_down(matrix)
        .chunks(matrix.len())
        .fold(Vec::new(), |mut expanded, tile| {
            expanded.extend(expand_right(tile));
            expanded
        })
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    println!(
        "Part one: {:?}",
        dijkstra::<GRID_SIZE>(&parse_input(&contents))
    );
    println!(
        "Part two: {:?}",
        dijkstra::<{ 5 * GRID_SIZE }>(&expand_matrix(&parse_input(&contents)))
    );
    println!("time: {}", now.elapsed().as_millis()); // 170ms
}
