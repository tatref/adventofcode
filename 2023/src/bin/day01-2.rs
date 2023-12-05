use std::error::Error;

use adventofcode2023::load_string;

fn get_number_indices(line: &str, number: &str, value: u32) -> Vec<(usize, u32)> {
    let line = line.as_bytes();
    let number = number.as_bytes();
    let mut result = Vec::new();

    for (idx, substr) in line.windows(number.len()).enumerate() {
        if substr == number {
            result.push((idx, value));
        }
    }

    result
}

fn get_numbers(line: &str) -> (u32, u32) {
    let mut indices: Vec<(usize, u32)> = Vec::new();

    for (number, value) in &[
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        let number_indices: Vec<(usize, u32)> = get_number_indices(line, number, *value);

        indices.extend(&number_indices);
    }

    indices.sort_by_key(|(idx, _value)| *idx);

    (indices.first().unwrap().1, indices.last().unwrap().1)
}

fn compute(input: &str) -> u32 {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| {
            let numbers = get_numbers(line);
            numbers.0 * 10 + numbers.1
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
fn day01_part1_line() {
    let line = "two1nine";

    let indices = get_number_indices(line, "two", 2);
    assert_eq!(indices, vec![(0, 2)]);

    let indices = get_number_indices(line, "1", 1);
    assert_eq!(indices, vec![(3, 1)]);

    let indices = get_number_indices(line, "nine", 9);
    assert_eq!(indices, vec![(4, 9)]);

    let numbers = get_numbers(line);
    assert_eq!(numbers, (2, 9));
}

#[test]
fn day01_part1_line_bis() {
    let numbers = get_numbers("eightwothree");
    assert_eq!(numbers, (8, 3));
}

#[test]
fn day01_part1() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    for line in input.lines() {
        let numbers = get_numbers(line);
        dbg!(numbers);
    }

    let result = compute(input);

    assert_eq!(result, 29 + 83 + 13 + 24 + 42 + 14 + 76);
}
