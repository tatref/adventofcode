#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn is_opening(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '<' => true,
        '{' => true,
        x => false,
    }
}

fn is_closing(c: char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '>' => true,
        '}' => true,
        x => false,
    }
}

fn get_closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        x => panic!("Unknown char: {}", x),
    }
}

fn complete(input: &str) -> Option<Vec<char>> {
    dbg!(input);
    let mut stack = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let expected = get_closing(*stack.last().unwrap());

            if expected == c {
                stack.pop();
            } else {
                // corrupted case
                return None;
            }
        }
    }

    let complete = stack.into_iter().rev().map(get_closing).collect();

    Some(complete)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/10.txt")?;

    /*
    let input = "[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]";
    */

    let points_map: HashMap<char, i64> = [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();
    let mut the_scores = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        let mut score = 0;

        match complete(line) {
            Some(complete) => {
                println!("{}: {:?}", line, complete);
                for c in complete {
                    score *= 5;
                    score += points_map[&c];
                }

                dbg!(score);
                the_scores.push(score);
            }
            None => (),
        }
    }

    the_scores.sort();
    let mid = the_scores.len() / 2;
    let winner = the_scores[mid];

    dbg!(winner);

    Ok(())
}
