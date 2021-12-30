use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
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

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn find_neighbours(p: Point, matrix: &[Vec<usize>]) -> Vec<Point> {
    let mut neighbours = Vec::new();

    //top
    if p.x > 0 {
        neighbours.push(Point::new(p.x - 1, p.y));
    }
    //left
    if p.y > 0 {
        neighbours.push(Point::new(p.x, p.y - 1));
    }
    //right
    if p.y + 1 < matrix[p.x].len() {
        neighbours.push(Point::new(p.x, p.y + 1));
    }
    //bottom
    if p.x + 1 < matrix.len() {
        neighbours.push(Point::new(p.x + 1, p.y));
    }
    neighbours
}

fn dijkstra(matrix: Vec<Vec<usize>>) -> Option<usize> {
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: matrix[0].len() - 1,
        y: matrix.len() - 1,
    };

    let mut dist = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()];
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

        for Point { x, y } in find_neighbours(position, &matrix) {
            let next = State {
                cost: cost + matrix[x][y],
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

fn increment_tile(tile: &[Vec<usize>], i: usize) -> Vec<Vec<usize>> {
    tile.iter()
        .map(|row| {
            row.iter()
                .map(|v| if *v + i > 9 { (v + i) - 9 } else { v + i })
                .collect()
        })
        .collect()
}

fn expand_down(matrix: &[Vec<usize>], steps: usize) -> Vec<Vec<usize>> {
    let mut expanded = matrix.to_vec();
    let mut i = 1;

    loop {
        expanded.extend(increment_tile(matrix, i));

        i += 1;
        if i == steps + 1 {
            break expanded;
        }
    }
}

fn expand_right(matrix: &[Vec<usize>], steps: usize) -> Vec<Vec<usize>> {
    let mut expanded = matrix.to_vec();
    let mut i = 1;

    loop {
        let incremented = increment_tile(matrix, i);

        for j in 0..expanded.len() {
            expanded[j].extend(&incremented[j]);
        }

        i += 1;
        if i == steps + 1 {
            break expanded;
        }
    }
}

fn expand_matrix(matrix: &[Vec<usize>], steps: usize) -> Vec<Vec<usize>> {
    expand_down(matrix, steps)
        .chunks(matrix.len())
        .fold(Vec::new(), |mut expanded, tile| {
            expanded.extend(expand_right(tile, 4));
            expanded
        })
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let now = Instant::now();
    let expanded = expand_matrix(&parse_input(&contents), 4);
    println!("{:?}", dijkstra(expanded));
    println!("time: {}", now.elapsed().as_millis()); // 400ms
}
