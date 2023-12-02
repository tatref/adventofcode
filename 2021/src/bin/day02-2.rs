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
    let mut aim = 0;
    for (cmd, dist) in input {
        match cmd {
            "down" => aim += dist,
            "up" => aim -= dist,
            "forward" => {
                pos.0 += dist;
                pos.1 += aim * dist;
            }
            _ => unreachable!(),
        };
    }

    dbg!(pos);

    dbg!(pos.0 * pos.1);

    Ok(())
}
