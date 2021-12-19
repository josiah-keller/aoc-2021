use std::env;
use std::fs;

const USAGE: &str = " [OPTIONS] FILENAME DAYS

Model population of lanternfish, starting from the initial state in the given
FILENAME, after the given number of DAYS.

OPTIONS:
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/6
";

fn usage(argv0: &String) {
    print!("{}{}", argv0, USAGE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    let mut days = None;
    for arg in args.iter().skip(1) {
        if arg == "-h" {
            usage(&args[0]);
            return;
        } else if filename.is_none() {
            filename = Some(arg);
        } else if days.is_none() {
            days = Some(arg.parse().expect("DAYS must be numeric"));
        }
    }
    if filename.is_none() || days.is_none() {
        usage(&args[0]);
    } else {
        let filename = filename.unwrap();
        let days = days.unwrap();
        app(&filename, days);
    }
}

fn app(filename: &String, days: u64) {
    let data = fs::read_to_string(filename).unwrap();
    let initial_state: Vec<usize> = data.trim().split(",").map(|v| v.parse().unwrap()).collect();
    let mut counts = [0u64; 9];
    for &value in &initial_state {
        counts[value] += 1;
    }

    for _day in 0..days {
        advance(&mut counts);
    }

    let total: u64 = counts.iter().sum();
    println!("Total lanternfish: {}", total);
}

fn advance(counts: &mut [u64; 9]) {
    let repros = counts[0];
    for i in 1..9 {
        counts[i - 1] = counts[i];
    }
    counts[6] += repros;
    counts[8] = repros;
}