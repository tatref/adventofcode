use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use adventofcode2021::load_string;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/08.txt")?;

    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";

    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf";

    for line in input.lines() {
        let unique_signal: Vec<_> = line
            .split('|')
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|w| {
                let v: HashSet<char>;
                v = w.chars().collect();
                v
            })
            .collect();
        let output: HashSet<_> = line
            .split('|')
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|w| {
                let v: Vec<char>;
                v = w.chars().collect();
                v
            })
            .collect();

        let mut standard_display: HashMap<u8, HashSet<char>> = HashMap::new();
        standard_display.insert(0, "abcefg".chars().collect());
        standard_display.insert(1, "cf".chars().collect());
        standard_display.insert(2, "acdeg".chars().collect());
        standard_display.insert(3, "acdfg".chars().collect());
        standard_display.insert(4, "bcdf".chars().collect());
        standard_display.insert(5, "abdfg".chars().collect());
        standard_display.insert(6, "abdefg".chars().collect());
        standard_display.insert(7, "acf".chars().collect());
        standard_display.insert(8, "abcdefg".chars().collect());
        standard_display.insert(9, "abcdfg".chars().collect());

        fn map(
            from: &HashMap<u8, HashSet<char>>,
            mapping: &Vec<char>,
        ) -> HashMap<u8, HashSet<char>> {
            HashMap::new()
        }

        let perms = "abcdefg".chars().permutations(7);
        let letters: Vec<_> = "abcdefg".chars().enumerate().collect();
        for perm in perms {
            let mut display = standard_display.clone();
            for v in display.values_mut() {
                let mut new_v = HashSet::new();
                for letter in v.iter() {
                    let idx = letters
                        .iter()
                        .find_map(|(i, c)| if *c == *letter { Some(*i) } else { None })
                        .unwrap();
                    new_v.insert(perm[idx]);
                }

                *v = new_v;
            }

            break;
        }

        for w in unique_signal.iter() {}
    }

    Ok(())
}
