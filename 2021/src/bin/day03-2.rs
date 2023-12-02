use std::error::Error;

use adventofcode2021::{load_sample_vec, load_vec};

fn most_common(input: &Vec<String>, bit: usize, prefer_zero: bool) -> char {
    let mut counter = (0, 0);

    for line in input {
        let c = line.chars().nth(bit).unwrap();
        match c {
            '0' => counter.0 += 1,
            '1' => counter.1 += 1,
            x => unreachable!("unknown char: {:?}", x),
        }
    }

    match (counter.0.cmp(&counter.1), prefer_zero) {
        (std::cmp::Ordering::Equal, false) => '1',
        (std::cmp::Ordering::Equal, true) => '0',
        (std::cmp::Ordering::Greater, false) => '0',
        (std::cmp::Ordering::Greater, true) => '1',
        (std::cmp::Ordering::Less, false) => '1',
        (std::cmp::Ordering::Less, true) => '0',
    }
}

fn get_value(input: &Vec<String>, prefer_zero: bool) -> u32 {
    let mut input = input.clone();

    let mut current_bit = 0;
    while input.len() != 1 {
        let common = most_common(&input, current_bit, prefer_zero);

        input = input
            .iter()
            .filter(|line| line.chars().nth(current_bit).unwrap() == common)
            .cloned()
            .collect();

        current_bit += 1;
    }

    let input = input.pop().unwrap();

    u32::from_str_radix(&input, 2).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_vec("inputs/03.txt")?;
    /*
    let mut input = load_sample(
        "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010",
    )?;
    */

    let oxygen = get_value(&input, false);
    let co2 = get_value(&input, true);

    dbg!(oxygen, co2);
    dbg!(oxygen * co2);

    Ok(())
}
