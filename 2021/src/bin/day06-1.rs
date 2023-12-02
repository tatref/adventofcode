use std::{collections::HashMap, error::Error, str::FromStr, string::ParseError};

use adventofcode2021::load_string;

#[derive(Debug)]
struct Fish {
    new: bool,
    timer: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/06.txt")?;
    let mut input = "3,4,3,1,2";

    let mut fishes: Vec<Fish> = input
        .split(',')
        .map(|s| Fish {
            new: false,
            timer: s.parse().unwrap(),
        })
        .collect();
    for f in &fishes {
        print!("{},", f.timer);
    }

    println!();
    for i in 0..80 {
        dbg!(i);

        let mut new_fishes = Vec::new();
        for fish in fishes.iter_mut() {
            if fish.timer > 0 {
                let modulo = if fish.new { 8 } else { 7 };
                fish.timer = (fish.timer - 1) % modulo;
            } else {
                fish.timer = 6;
                new_fishes.push(Fish {
                    new: true,
                    timer: 8,
                });
            }
        }
        fishes.extend(new_fishes);

        //for f in &fishes {
        //    print!("{},", f.timer);
        //}
        //println!();

        dbg!(&fishes.len());
    }

    Ok(())
}
