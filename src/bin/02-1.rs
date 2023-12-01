mod adventofcode;


use adventofcode::load_file;


#[cfg(test)]
mod tests {
    use spreadsheet_sum;

    #[test]
    fn test1() {
        let input = r#"5 1 9 5
7 5 3
2 4 6 8"#;
        assert_eq!(spreadsheet_sum(&input), 18);
    }
}


fn spreadsheet_sum(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let values: Vec<_> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let min = values.iter().min().unwrap();
        let max = values.iter().max().unwrap();

        let diff = max - min;

        sum += diff;
    }
    sum
}

fn main() {
    let input = load_file("inputs/02-1.txt").unwrap();

    let sum = spreadsheet_sum(&input);

    println!("{}", sum);

}
