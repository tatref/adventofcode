use std::error::Error;

use adventofcode2021::load_vec;

fn main() -> Result<(), Box<dyn Error>> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let input = load_vec("inputs/01.txt")?;
    let input: Vec<u32> = input.iter().map(|s| s.parse().unwrap()).collect();

    let input: Vec<u32> = input.windows(3).map(|abc| abc.iter().sum()).collect();

    let mut last = None;
    let mut counter = 0;

    for line in input {
        if last.is_some() && line > last.unwrap() {
            counter += 1;
        }
        last = Some(line);
    }

    dbg!(counter);
    dbg!(v);

    Ok(())
}
