use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input.lines().fold(Vec::new(), |mut edges, line| {
        edges.push(line.split_once('-').unwrap());
        edges
    })
}

fn build_adj_list<'a>(edges: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    edges.iter().fold(HashMap::new(), |mut adj_list, (a, b)| {
        adj_list.entry(a).or_insert_with(Vec::new).push(b);
        adj_list.entry(b).or_insert_with(Vec::new).push(a);
        adj_list
    })
}

fn is_lower(s: &str) -> bool {
    s == s.to_lowercase()
}

fn contains_duplicate_small(path: &[&str]) -> bool {
    let mut visited = Vec::new();
    for s in path.iter().filter(|n| is_lower(n)).collect::<Vec<&&str>>() {
        if visited.contains(&s) {
            return true;
        }
        visited.push(s);
    }
    false
}

fn find_paths(
    graph: &HashMap<&str, Vec<&str>>,
    cave: &str,
    visited: &[&str],
    visited_small: bool,
) -> i32 {
    let mut path_count = 0;
    let mut path = visited.to_vec();
    path.push(cave);

    if cave == "end" {
        return 1;
    }

    let contains_dup = if !visited_small {
        contains_duplicate_small(&path)
    } else {
        visited_small
    };

    let edges = graph.get(cave).unwrap();
    edges.iter().for_each(|e| {
        if *e != "start" && (!is_lower(e) || (!contains_dup || !path.contains(e))) {
            path_count += find_paths(graph, e, &path, contains_dup);
        }
    });

    path_count
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let adj_list = build_adj_list(parse_input(&contents));

    let now = Instant::now();
    println!(
        "Part one paths: {:?}",
        find_paths(&adj_list, "start", &[], true)
    );
    println!(
        "Part two paths: {:?}",
        find_paths(&adj_list, "start", &[], false)
    );

    println!("Timer: {}ms", now.elapsed().as_millis()); // 0.8s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let adj_list = build_adj_list(parse_input(input));
        assert_eq!(find_paths(&adj_list, "start", &[], true), 19);
    }

    #[test]
    fn part_two() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let adj_list = build_adj_list(parse_input(input));
        assert_eq!(find_paths(&adj_list, "start", &[], false), 36);
    }
}
