#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_neighbors(
    graph: &Vec<Vec<u8>>,
    unvisited: &HashSet<(i32, i32)>,
    current: (i32, i32),
) -> Vec<(i32, i32)> {
    let h = graph.len() as i32;
    let w = graph[0].len() as i32;

    let mut result = Vec::new();
    for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (x, y) = (current.0 + dx, current.1 + dy);
        if x >= 0 && x < w && y >= 0 && y < h && unvisited.contains(&(x, y)) {
            result.push((x, y));
        }
    }

    result
}

fn next_smallest_dist_unvisited(
    dist: &Vec<Vec<u32>>,
    unvisited: &HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let mut all_nodes = Vec::new();
    for (y, line) in dist.iter().enumerate() {
        for (x, cost) in line.iter().enumerate() {
            if !unvisited.contains(&(x as i32, y as i32)) {
                continue;
            }
            all_nodes.push((x, y, cost));
        }
    }

    all_nodes.sort_by_key(|&(a, b, c)| *c);

    if let Some(x) = all_nodes.first() {
        let smallest = (x.0 as i32, x.1 as i32);
        Some(smallest)
    } else {
        None
    }
}

fn dijkstra(graph: &Vec<Vec<u8>>, from: (i32, i32), to: (i32, i32)) -> Vec<(usize, usize)> {
    let mut unvisited: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in graph.iter().enumerate() {
        for (x, _cost) in line.iter().enumerate() {
            unvisited.insert((x as i32, y as i32));
        }
    }

    let mut distances: Vec<Vec<u32>> = graph
        .iter()
        .map(|line| line.iter().map(|_cost| u32::MAX).collect())
        .collect();

    let mut prev: Vec<Vec<Option<(i32, i32)>>> = graph
        .iter()
        .map(|line| line.iter().map(|_cost| None).collect())
        .collect();

    distances[from.0 as usize][from.1 as usize] = 0;

    let mut chrono = std::time::Instant::now();

    while !unvisited.is_empty() {
        if unvisited.len() % 1000 == 0 {
            let elapsed = chrono.elapsed();
            dbg!(&unvisited.len(), elapsed);
            chrono = std::time::Instant::now();
        }

        let u = next_smallest_dist_unvisited(&distances, &unvisited).unwrap();
        unvisited.remove(&u);

        let neighbors = get_neighbors(&graph, &unvisited, u);

        for neighbor in &neighbors {
            let d = distances[u.1 as usize][u.0 as usize]
                + graph[neighbor.1 as usize][neighbor.0 as usize] as u32;

            if d < distances[neighbor.1 as usize][neighbor.0 as usize] {
                distances[neighbor.1 as usize][neighbor.0 as usize] = d;
                prev[neighbor.1 as usize][neighbor.0 as usize] = Some(u);
            }
        }
    }

    for line in &distances {
        for cost in line {
            print!("{}", cost);
        }
        println!()
    }

    dbg!(distances.last().unwrap().last().unwrap());

    let mut result = Vec::new();
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/15.txt")?;

    /*
    let input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";
    */

    /*
    let input = "010
                      331
                      010";
                      */

    let graph = parse(&input);

    fn big_map(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = vec![vec![0; input[0].len() * 5]; input.len() * 5];

        for j in 0..5 {
            for i in 0..5 {
                for (y, line) in input.iter().enumerate() {
                    for (x, v) in line.iter().enumerate() {
                        let new_y = input.len() * j + y;
                        let new_x = input[0].len() * i + x;

                        result[new_y][new_x] = ((input[y][x] + i as u8 + j as u8) - 1) % 9 + 1;
                    }
                }
            }
        }

        for line in result.iter() {
            for v in line.iter() {
                print!("{}", *v);
            }
            println!();
        }

        assert_eq!(result.iter().all(|line| line.iter().all(|v| *v != 0)), true);

        assert_eq!(result.len(), input.len() * 5);
        assert_eq!(result[0].len(), input[0].len() * 5);

        result
    }

    let graph = big_map(&graph);

    dbg!(&graph.len(), &graph[0].len());

    let path = dijkstra(&graph, (0, 0), (0, 2));

    Ok(())
}
