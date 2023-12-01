#![feature(try_from)]

use std::convert::TryFrom;
use std::collections::HashSet;


mod adventofcode;


use adventofcode::load_file;


#[derive(Debug,Eq,PartialEq,Clone,Hash)]
struct Memory {
    memory_blocks: Vec<isize>,
}

impl Memory {
    fn balance_one(&mut self) {
        let max = *self.memory_blocks.iter().max().unwrap();
        let max_idx = self.memory_blocks.iter().position(|x| x == &max).unwrap();

        self.memory_blocks[max_idx] = 0;

        let mut idx = (max_idx + 1) % self.memory_blocks.len();
        for _ in 0..max {
            self.memory_blocks[idx] += 1;
            idx = (idx + 1) % self.memory_blocks.len();
        }
    }

    fn run(&mut self) -> usize {
        let mut seen_distributions = HashSet::new();

        let mut i = 1;
        loop {
            seen_distributions.insert(self.clone());
            self.balance_one();

            if seen_distributions.contains(self) {
                return i;
            }

            i += 1;
        }
    }
}


impl <'a> TryFrom<&'a str> for Memory {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut memory_blocks = Vec::new();

        for blocks in value.split_whitespace() {
            memory_blocks.push(blocks.parse::<isize>().unwrap());
        }

        Ok(
            Self {
                memory_blocks,
            }
          )
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::convert::TryFrom;

    use Memory;

    #[test]
    fn parse() {
        let memory = "0 2 7 0".try_into();

        let expected = Ok(
            Memory {
                memory_blocks: vec![0, 2, 7, 0],
            }
            );

        assert_eq!(memory, expected);
    }

    macro_rules! test_memory {
        ($a:ident, $b:expr, $c:expr) => {
            #[test]
            fn $a () {
                let mut memory = Memory::try_from($b).unwrap();
                memory.balance_one();

                let expected = Memory::try_from($c).unwrap();

                assert_eq!(expected, memory);
            }
        }
    }

    test_memory!(test1, "0 2 7 0", "2 4 1 2");
    test_memory!(test2, "2 4 1 2", "3 1 2 3");
    test_memory!(test3, "3 1 2 3", "0 2 3 4");
    test_memory!(test4, "0 2 3 4", "1 3 4 1");
    test_memory!(test5, "1 3 4 1", "2 4 1 2");

    #[test]
    fn run() {
        let mut memory: Memory = "0 2 7 0".try_into().unwrap();

        let i = memory.run();

        assert_eq!(i, 5);
    }

}



fn main() {
    let input = load_file("inputs/06-1.txt").unwrap();
    let mut memory = Memory::try_from(input.as_ref()).unwrap();

    let i = memory.run();

    println!("{}", i);
}
