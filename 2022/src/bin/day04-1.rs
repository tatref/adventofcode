#![feature(iter_array_chunks)]
#![feature(array_chunks)]

use std::{error::Error, ops::RangeInclusive};

use adventofcode2022::load_vec;

fn process_line(line: &str) -> u32 {
    println!();
    dbg!(line);

    let mut range_a = line
        .split(',')
        .next()
        .unwrap()
        .split('-')
        .map(|v| v.parse::<u32>().expect("Not a number"));
    let mut range_b = line
        .split(',')
        .nth(1)
        .unwrap()
        .split('-')
        .map(|v| v.parse::<u32>().expect("Not a number"));

    let range_a = range_a.next().unwrap()..=range_a.next().unwrap();
    let range_b = range_b.next().unwrap()..=range_b.next().unwrap();

    dbg!(&range_a, &range_b);
    dbg!(&range_a.start(), &range_b.start());
    dbg!(&range_a.end(), &range_b.end());

    fn contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
        dbg!(&a, &b);

        if a.start() <= b.start() && a.end() >= b.end() {
            true
        } else {
            false
        }
    }

    if contains(&range_a, &range_b) || contains(&range_b, &range_a) {
        1
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/04.txt")?;
    let mut input = input.iter().map(|s| s.as_str());

    /*
        let input = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "
        .lines();
    */

    let total = input.map(|line| process_line(line)).sum::<u32>();

    dbg!(total);

    Ok(())
}
