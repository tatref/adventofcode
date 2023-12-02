#![feature(iter_array_chunks)]
#![feature(array_chunks)]

use std::{error::Error, str::FromStr};

use adventofcode2022::load_string;

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let count = split.nth(1).unwrap().parse().unwrap();
        let from = split.nth(1).unwrap().parse().unwrap();
        let to = split.nth(1).unwrap().parse().unwrap();

        Ok(Self { count, from, to })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/05.txt")?;

    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let mut stacks_input = input.split("\n\n").next().unwrap().lines().rev();
    let moves = input.split("\n\n").nth(1).unwrap();

    dbg!(moves);

    let first_line = stacks_input.next().unwrap();
    let n_stacks: usize = first_line
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];
    for line in stacks_input {
        for i in 0..n_stacks {
            let idx = i * 4 + 1;
            let c = line.chars().nth(idx).unwrap();

            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    dbg!(stacks);

    let moves: Vec<Move> = moves.lines().map(|m| m.parse().unwrap()).collect();
    for Move { from, to, count } in moves.iter() {
        stacks[from].drain()
    }

    Ok(())
}
