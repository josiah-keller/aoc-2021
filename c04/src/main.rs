use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " FILENAME

Determine the outcome of a bingo game.

The given file begins with a line of comma-separated numbers to be called. The
rest of the file is a series of bingo boards.

This program will determine which board wins first and what its score is.

https://adventofcode.com/2021/day/4
";

fn usage(argv0: &String) {
    print!("{}{}", argv0, USAGE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    if filename.is_none() {
        usage(&args[0]);
    } else {
        let filename = filename.unwrap();
        if filename == "-h" {
            usage(&args[0]);
            return;
        }
        bingo(filename);
    }
}

fn bingo(filename: &String) {
    let mut game = BingoGame::from_file(&filename);
    let winning_board = loop {
        game.advance();
        if let Some(winner) = game.find_winner() {
            break winner;
        }
        if game.is_over() {
            println!("Reached end of game without a winner!");
            return;
        }
    };
    println!("Final score of first winning board: {}", winning_board.score);
    let final_winner = loop {
        game.advance();
        if game.is_over() {
            // can unwrap b/c there has to have been a winner by this point or the game.is_over() check above would have caught it
            break &game.boards[game.latest_winner.unwrap()];
        }
    };
    println!("Final score of final winning board: {}", final_winner.score);
}

struct BingoBoard {
    values: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
    score: i32,
    has_won: bool,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            values: [[0; 5]; 5],
            marks: [[false; 5]; 5],
            score: 0,
            has_won: false,
        }
    }
    fn check_win(&self) -> bool {
        for row_idx in 0..5 {
            let mut col_ct = 0;
            let mut row_ct = 0;
            for col_idx in 0..5 {
                if self.marks[row_idx][col_idx] {
                    col_ct += 1;
                }
                if self.marks[col_idx][row_idx] {
                    row_ct += 1;
                }
            }
            if row_ct == 5 || col_ct == 5 {
                return true;
            }
        }
        false
    }
    fn call(&mut self, number: u8) {
        for row_idx in 0..5 {
            for col_idx in 0..5 {
                if self.values[row_idx][col_idx] == number {
                    self.marks[row_idx][col_idx] = true;
                }
            }
        }
        if self.has_won {
            // Don't recalculate score if we already won
            return;
        }
        self.has_won = self.check_win();
        self.score = self.local_score() * i32::from(number);
    }
    fn local_score(&self) -> i32 {
        let mut sum: i32 = 0;
        for row_idx in 0..5 {
            for col_idx in 0..5 {
                if !self.marks[row_idx][col_idx] {
                    sum += i32::from(self.values[row_idx][col_idx]);
                }
            }
        }
        sum
    }
}

struct BingoGame {
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
    number_idx: usize,
    latest_winner: Option<usize>,
}

impl BingoGame {
    fn from_file(filename: &String) -> BingoGame {
        let mut numbers = Vec::new();
        let mut boards = Vec::new();

        let file = File::open(filename).expect("Couldn't open file");
        let reader = BufReader::new(file);

        let mut cur_board = BingoBoard::new();
        let mut row_idx = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            if numbers.len() == 0 {
                numbers.append(&mut line.split(",").map(|v| v.parse().unwrap()).collect::<Vec<u8>>());
                continue;
            }
            if line == "" {
                continue;
            }

            let row = line.split_whitespace().map(|v| v.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            assert_eq!(row.len(), 5, "Not 5 columns in this row: {:?}", row);
            for i in 0..5 {
                cur_board.values[row_idx][i] = row[i];
            }
            row_idx += 1;
            if row_idx == 5 {
                row_idx = 0;
                boards.push(cur_board);
                cur_board = BingoBoard::new();
            }
        }

        BingoGame {
            numbers,
            boards,
            number_idx: 0,
            latest_winner: None,
        }
    }
    fn find_winner(&self) -> Option<&BingoBoard> {
        for board in &self.boards {
            if board.has_won {
                return Some(board);
            }
        }
        None
    }
    fn is_over(&self) -> bool {
        self.number_idx >= self.numbers.len()
    }
    fn advance(&mut self) {
        if self.is_over() {
            return;
        }
        let number = self.numbers[self.number_idx];
        self.number_idx += 1;
        for (idx, board) in self.boards.iter_mut().enumerate() {
            let had_won = board.has_won;
            board.call(number);
            if !had_won && board.has_won {
                self.latest_winner = Some(idx);
            }
        }
    }
}