use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, error::Error, str::FromStr};

use adventofcode2023::load_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Card(u8);

impl Card {
    fn from_char(c: char) -> Result<Self, String> {
        let val = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            x => return Err(format!("Unknown card: {:?}", x)),
        };
        Ok(Card(val))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(());
        } else {
            let hand: Vec<_> = s.chars().map(|c| Card::from_char(c).unwrap()).collect();

            let result = Hand(hand.try_into().unwrap());

            Ok(result)
        }
    }
}

impl Hand {
    fn get_poker_type(&self) -> PokerType {
        let mut counts: HashMap<Card, u8> = HashMap::new();

        for c in self.0.iter() {
            counts.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        }

        let values: Vec<u8> = counts.values().sorted().rev().copied().collect();

        let result = match values.as_slice() {
            [5] => PokerType::FiveOfAKind,
            [4, 1] => PokerType::FourOfAKind,
            [3, 2, ..] => PokerType::FullHouse,
            [3, ..] => PokerType::ThreeOfAKind,
            [2, 2, ..] => PokerType::TwoPair,
            [2, ..] => PokerType::OnePair,
            [1, 1, 1, 1, 1] => PokerType::HighCard,
            x => unreachable!("Can't find poker type: {:?}", x),
        };

        result
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_poker_type().cmp(&other.get_poker_type()) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Equal => (),
        }

        // high card
        for (me, other) in self.0.iter().zip(other.0.iter()) {
            match me.cmp(other) {
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Equal => (),
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PokerType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn compute(input: &str) -> u32 {
    let mut bids: Vec<Bid> = input
        .lines()
        .map(|line| line.parse::<Bid>().unwrap())
        .collect();
    bids.sort_by_key(|bid| bid.hand);

    let mut result = 0;

    for (rank, bid) in bids.iter().enumerate() {
        result += (rank as u32 + 1) * bid.bid;
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl FromStr for Bid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let hand = split.next().unwrap();
        let bid = split.next().unwrap();

        let hand: Hand = hand.parse().unwrap();
        let bid: u32 = bid.parse().unwrap();

        Ok(Bid { hand, bid })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/07.txt")?;

    let result = compute(&input);
    dbg!(result);

    Ok(())
}

#[test]
fn day07_parse_bid() {
    let input = "32T3K 765";

    let result: Bid = input.parse().unwrap();

    assert_eq!(
        result,
        Bid {
            hand: Hand([Card(3), Card(2), Card(10), Card(3), Card(13)]),
            bid: 765
        }
    );
}

#[test]
fn day07_parse_hand() {
    let input = "32T3K";

    let result: Hand = input.parse().unwrap();

    assert_eq!(
        result,
        Hand([Card(3), Card(2), Card(10), Card(3), Card(13)])
    );
}

#[test]
fn day07_poker_type() {
    let input = [
        ("AAAAA", PokerType::FiveOfAKind),
        ("AA8AA", PokerType::FourOfAKind),
        ("23332", PokerType::FullHouse),
        ("TTT98", PokerType::ThreeOfAKind),
        ("23432", PokerType::TwoPair),
        ("A23A4", PokerType::OnePair),
        ("23456", PokerType::HighCard),
    ];

    for &(hand, expected_poker_type) in input.iter() {
        let hand: Hand = hand.parse().unwrap();
        let poker_type = hand.get_poker_type();
        assert_eq!(poker_type, expected_poker_type);
    }
}

#[test]
fn day07_part1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let result = compute(input);

    assert_eq!(result, 6440);
}
