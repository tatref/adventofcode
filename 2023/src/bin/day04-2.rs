use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

use adventofcode2023::load_string;

struct Card {
    id: usize,
    have: HashSet<u64>,
    winning: HashSet<u64>,
}

impl Card {
    fn get_matches(&self) -> usize {
        let winning_numbers: HashSet<_> = self.have.intersection(&self.winning).collect();
        let count = winning_numbers.len();

        count
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

fn compute(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();
    let matches_cache: Vec<usize> = cards.iter().map(|c| c.get_matches()).collect(); // memoization

    let mut stack: VecDeque<usize> = cards.iter().map(|c| c.id - 1).collect();
    let mut processed: VecDeque<usize> = VecDeque::new();

    while let Some(card_id) = stack.pop_back() {
        //let matches = cards[card_id].get_matches();
        // memoization
        let matches = matches_cache[card_id];

        if matches > 0 {
            let next_cards: Vec<usize> = ((card_id + 1)..(card_id + 1 + matches)).collect();
            for next_card in next_cards.iter() {
                stack.push_front(*next_card);
            }
        }

        processed.push_back(card_id);
    }

    processed.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/04.txt")?;

    let result = compute(&input);
    dbg!(result);

    Ok(())
}

#[test]
fn day04_2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut stack: VecDeque<usize> = cards.iter().map(|c| c.id - 1).collect();
    let mut processed: VecDeque<usize> = VecDeque::new();

    while let Some(card_id) = stack.pop_back() {
        let matches = cards[card_id].get_matches();

        if matches > 0 {
            let next_cards: Vec<usize> = ((card_id + 1)..(card_id + 1 + matches)).collect();
            for next_card in next_cards.iter() {
                stack.push_front(*next_card);
            }
        }

        processed.push_back(card_id);
    }

    assert_eq!(processed.len(), 30);
}
