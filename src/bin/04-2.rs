mod adventofcode;


use adventofcode::load_file;
use std::collections::HashSet;


#[cfg(test)]
mod tests {
    use validate_passphrase;

    #[test]
    fn test1() {
        let input = "abcde fghij";
        assert_eq!(validate_passphrase(&input), true);
    }
    #[test]
    fn test2() {
        let input = "abcde xyz ecdab";
        assert_eq!(validate_passphrase(&input), false);
    }
    #[test]
    fn test3() {
        let input = "a ab abc abd abf abj";
        assert_eq!(validate_passphrase(&input), true);
    }
    #[test]
    fn test4() {
        let input = "iiii oiii ooii oooi oooo";
        assert_eq!(validate_passphrase(&input), true);
    }
    #[test]
    fn test5() {
        let input = "oiii ioii iioi iiio";
        assert_eq!(validate_passphrase(&input), false);
    }
}


fn validate_passphrase(input: &str) -> bool {
    let mut reduced_words = HashSet::new();

    for w in input.trim().split_whitespace() {
        let mut reduced_word: Vec<_> = w.chars().collect();

        reduced_word.sort();


        if reduced_words.contains(&reduced_word) {
            return false;
        }
        else {
            reduced_words.insert(reduced_word);
        }
    }
    true
}

fn main() {
    let input = load_file("inputs/04-2.txt").unwrap();

    let count = input
        .lines()
        .map(|line| validate_passphrase(line))
        .filter(|x| *x)
        .count();

    println!("{:?}", count);

}
