use std::error::Error;

use adventofcode2022::load_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/01.txt")?;
    let mut result: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    result.sort();
    result.reverse();

    dbg!(result[..3].iter().sum::<u32>());

    Ok(())
}
