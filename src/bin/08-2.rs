#![feature(try_from)]

use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


mod adventofcode;


use adventofcode::load_file;


#[derive(Debug,Eq,PartialEq,Clone)]
struct Cpu {
    registers: HashMap<String, isize>,
}

impl Cpu {
}


impl <'a> TryFrom<&'a str> for Cpu {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut registers = HashMap::new();
        let mut max_value = 0;


        for line in input.lines() {
            let mut iter = line.split_whitespace();

            // parse the line
            let register1: String = iter.next().unwrap().into();
            let inc_or_dec: &str = iter.next().unwrap();
            let mut increment: isize = iter.next().unwrap().parse().unwrap();
            let _if = iter.next();
            let register2: String = iter.next().unwrap().into();
            let comparison: &str = iter.next().unwrap();
            let value2: isize = iter.next().unwrap().parse().unwrap();

            let _register1_val = registers.entry(register1.clone()).or_insert(0).clone();
            let register2_val = registers.entry(register2.clone()).or_insert(0).clone();

            // flip sign?
            match inc_or_dec {
                "inc" => (),
                "dec" => increment = -increment,
                _ => unreachable!(),
            }

            // should we execute the instruction?
            if match comparison {
                "==" => register2_val == value2,
                    "!=" => register2_val != value2,
                    "<"  => register2_val <  value2,
                    "<=" => register2_val <= value2,
                    ">"  => register2_val >  value2,
                    ">=" => register2_val >= value2,
                    x    => panic!("expected comparison operator, got {:?}", x),

            } {
                if let Entry::Occupied(ref mut x) = registers.entry(register1) {
                    *x.get_mut() += increment;
                }
            }


            // find max
            let (_, value) = registers.iter()
                .max_by_key(|&(_register, value)| value)
                .unwrap();

            if *value > max_value {
                max_value = *value;
            }

        }

        println!("max = {}", max_value);


        Ok(
            Self {
                registers,
            }
          )
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::collections::HashMap;

    use Cpu;

    #[test]
    fn parse1() {
        let cpu = "b inc 5 if a > 1".try_into();

        let mut registers = HashMap::new();
        registers.insert("a".into(), 0);
        registers.insert("b".into(), 0);

        let expected = Ok(
            Cpu {
                registers
            }
            );

        assert_eq!(cpu, expected);
    }

    #[test]
    fn parse2() {
        let cpu = r#"b inc 5 if a > 1
a inc 1 if b < 5"#.try_into();

        let mut registers = HashMap::new();
        registers.insert("a".into(), 1);
        registers.insert("b".into(), 0);

        let expected = Ok(
            Cpu {
                registers
            }
            );

        assert_eq!(cpu, expected);
    }

    #[test]
    fn parse3() {
        let cpu = r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#.try_into();

        let mut registers = HashMap::new();
        registers.insert("a".into(), 1);
        registers.insert("b".into(), 0);
        registers.insert("c".into(), -10);

        let expected = Ok(
            Cpu {
                registers
            }
            );

        assert_eq!(cpu, expected);
    }
}



fn main() {
    let input = load_file("inputs/08-1.txt").unwrap();
    let _cpu = Cpu::try_from(input.as_ref()).unwrap();

}
