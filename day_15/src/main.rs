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
            let chars = line.chars().collect::<Vec<char>>();
            chars
                .iter()
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

    let mut dist: Vec<Vec<_>> = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()];
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

        for p in find_neighbours(position, &matrix) {
            let next = State {
                cost: cost + matrix[p.x][p.y],
                position: p,
            };
            if next.cost < dist[p.x][p.y] {
                heap.push(next);

                dist[p.x][p.y] = next.cost;
            }
        }
    }

    None
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let now = Instant::now();
    println!("{:?}", dijkstra(parse_input(&contents)));
    println!("time: {}", now.elapsed().as_millis());
}
