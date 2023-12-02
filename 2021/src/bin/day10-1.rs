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

fn get_closing(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        x => panic!("Unknown char: {}", x),
    }
}

fn validate(input: &str) -> Option<char> {
    dbg!(input);
    let mut stack = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        //dbg!(&stack);

        if is_opening(c) {
            stack.push(c);
        } else {
            let expected = get_closing(stack.last().unwrap());

            if expected == c {
                stack.pop();
            } else {
                //println!("{}: expected: {:?}, got: {:?}", idx, expected, c);
                return Some(c);
            }
        }
    }

    None
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

    let points_map: HashMap<char, i32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    let mut points = 0;
    for line in input.lines() {
        let line = line.trim();

        match validate(line) {
            Some(c) => {
                let some_points = points_map[&c];
                points += some_points;
                println!("{}: {}", line, c);
            }
            None => (),
        }
    }

    dbg!(points);

    Ok(())
}
