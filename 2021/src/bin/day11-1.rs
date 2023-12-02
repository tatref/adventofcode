#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn display(input: &Vec<Vec<u32>>, spaces: bool) {
    for line in input {
        for c in line {
            let mut sep = "";
            if spaces {
                sep = " ";
            }
            print!("{}{}", c, sep);
        }
        println!();
    }
    println!();
}

fn step(input: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, u32) {
    let mut result = input.clone();
    let mut will_flash: Vec<(usize, usize)> = Vec::new();
    let mut flashed = HashSet::new();

    for (j, line) in result.iter_mut().enumerate() {
        for (i, v) in line.iter_mut().enumerate() {
            *v += 1;
            if *v > 9 {
                will_flash.push((i, j));
            }
        }
    }

    while let Some((x, y)) = will_flash.pop() {
        if flashed.contains(&(x, y)) {
            continue;
        }

        for (i, j) in itertools::iproduct!(-1..=1, -1..=1) {
            if i == 0 && j == 0 {
                continue;
            }

            let x = x as i32 + i;
            let y = y as i32 + j;
            if x < 0 || x >= input[0].len() as i32 {
                continue;
            }
            if y < 0 || y >= input.len() as i32 {
                continue;
            }

            result[y as usize][x as usize] += 1;
            if result[y as usize][x as usize] > 9 && !flashed.contains(&(x as usize, y as usize)) {
                will_flash.push((x as usize, y as usize));
            }
        }

        flashed.insert((x, y));
    }

    for line in result.iter_mut() {
        for x in line.iter_mut() {
            if *x > 9 {
                *x = 0;
            }
        }
    }
    (result, flashed.len() as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/11.txt")?;

    /*
        let input = "11111
    19991
    19191
    19991
    11111";
    */

    /*
        let input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    */

    let mut input: Vec<Vec<u32>> = parse(&input);

    let mut total_flashes = 0;
    for i in 0..100 {
        dbg!(i);
        display(&input, false);
        let x = step(&input);
        input = x.0;
        let flashes = x.1;
        total_flashes += flashes;
    }

    dbg!(total_flashes);

    Ok(())
}
