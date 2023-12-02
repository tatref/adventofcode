use std::{collections::HashMap, error::Error, str::FromStr, string::ParseError};

use adventofcode2021::load_string;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Fish {
    new: bool,
    timer: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/06.txt")?;
    //let mut input = "3,4,3,1,2";

    let mut input: Vec<Fish> = input
        .split(',')
        .map(|s| Fish {
            new: false,
            timer: s.parse().unwrap(),
        })
        .collect();

    let mut fishes = [0u64; 9];
    for f in input.iter() {
        fishes[f.timer as usize] += 1;
    }

    for i in 0..256 {
        dbg!(i);

        let mut new_fishes = [0u64; 9];
        new_fishes[8] = fishes[0];
        new_fishes[7] = fishes[8];
        new_fishes[6] = fishes[7] + fishes[0];
        new_fishes[5] = fishes[6];
        new_fishes[4] = fishes[5];
        new_fishes[3] = fishes[4];
        new_fishes[2] = fishes[3];
        new_fishes[1] = fishes[2];
        new_fishes[0] = fishes[1];

        fishes = new_fishes;
    }

    dbg!(&fishes.iter().sum::<u64>());

    Ok(())
}
