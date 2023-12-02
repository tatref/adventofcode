use std::error::Error;

use adventofcode2021::load_string;

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn new(data: &Vec<Vec<u32>>) -> Self {
        let data: Vec<Vec<(u32, bool)>> = data
            .iter()
            .filter(|v| v.len() > 0)
            .map(|v| v.iter().map(|x| (*x, false)).collect::<Vec<_>>())
            .collect();

        Self { data }
    }

    fn finished(&self) -> bool {
        for j in 0..self.data.len() {
            if self.data[j].iter().all(|x| x.1) {
                return true;
            }
        }
        for i in 0..self.data[0].len() {
            let mut all_marked = true;
            for j in 0..self.data.len() {
                if !self.data[j][i].1 {
                    all_marked = false;
                }
            }
            if all_marked {
                println!("column!");
                return true;
            }
        }

        false
    }

    fn mark(&mut self, n: u32) -> bool {
        for line in self.data.iter_mut() {
            for (x, marked) in line.iter_mut() {
                if *x == n {
                    *marked = true;
                    return true;
                }
            }
        }
        false
    }

    fn get_non_marked(&self) -> Vec<u32> {
        self.data
            .iter()
            .flatten()
            .filter_map(|(x, marked)| if !marked { Some(*x) } else { None })
            .collect()
    }
}

fn parse_input(s: &str) -> Result<(Vec<u32>, Vec<Board>), Box<dyn Error>> {
    let mut input = s.lines();

    let numbers: Vec<u32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    input.next();

    let mut boards = Vec::new();
    let mut board_input: Vec<Vec<u32>> = Vec::new();
    let mut line = input.next();
    while line.is_some() {
        if line.unwrap() == "" {
            let board = Board::new(&board_input);
            boards.push(board);
            board_input.clear();
        }

        board_input.push(
            line.unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        line = input.next();
    }
    let board = Board::new(&board_input);
    boards.push(board);

    Ok((numbers, boards))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = load_string("inputs/04.txt")?;
    /*
    let mut input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";
     */

    let (numbers, mut boards) = parse_input(&input)?;
    let mut last_number = 0;
    let mut last_board = None;

    'outer_loop: for number in numbers.iter() {
        dbg!(number);

        for board in boards.iter_mut() {
            board.mark(*number);
        }

        if boards.len() == 1 && boards[0].finished() {
            last_board = Some(boards[0].clone());
            last_number = *number;

            break 'outer_loop;
        }

        boards = boards
            .iter()
            .filter(|board| !board.finished())
            .cloned()
            .collect();
    }

    let sum: u32 = last_board.unwrap().get_non_marked().iter().sum();

    dbg!(sum * last_number);

    Ok(())
}
