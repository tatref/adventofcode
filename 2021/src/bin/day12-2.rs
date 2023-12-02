#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn can_visit(visited: &HashMap<String, u32>, path: &Vec<String>, node: &str) -> bool {
        let big_cave = node.to_uppercase() == node;

        match (node, big_cave) {
            ("start", _) => *visited.get(node).unwrap() < 1,
            ("end", _) => *visited.get(node).unwrap() < 1,
            (_, false) => {
                let small_caves: HashMap<String, u32> = path
                    .iter()
                    .filter(|s| s.to_lowercase() == **s)
                    .fold(HashMap::new(), |mut acc, x| {
                        *acc.entry(x.to_string()).or_default() += 1;
                        acc
                    });
                let visited_twice = small_caves.values().any(|count| *count == 2);
                if *visited.get(node).unwrap() == 0 {
                    return true;
                }
                if visited_twice {
                    false
                } else {
                    true
                }
            }
            (_, true) => true,
        }
    }

    fn get_all_paths(
        &self,
        start: &str,
        goal: &str,
        mut visited: HashMap<String, u32>,
        mut path: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut some_paths: Vec<Vec<String>>;

        *visited.get_mut(start.into()).unwrap() += 1;
        path.push(start.into());

        if start == goal {
            // path found
            some_paths = vec![path.clone()];
        } else {
            some_paths = Vec::new();

            for child in self.edges.get(start).unwrap() {
                if Graph::can_visit(&visited, &path, child) {
                    some_paths.extend(self.get_all_paths(
                        child,
                        goal,
                        visited.clone(),
                        path.clone(),
                    ));
                }
            }
        }

        path.pop();
        *visited.get_mut(start.into()).unwrap() -= 1;

        some_paths
    }
}

fn parse(input: &str) -> Graph {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let mut it = line.split('-');

        let first: String = it.next().unwrap().to_owned();
        let second: String = it.next().unwrap().to_owned();

        nodes.insert(first.clone());
        nodes.insert(second.clone());

        edges
            .entry(first.clone())
            .or_insert(Vec::new())
            .push(second.clone());
        edges
            .entry(second)
            .or_insert(Vec::new())
            .push(first.clone());
    }

    Graph { nodes, edges }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/12.txt")?;

    /*
    let input = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    */

    let graph = parse(&input);

    dbg!(&graph.nodes);
    dbg!(&graph.edges);

    let mut visited: HashMap<String, u32> = graph.nodes.iter().cloned().map(|s| (s, 0)).collect();
    let mut path: Vec<String> = Vec::new();
    let all_paths = graph.get_all_paths("start", "end", visited, path);
    //dbg!(&all_paths);
    dbg!(all_paths.len());

    Ok(())
}
