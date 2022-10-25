use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " [OPTIONS] FILENAME

Validate syntax of bracket pairs.

The input in FILENAME contains lines of various bracket characters. A line is
considered corrupt if it has a mismatched bracket pair, while it is incomplete
if there are unmatched brackets.

OPTIONS:
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/10
";

fn usage(argv0: &String) {
    print!("{}{}", argv0, USAGE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    for arg in args.iter().skip(1) {
        if arg == "-h" {
            usage(&args[0]);
            return;
        } else {
            filename = Some(arg);
        }
    }
    if filename.is_none() {
        usage(&args[0]);
    } else {
        let filename = filename.unwrap();
        app(&filename);
    }
}

fn app(filename: &String) {
    let file = File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut total_validation_score = 0;
    let mut completion_scores: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (validation_score, completion_score) = score_line(&line);
        total_validation_score += validation_score;
        if validation_score == 0 {
            // Corrupt lines have nonzero validation score, so ignore them
            completion_scores.push(completion_score);
        }
    }
    completion_scores.sort();
    let middle_score = completion_scores[completion_scores.len() / 2];
    println!("Total syntax error score: {}", total_validation_score);
    println!("Middle completion score: {}", middle_score);
}

fn score_line(line: &String) -> (u64, u64) {
    let mut expected_stack: Vec<char> = Vec::new();
    let mut validation_score: u64 = 0;

    fn validate(expected_stack: &mut Vec<char>, c: char) -> u64 {
        let expected = expected_stack.pop();
        if expected.is_none() {
            println!("Unmatched closer {:?}", c);
            return 0;
        }
        let expected = expected.unwrap();
        if expected == c {
            return 0;
        }
        println!("Expected {:?}, but found {:?} instead", expected, c);
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            unknown => panic!("Unknown character '{:?}'", unknown),
        }
    }

    fn expect(expected_stack: &mut Vec<char>, c: char) -> u64 {
        expected_stack.push(c);
        0
    }

    for c in line.chars() {
        let score = match c {
            '(' => expect(&mut expected_stack, ')'),
            ')' => validate(&mut expected_stack, c),
            '[' => expect(&mut expected_stack, ']'),
            ']' => validate(&mut expected_stack, c),
            '{' => expect(&mut expected_stack, '}'),
            '}' => validate(&mut expected_stack, c),
            '<' => expect(&mut expected_stack, '>'),
            '>' => validate(&mut expected_stack, c),
            unknown => panic!("Unknown character '{:?}'", unknown),
        };
        validation_score += score;
        if score > 0 {
            break;
        }
    }

    let mut completion_score = 0;
    // Handle incomplete lines
    while expected_stack.len() > 0 {
        let c = expected_stack.pop().unwrap();
        completion_score = completion_score * 5 + match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            unknown => panic!("Bad completion stack value '{:?}'", unknown),
        }
    }

    (validation_score, completion_score)
}