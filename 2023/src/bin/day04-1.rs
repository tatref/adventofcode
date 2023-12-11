use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

use adventofcode2023::load_string;

struct Card {
    id: u64,
    have: HashSet<u64>,
    winning: HashSet<u64>,
}

impl Card {
    fn get_points(&self) -> u64 {
        let winning_numbers: HashSet<_> = self.have.intersection(&self.winning).collect();
        let count = winning_numbers.len() as u32;

        let points = if count > 0 { 2u64.pow(count - 1) } else { 0 };

        points
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');

        let id = split.next().unwrap();
        let id = id.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();

        let mut split = split.next().unwrap().split('|');

        let have: HashSet<u64> = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let winning: HashSet<u64> = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Card { id, have, winning })
    }
}

fn compute(input: &str) -> u64 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/04.txt")?;

    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();
    let points: u64 = cards.iter().map(|card| card.get_points()).sum();
    dbg!(points);

    Ok(())
}

#[test]
fn day04_points_1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

    let card: Card = input.parse().unwrap();
    let points = card.get_points();

    assert_eq!(points, 8);
}

#[test]
fn day04_points_4() {
    let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";

    let card: Card = input.parse().unwrap();
    let points = card.get_points();

    assert_eq!(points, 1);
}

#[test]
fn day04_parse_card() {
    let input = "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();
}

#[test]
fn day04_example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();

    let points: u64 = cards.iter().map(|card| card.get_points()).sum();

    assert_eq!(points, 13);
}
