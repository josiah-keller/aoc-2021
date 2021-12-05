use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " FILENAME

Analyze sonar measurements.

The file given by FILENAME contains a series of depth measurements separated by
newlines. This program will count how many measurements are deeper than their
predecessor, both individually and in a window of three measurements.

https://adventofcode.com/2021/day/1
";

fn usage(argv0: &String) {
    print!("{}{}", argv0, USAGE);
}

fn single_measurement(depths: &Vec<i32>) {
    let mut is_first = true;
    let mut prev_depth = 0;
    let mut count = 0;
    for depth in depths {
        if !is_first && *depth > prev_depth {
            count += 1;
        }
        prev_depth = *depth;
        is_first = false;
    }
    println!("Single-measurement count: {}", count);
}

fn window_measurement(depths: &Vec<i32>) {
    let mut is_first = true;
    let mut prev_sum = 0;
    let mut count = 0;
    for i in 0..depths.len() - 2 {
        let sum = depths[i] + depths[i + 1] + depths[i + 2];
        if !is_first && sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
        is_first = false;
    }
    println!("Windowed-measurement count: {}", count)
}

fn load_depths(filename: &String) -> Vec<i32> {
    let mut depths = Vec::new();
    let file = File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let depth = line.unwrap().parse().expect("Non-numeric line");
        depths.push(depth);
    }
    depths
}

fn sonar(filename: &String) {
    let depths = load_depths(filename);
    single_measurement(&depths);
    window_measurement(&depths);
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
        sonar(filename);
    }
}
