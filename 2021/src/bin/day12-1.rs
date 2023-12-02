#![allow(unused_imports)]
#![allow(dead_code)]

// https://www.geeksforgeeks.org/find-paths-given-source-destination/

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
    fn get_all_paths(
        &self,
        start: &str,
        goal: &str,
        mut visited: HashSet<String>,
        mut path: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut some_paths: Vec<Vec<String>>;

        if start.to_lowercase() == start {
            visited.insert(start.into());
        }
        path.push(start.into());

        if start == goal {
            some_paths = vec![path.clone()];
            dbg!(&path);
        } else {
            some_paths = Vec::new();

            for child in self.edges.get(start).unwrap() {
                if !visited.contains(child) {
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
        visited.remove(start);

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

    let mut visited = HashSet::new();
    let mut path: Vec<String> = Vec::new();
    let all_paths = graph.get_all_paths("start", "end", visited, path);
    dbg!(&all_paths);
    dbg!(all_paths.len());

    Ok(())
}
