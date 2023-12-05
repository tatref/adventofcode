use glam::i32::IVec2;
use itertools::Itertools;
use std::{error::Error, str::FromStr};

use adventofcode2023::load_string;

#[derive(Debug)]
struct Schematic {
    numbers: Vec<(u32, IVec2, IVec2)>,
    symbols: Vec<IVec2>,
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut all_numbers = Vec::new();
        let mut all_symbols = Vec::new();

        for (line_number, line) in s.lines().enumerate() {
            let symbols: Vec<_> = line
                .chars()
                .enumerate()
                .group_by(|(_idx, c)| !c.is_digit(10) && *c != '.')
                .into_iter()
                .map(|(val, group)| (val, group.into_iter().collect::<Vec<_>>()))
                .collect();
            let symbols: Vec<IVec2> = symbols
                .iter()
                .filter_map(|(is_symbol, values)| {
                    if *is_symbol {
                        Some(
                            values
                                .iter()
                                .map(|(x, _sym)| IVec2::new(*x as i32, line_number as i32)),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();
            all_symbols.extend(&symbols);

            let numbers: Vec<_> = line
                .chars()
                .enumerate()
                .group_by(|(_idx, c)| c.is_digit(10))
                .into_iter()
                .map(|(val, group)| (val, group.into_iter().collect::<Vec<_>>()))
                .collect();
            let numbers: Vec<(u32, IVec2, IVec2)> = numbers
                .iter()
                .filter_map(|(is_number, values)| {
                    if *is_number {
                        let start = values.first().unwrap();
                        let end = values.last().unwrap();
                        let start = IVec2::new(start.0 as i32, line_number as i32);
                        let end = IVec2::new(end.0 as i32, line_number as i32);

                        let number: u32 = values
                            .iter()
                            .rev()
                            .map(|(_idx, n)| n.to_digit(10).unwrap())
                            .enumerate()
                            .map(|(a, b)| 10u32.pow(a as u32) * b)
                            .sum();

                        Some((number, start, end))
                    } else {
                        None
                    }
                })
                .collect();

            all_numbers.extend(&numbers);
        }

        Ok(Schematic {
            numbers: all_numbers,
            symbols: all_symbols,
        })
    }
}

fn compute(input: &str) -> u32 {
    let schematic: Schematic = input.parse().unwrap();

    let mut sum = 0;
    for &number in &schematic.numbers {
        let mut is_part_number = false;
        for offset in number.1.x..=number.2.x {
            let p = IVec2::new(offset, number.1.y);
            for &symbol in &schematic.symbols {
                if (p - symbol).length_squared() <= 2 {
                    is_part_number = true;
                }
            }
        }

        if is_part_number {
            sum += number.0;
        } else {
        }

        if number.0 > 999 {
            panic!("{:?}", number);
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_string("inputs/03.txt")?;

    let result = compute(&input);
    dbg!(result);

    Ok(())
}

#[test]
fn day03_part1() {
    let input = "467..114..
...*......
..35..633.
......#*..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let result = compute(input);

    assert_eq!(result, 4361);
}
