use std::env;
use std::fs;

const USAGE: &str = " [OPTIONS] FILENAME

Determine the optimal crab alignment.

The input in FILENAME is a comma-separated list of horizontal position values.
Determine the horizontal position that all crabs could travel to using the
least fuel.

OPTIONS:
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/7
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
    let data = fs::read_to_string(filename).unwrap();
    let positions: Vec<i64> = data.trim().split(",").map(|v| v.parse().unwrap()).collect();
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut simple_optimal_pos = min_pos;
    let mut simple_cur_cost = i64::MAX;
    let mut dyanmic_optimal_pos = min_pos;
    let mut dynamic_cur_cost = i64::MAX;

    // simple brute-force approach... this feels like the kind of problem where
    // it's meant to fit some well-known algorithm, but idk what it is and this
    // doesn't really take that long to run.
    for pos in min_pos..=max_pos {
        let simple_cost = compute_cost_simple(&positions, pos);
        if simple_cost < simple_cur_cost {
            simple_cur_cost = simple_cost;
            simple_optimal_pos = pos;
        }
        let dynamic_cost = compute_cost_dynamic(&positions, pos);
        if dynamic_cost < dynamic_cur_cost {
            dynamic_cur_cost = dynamic_cost;
            dyanmic_optimal_pos = pos;
        }
    }
    println!("SIMPLE MODE: Optimal position was {} with fuel usage of {}", simple_optimal_pos, simple_cur_cost);
    println!("DYNAMIC MODE: Optimal position was {} with fuel usage of {}", dyanmic_optimal_pos, dynamic_cur_cost);
}

fn compute_cost_simple(positions: &Vec<i64>, origin: i64) -> i64 {
    let mut cost = 0;
    for pos in positions {
        cost += (pos - origin).abs();
    }
    cost
}

fn compute_cost_dynamic(positions: &Vec<i64>, origin: i64) -> i64 {
    let mut cost = 0;
    for pos in positions {
        cost += distance_to_fuel((pos - origin).abs());
    }
    cost
}

fn distance_to_fuel(distance: i64) -> i64 {
    if distance == 0 { 0 } else { distance + distance_to_fuel(distance - 1) }
}