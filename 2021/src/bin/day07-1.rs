use std::{error::Error, sync::atomic::compiler_fence};

use adventofcode2021::load_string;

fn compute_value(numbers: &Vec<i64>, point: i64) -> i64 {
    let mut total = 0;

    for n in numbers {
        total += (n - point).abs();
    }

    total
}

fn solve(numbers: &Vec<i64>) -> i64 {
    let min: i64 = *numbers.first().unwrap();
    let max: i64 = *numbers.last().unwrap();

    let mut min_fuel = i64::MAX;
    for p in min..=max {
        let val = compute_value(numbers, p);
        min_fuel = min_fuel.min(val);
    }

    min_fuel
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/07.txt")?;
    //let mut input = "16,1,2,0,4,2,7,1,2,14";

    let mut numbers: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();

    numbers.sort();

    dbg!(numbers.first().unwrap());
    dbg!(numbers.last().unwrap());
    dbg!(numbers.len());

    dbg!(compute_value(&numbers, 2));
    dbg!(compute_value(&numbers, 10));
    dbg!(compute_value(&numbers, 100));

    dbg!(solve(&numbers));

    Ok(())
}
