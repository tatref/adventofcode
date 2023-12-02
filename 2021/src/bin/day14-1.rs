#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let mut iter = lines.split(|line| line == "");

    let polymer: Vec<char> = iter.next().unwrap()[0].clone().chars().collect();
    let insertion_rules: HashMap<(char, char), char> = iter
        .next()
        .unwrap()
        .iter()
        .map(|s| {
            let mut it = s.chars();
            let between = (it.next().unwrap(), it.next().unwrap());
            let to = it.last().unwrap();
            (between, to)
        })
        .collect();

    (polymer, insertion_rules)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/14.txt")?;

    let input = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    let (mut polymer, insertion_rules) = parse(&input);

    dbg!(&polymer);

    fn step(polymer: &Vec<char>, insertion_rules: &HashMap<(char, char), char>) -> Vec<char> {
        let mut result = polymer.clone();

        let mut i = 0;
        while i < result.len() - 1 {
            let between = (result[i], result[i + 1]);

            if let Some(c) = insertion_rules.get(&between) {
                result.insert(i + 1, *c);
                i += 1;
            }

            i += 1;
        }

        result
    }

    println!("{:?}", &polymer.iter().collect::<String>());
    for i in 0..10 {
        polymer = step(&polymer, &insertion_rules);
        //println!("{:?}", &polymer.iter().collect::<String>());
        dbg!(i, &polymer.len());
    }

    let stats = polymer.iter().fold(HashMap::new(), |mut acc, e| {
        *acc.entry(e).or_insert(0) += 1;
        acc
    });

    dbg!(&stats);

    let most_common = stats.iter().max_by_key(|(a, b)| *b).unwrap().1;
    let least_common = stats.iter().min_by_key(|(a, b)| *b).unwrap().1;

    dbg!(most_common, least_common);

    dbg!(most_common - least_common);

    Ok(())
}
