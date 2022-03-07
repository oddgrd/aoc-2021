use arrayvec::ArrayVec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u16,
    position: Point,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: matrix[0].len() - 1,
        y: matrix.len() - 1,
    };

    let mut dist = [[u16::MAX; SIZE]; SIZE];
    let mut heap = BinaryHeap::new();

    dist[start.x][start.y] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[position.x][position.y] {
            continue;
        }

        for Point { x, y } in find_neighbours(position, SIZE) {
            let next = State {
                cost: cost + matrix[x][y] as u16,
                position: Point { x, y },
            };

            if next.cost < dist[x][y] {
                heap.push(next);
                dist[x][y] = next.cost;
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
    println!("time: {}", now.elapsed().as_millis()); // 270ms
}
