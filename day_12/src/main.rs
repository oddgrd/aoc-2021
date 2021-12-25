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

fn find_paths_one<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    cave: &'a str,
    visited: &[&'a str],
) -> Vec<Vec<&'a str>> {
    let mut paths = Vec::new();
    let mut new_path = visited.to_vec();
    new_path.push(cave);
    if cave == "end" {
        return vec![new_path];
    }

    let edges = graph.get(cave).unwrap();

    edges.iter().for_each(|e| {
        if *e != "start" && (!visited.contains(e) || !is_lower(e)) {
            let tmp_path = find_paths_one(graph, e, &new_path);
            paths.extend(tmp_path);
        }
    });
    paths
}

fn contains_duplicate(path: &[&str]) -> bool {
    let mut visited = Vec::new();
    for s in path.iter().filter(|n| is_lower(n)).collect::<Vec<&&str>>() {
        if visited.contains(&s) {
            return true;
        }
        visited.push(s);
    }
    false
}

fn find_paths_two<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    cave: &'a str,
    visited: &[&'a str],
) -> Vec<Vec<&'a str>> {
    let mut paths = Vec::new();
    let mut new_path = visited.to_vec();
    new_path.push(cave);

    if cave == "end" {
        return vec![new_path];
    }

    let edges = graph.get(cave).unwrap();
    edges.iter().for_each(|e| {
        if *e != "start"
            && (!is_lower(e) || (!contains_duplicate(&new_path) || !new_path.contains(e)))
        {
            let tmp_path = find_paths_two(graph, e, &new_path);
            paths.extend(tmp_path);
        }
    });

    paths
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let adj_list = build_adj_list(parse_input(&contents));

    println!(
        "Part one paths: {:?}",
        find_paths_one(&adj_list, "start", &[]).len()
    );
    let now = Instant::now();
    println!(
        "Part two paths: {:?}",
        find_paths_two(&adj_list, "start", &[]).len()
    );
    // Part two runs in 4 seconds
    println!("Part two timer: {}ms", now.elapsed().as_millis());
}
