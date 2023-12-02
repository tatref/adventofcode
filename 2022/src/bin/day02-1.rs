use std::{cmp::Ordering, error::Error, str::FromStr};

use adventofcode2022::load_vec;

#[derive(PartialEq)]
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
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
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
    let b: Shape = splt.next().unwrap().parse().unwrap();

    let result = match a.partial_cmp(&b).unwrap() {
        Ordering::Equal => 3,
        Ordering::Less => 6,
        Ordering::Greater => 0,
    };

    let shape_points = match b {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    dbg!(shape_points, result);

    result + shape_points
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
