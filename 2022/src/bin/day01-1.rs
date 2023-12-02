use std::error::Error;

use adventofcode2022::load_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/01.txt")?;
    let result = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .max();

    dbg!(result);

    Ok(())
}
