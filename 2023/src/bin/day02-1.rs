use std::{collections::HashMap, error::Error};

use adventofcode2023::load_string;

#[derive(Debug)]
struct RevealedCubes {
    id: u32,
    sets: Vec<HashMap<String, u32>>,
}

fn parse_revealed_cubes(line: &str) -> RevealedCubes {
    let mut line = line.split(": ");
    let id: u32 = line
        .next()
        .expect("game")
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut all_sets = Vec::new();
    for set_line in line.next().unwrap().split("; ") {
        let mut h = HashMap::new();
        for cube in set_line.split(", ") {
            let mut cube = cube.split(' ');
            let count: u32 = cube.next().unwrap().parse().unwrap();
            let color = cube.next().unwrap().to_owned();
            h.insert(color, count);
        }
        all_sets.push(h);
    }

    RevealedCubes { id, sets: all_sets }
}

fn compute(input: &str) -> u32 {
    let mut available_cubes = HashMap::new();
    available_cubes.insert("red", 12);
    available_cubes.insert("green", 13);
    available_cubes.insert("blue", 14);

    let mut result = 0;
    for line in input.lines() {
        let revealed_cubes = parse_revealed_cubes(line);

        let mut max_cubes: HashMap<String, u32> = HashMap::new();
        for set in revealed_cubes.sets.iter() {
            for (color, &n) in set.iter() {
                max_cubes
                    .entry(color.clone())
                    .and_modify(|e| *e = (*e).max(n))
                    .or_insert(n);
            }
        }

        let mut valid = true;
        for (k, available) in available_cubes.iter() {
            if max_cubes.get(&**k).unwrap() > available {
                valid = false;
            }
        }

        if valid {
            result += revealed_cubes.id;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/02.txt")?;

    let result = compute(&input);
    dbg!(result);

    Ok(())
}

#[test]
fn day02_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let result = compute(input);

    assert_eq!(result, 8);
}
