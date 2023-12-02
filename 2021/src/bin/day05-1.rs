use std::{collections::HashMap, error::Error, str::FromStr, string::ParseError};

use adventofcode2021::load_string;

#[derive(Debug, Clone)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_non_diag(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn iter(&self) -> Vec<(i32, i32)> {
        dbg!(&self);

        let mut result = Vec::new();

        if self.start.0 == self.end.0 {
            let a = self.start.1.min(self.end.1);
            let b = self.start.1.max(self.end.1);
            for y in a..=b {
                let p = (self.start.0, y);
                result.push(p);
            }
            return result;
        }

        if self.start.1 == self.end.1 {
            let a = self.start.0.min(self.end.0);
            let b = self.start.0.max(self.end.0);
            for x in a..=b {
                let p = (x, self.start.1);
                result.push(p);
            }
            return result;
        }

        let counter = (self.start.0 - self.end.0).abs();
        let x_dir = (self.end.0 - self.start.0) / counter;
        let y_dir = (self.end.1 - self.start.1) / counter;
        for i in 0..=counter {
            let p = (self.start.0 + x_dir * i, self.start.1 + y_dir * i);
            result.push(p);
        }

        result
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" -> ");

        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        let start = str_to_tuple(start);
        let end = str_to_tuple(end);

        fn str_to_tuple(s: &str) -> (i32, i32) {
            let mut it = s.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();

            (x, y)
        }

        Ok(Self { start, end })
    }
}

fn parse_input(s: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut input = s.lines().map(|s| s.trim());

    Ok(input.map(|s| Line::from_str(s).unwrap()).collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/05.txt")?;
    /*
    let mut input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        */

    let lines = parse_input(&input).unwrap();

    let mut board = HashMap::new();

    for line in lines.iter() {
        let points = line.iter();
        for p in points {
            let ptr = board.entry(p).or_insert(0);
            *ptr += 1;
        }
    }

    let num_dangerous_zones = board.iter().filter(|(k, v)| **v >= 2).count();

    dbg!(num_dangerous_zones);

    Ok(())
}
