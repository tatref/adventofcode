use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn pad(mut input: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let h = input.len();
    let w = input[0].len();

    input.insert(0, vec![9; w]);
    input.push(vec![9; w]);

    for line in input.iter_mut() {
        line.insert(0, 9);
        line.push(9);
    }

    input
}

fn low_points(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();

    for (j, line) in input.iter().enumerate() {
        for (i, v) in line.windows(3).enumerate() {
            if v[0] > v[1] && v[1] < v[2] {
                candidates.push((i, j));
            }
        }
    }

    let mut result = Vec::new();

    for (i, j) in candidates {
        let v0 = input[j - 1][i + 1];
        let v1 = input[j][i + 1];
        let v2 = input[j + 1][i + 1];

        if v0 > v1 && v1 < v2 {
            result.push((i, j));
        }
    }

    result
}

fn in_bounds(input: &Vec<Vec<u32>>, coord: (i32, i32)) -> bool {
    coord.0 >= 0
        && coord.1 >= 0
        && (coord.1 as usize) < input.len()
        && (coord.0 as usize) < input[0].len()
}

fn get_basin(input: &Vec<Vec<u32>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![start];

    let mut to_be_explored = vec![start];
    let mut explored = HashSet::new();

    while !to_be_explored.is_empty() {
        let current = to_be_explored.pop().unwrap();
        let current_val = input[current.1][current.0];

        //println!();
        //println!("current: {:?} {}", current, current_val);

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let coord = ((current.0 as i32 + dx), (current.1 as i32 + dy));

            if !in_bounds(input, coord) {
                continue;
            }
            let coord = (coord.0 as usize, coord.1 as usize);
            let v = input[coord.1][coord.0];

            //print!("Considering {:?} {}....", coord, v);
            //print!(" in to_be_explored: {}", to_be_explored.contains(&coord));
            //print!(" in explored: {} ", explored.contains(&coord));

            if !to_be_explored.contains(&coord)
                && !explored.contains(&coord)
                && v > current_val
                && v < 9
            {
                to_be_explored.push(coord);
                result.push(coord);
                //println!("Added {:?}: {}", coord, v);
            } else {
                //println!();
            }
        }
        explored.insert(current);
    }

    explored.insert(start);

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/09.txt")?;

    /*
    let input = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";
    */

    let input = parse(&input);
    let input = pad(input);

    for line in &input {
        for v in line {
            print!("{}", v);
        }
        println!();
    }

    let mut risk_level = 0;
    for (x, y) in low_points(&input) {
        let val = input[y][x + 1];
        risk_level += val + 1;
    }

    //dbg!(risk_level);
    // end of part 1

    let low = low_points(&input);
    let mut basin_sizes: Vec<_> = low
        .iter()
        .map(|start| get_basin(&input, (start.0 + 1, start.1)).len())
        .collect();

    basin_sizes.sort();
    let a = basin_sizes.pop().unwrap();
    let b = basin_sizes.pop().unwrap();
    let c = basin_sizes.pop().unwrap();

    dbg!(a * b * c);

    Ok(())
}
