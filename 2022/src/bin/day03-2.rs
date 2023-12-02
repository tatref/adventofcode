#![feature(iter_array_chunks)]
#![feature(array_chunks)]

use std::collections::HashSet;
use std::{cmp::Ordering, error::Error, str::FromStr};

use adventofcode2022::load_vec;

fn process_group(group: [&str; 3]) -> u32 {
    let first: HashSet<char> = group[0].chars().collect();
    let second: HashSet<char> = group[1].chars().collect();
    let third: HashSet<char> = group[2].chars().collect();

    let inter1: HashSet<char> = first.intersection(&second).copied().collect();
    let common = inter1
        .intersection(&third)
        .next()
        .expect("No common element");

    dbg!(common);

    let value = match common {
        c @ 'a'..='z' => *c as u32 - 'a' as u32 + 1,
        c @ 'A'..='Z' => *c as u32 - 'A' as u32 + 1 + 26,
        _ => panic!("Unknown value"),
    };

    dbg!(value);

    value
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/03.txt")?;
    let mut input = input.iter().map(|s| s.as_str()).array_chunks::<3>();

    /*
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines()
            .array_chunks::<3>();
    */

    let total = input.map(|group| process_group(group)).sum::<u32>();

    dbg!(total);

    Ok(())
}
