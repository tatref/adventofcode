use std::error::Error;

use adventofcode2023::load_string;

fn compute(input: &str) -> u32 {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .collect();

    numbers.iter().sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/01.txt")?;

    let result = compute(&input);
    dbg!(result);

    Ok(())
}

#[test]
fn day01_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let result = compute(input);

    assert_eq!(result, 12 + 38 + 15 + 77);
}
