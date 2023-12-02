use std::error::Error;

use adventofcode2021::load_vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/02.txt")?;
    let input: Vec<_> = input
        .iter()
        .map(|s| {
            let mut iter = s.split_ascii_whitespace();
            let cmd = iter.next().unwrap();
            let dist: i32 = iter.next().unwrap().parse().unwrap();

            (cmd, dist)
        })
        .collect();

    let mut pos = (0, 0);
    for (cmd, dist) in input {
        let dir = match cmd {
            "forward" => (1, 0),
            "down" => (0, 1),
            "up" => (0, -1),
            _ => unreachable!(),
        };

        pos.0 += dir.0 * dist;
        pos.1 += dir.1 * dist;
    }

    dbg!(pos);

    dbg!(pos.0 * pos.1);

    Ok(())
}
