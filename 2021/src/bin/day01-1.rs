use std::error::Error;

use adventofcode2021::load_vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/01.txt")?;
    let input: Vec<u32> = input.iter().map(|s| s.parse().unwrap()).collect();

    let mut last = None;
    let mut counter = 0;

    for line in input {
        if last.is_some() && line > last.unwrap() {
            counter += 1;
        }
        last = Some(line);
    }

    dbg!(counter);

    Ok(())
}
