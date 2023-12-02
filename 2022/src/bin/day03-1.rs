use std::collections::HashSet;
use std::{cmp::Ordering, error::Error, str::FromStr};

use adventofcode2022::load_vec;

fn process_rucksack(line: &str) -> u32 {
    let items_length = line.chars().count();

    let first: HashSet<char> = line.chars().take(items_length / 2).collect();
    let second: HashSet<char> = line
        .chars()
        .skip(items_length / 2)
        .take(items_length / 2)
        .collect();

    //dbg!(first, second);
    let common = first
        .intersection(&second)
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
    let mut input = input.iter();

    /*
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines();
    */

    let total = input.map(|line| process_rucksack(&line)).sum::<u32>();

    dbg!(total);

    Ok(())
}
