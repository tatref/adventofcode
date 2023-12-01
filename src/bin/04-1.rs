mod adventofcode;


use adventofcode::load_file;
use std::collections::HashSet;


#[cfg(test)]
mod tests {
    use validate_passphrase;

    #[test]
    fn test1() {
        let input = "aa bb cc dd ee";
        assert_eq!(validate_passphrase(&input), true);
    }
    #[test]
    fn test2() {
        let input = "aa bb cc dd aa";
        assert_eq!(validate_passphrase(&input), false);
    }
    #[test]
    fn test3() {
        let input = "aa bb cc dd aaa";
        assert_eq!(validate_passphrase(&input), true);
    }
}


fn validate_passphrase(input: &str) -> bool {
    let mut words = HashSet::new();
    for w in input.trim().split_whitespace() {
        if words.contains(w) {
            return false;
        }
        else {
            words.insert(w);
        }
    }
    true
}

fn main() {
    let input = load_file("inputs/04-1.txt").unwrap();

    let count = input
        .lines()
        .map(|line| validate_passphrase(line))
        .filter(|x| *x)
        .count();

    println!("{:?}", count);

}
