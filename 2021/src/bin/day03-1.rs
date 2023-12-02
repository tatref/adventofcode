use std::error::Error;

use adventofcode2021::{load_sample_vec, load_vec};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_vec("inputs/03.txt")?;
    /*
        let input = load_sample(
            "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010",
        )?;
        */

    let bit_length = input[0].len();
    dbg!(bit_length);

    let mut result = vec![(0, 0); bit_length];
    for line in input {
        for (idx, c) in line.char_indices() {
            match c {
                '0' => result[idx].0 += 1,
                '1' => result[idx].1 += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (idx, (bit_0, bit_1)) in result.iter().enumerate() {
        //dbg!(idx);
        let gamma_bit = if bit_0 > bit_1 { 0 } else { 1 };
        let epsilon_bit = if bit_0 > bit_1 { 1 } else { 0 };
        gamma += 2u32.pow(bit_length as u32 - 1 - idx as u32) * gamma_bit;
        epsilon += 2u32.pow(bit_length as u32 - 1 - idx as u32) * epsilon_bit;
    }

    dbg!(gamma);
    dbg!(epsilon);

    dbg!(gamma * epsilon);

    Ok(())
}
