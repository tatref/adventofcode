#![feature(try_from)]

use std::convert::TryFrom;


mod adventofcode;


use adventofcode::load_file;


#[derive(Debug,PartialEq,Clone)]
struct Cpu {
    instructions: Vec<isize>,
    ptr: usize,
}

impl Cpu {
    fn next(&self) -> Option<Self> {
        let next_instruction = (self.ptr as isize + self.instructions[self.ptr]) as usize;
        let mut next_instructions_list = self.instructions.clone();

        next_instructions_list[self.ptr] += 1;


        if next_instruction >= next_instructions_list.len() {
            None
        }
        else {
            Some(
                Cpu {
                    instructions: next_instructions_list,
                    ptr: next_instruction,
                }
                )
        }
    }

    fn escape(&self) -> usize {
        let mut cpu = self.clone();

        let mut i = 1;
        while true {
            match cpu.next() {
                Some(_cpu) => {
                    cpu = _cpu.clone();
                    i += 1;
                },
                None => return i,
            };
        }

        i
    }
}


impl <'a> TryFrom<&'a str> for Cpu {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ptr = 0;
        let mut instructions = Vec::new();

        for (idx, instruction) in value.split_whitespace().enumerate() {
            if instruction.starts_with("(") {
                ptr = idx;

                let instruction = &instruction[1..(instruction.len() - 1)];
                instructions.push(instruction.parse::<isize>().unwrap());
            }
            else {
                instructions.push(instruction.parse::<isize>().unwrap());
            }
        }

        Ok(
            Cpu {
                instructions,
                ptr,
            }
          )
    }
}


mod tests {
    use std::convert::TryInto;
    use std::convert::TryFrom;

    use Cpu;

    #[test]
    fn parse() {
        let cpu = "(0) 3  0  1  -3".try_into();

        let expected = Ok(Cpu {
            instructions: vec![0, 3, 0, 1, -3],
            ptr: 0,
        });

        assert_eq!(cpu, expected);
    }

    macro_rules! test_cpu {
        ($a:ident, $b:expr, $c:expr) => {
            #[test]
            fn $a () {
                let cpu = Cpu::try_from($b).unwrap();

                let expected = Cpu::try_from($c).unwrap();

                assert_eq!(Some(expected), cpu.next());
            }
        }
    }

    test_cpu!(test1, "(0) 3  0  1  -3", "(1) 3  0  1  -3");
    test_cpu!(test2, "(1) 3  0  1  -3", "2 (3) 0  1  -3");
    test_cpu!(test3, "2 (3) 0  1  -3", "2  4  0  1 (-3)");
    test_cpu!(test4, "2  4  0  1 (-3)", "2 (4) 0  1  -2");

    #[test]
    fn escape() {
        let cpu = Cpu::try_from("0
3
0
1
-3").unwrap();

        let i = cpu.escape();

        assert_eq!(i, 5);
    }

    #[test]
    fn exit () {
        let cpu = Cpu {
            instructions: vec![2, 4, 0, 1, -2],
            ptr: 1,
        };


        assert_eq!(None, cpu.next());
    }

}



fn main() {
    let input = load_file("inputs/05-1.txt").unwrap();
    let cpu = Cpu::try_from(input.as_ref()).unwrap();

    let i = cpu.escape();

    println!("{}", i);
}
