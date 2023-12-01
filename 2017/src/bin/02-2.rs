mod adventofcode;


use adventofcode::load_file;


#[cfg(test)]
mod tests {
    use spreadsheet_sum;

    #[test]
    fn test1() {
        let input = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;
        assert_eq!(spreadsheet_sum(&input), 9);
    }
}


fn spreadsheet_sum(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let values: Vec<_> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        for a in values.iter() {
            for b in values.iter() {
                if b != a && a % b == 0 {
                    sum += a / b;
                }
            }
        }
    }
    sum
}

fn main() {
    let input = load_file("inputs/02-1.txt").unwrap();

    let sum = spreadsheet_sum(&input);

    println!("{}", sum);

}
