use std::{cmp::Ordering, error::Error, str::FromStr};

use adventofcode2022::load_vec;

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().ok_or(())?;

        let val = match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => return Err(()),
        };

        Ok(val)
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Shape::*;

        let cmp = match (self, other) {
            (Rock, Rock) => Ordering::Equal,
            (Paper, Paper) => Ordering::Equal,
            (Scissors, Scissors) => Ordering::Equal,
            (Rock, Paper) => Ordering::Less,
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Paper, Scissors) => Ordering::Less,
            (Scissors, Rock) => Ordering::Less,
            (Scissors, Paper) => Ordering::Greater,
        };

        Some(cmp)
    }
}

fn round_points(line: &str) -> u32 {
    let mut splt = line.split_whitespace();
    let a: Shape = splt.next().unwrap().parse().unwrap();
    let game_result: Ordering = match splt.next().unwrap().chars().next().unwrap() {
        'X' => Ordering::Less,
        'Y' => Ordering::Equal,
        'Z' => Ordering::Greater,
        _ => panic!("Can't parse game_result"),
    };

    for choice in &[Shape::Rock, Shape::Paper, Shape::Scissors] {
        if choice.partial_cmp(&a).unwrap() == game_result {
        } else {
            continue;
        }

        println!("I choose {:?}", choice);

        let result = match game_result.reverse() {
            Ordering::Equal => 3,
            Ordering::Less => 6,
            Ordering::Greater => 0,
        };

        let shape_points = match &choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        dbg!(shape_points, result);

        return result + shape_points;
    }

    unreachable!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/02.txt")?;
    let mut input = input.iter();

    /*
        let input = "A Y
            B X
            C Z"
        .lines();
    */

    let total = &input.map(|line| round_points(line)).sum::<u32>();
    dbg!(total);

    Ok(())
}
