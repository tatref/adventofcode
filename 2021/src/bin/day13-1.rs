#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

#[derive(Debug)]
struct Fold {
    axis: String,
    index: i32,
}

#[derive(Debug)]
struct Paper {
    dots: Vec<(i32, i32)>,
    size: (i32, i32),
}

impl Paper {
    fn fold(&mut self, fold: &Fold) {
        if fold.axis == "y" {
            for dot in self.dots.iter_mut() {
                if dot.1 < fold.index {
                    continue;
                }
                dot.1 = 2 * fold.index - dot.1;
            }

            for (i, dot) in self.dots.iter().enumerate() {
                assert!(
                    dot.1 < fold.index,
                    "dot.1 < fold.index: {} < {}",
                    dot.1,
                    fold.index
                );
            }

            self.size.1 = fold.index;
        } else if fold.axis == "x" {
            for dot in self.dots.iter_mut() {
                if dot.0 < fold.index {
                    continue;
                }
                dot.0 = 2 * fold.index - dot.0;
            }

            for (i, dot) in self.dots.iter().enumerate() {
                assert!(
                    dot.0 < fold.index,
                    "dot.0 < fold.index: {} < {}",
                    dot.0,
                    fold.index
                );
            }

            self.size.0 = fold.index;
        } else {
            unreachable!();
        }

        dbg!(self.dots.len());
        self.dots = self
            .dots
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .cloned()
            .collect();
        dbg!(self.dots.len());
    }
}

fn parse(input: &str) -> (Paper, Vec<Fold>) {
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let mut iter = lines.split(|line| line == "");

    let dots = iter.next().unwrap();
    let folds = iter.next().unwrap();

    let dots: Vec<(i32, i32)> = dots
        .iter()
        .map(|line| {
            let mut it = line.split(',');
            let x: i32 = it.next().unwrap().parse().unwrap();
            let y: i32 = it.next().unwrap().parse().unwrap();

            (x, y)
        })
        .collect();

    let max_x = dots.iter().map(|&(x, _y)| x).max().unwrap();
    let max_y = dots.iter().map(|&(_x, y)| y).max().unwrap();

    let size = (max_x, max_y);

    let folds = folds
        .iter()
        .map(|line| {
            let it = line.split_ascii_whitespace();
            let value = it.last().unwrap();
            let axis = value.split('=').next().unwrap().to_string();
            let index = value.split('=').last().unwrap().parse().unwrap();

            Fold { axis, index }
        })
        .collect();

    (Paper { dots, size }, folds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/13.txt")?;

    /*
    let input = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5";
    */

    let (mut paper, folds) = parse(&input);

    for fold in &folds {
        paper.fold(fold);
        // dbg!(paper.dots.len());
    }
    display(&paper);

    fn display(paper: &Paper) {
        for y in 0..paper.size.1 {
            for x in 0..paper.size.0 {
                if paper.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }

    Ok(())
}
