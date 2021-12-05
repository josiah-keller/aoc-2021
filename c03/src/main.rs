use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " FILENAME

Calculate diagnostic values.

The file given by FILENAME contains a series of numeric values written in
binary separated by newlines.

The \"gamma\" and \"epsilon\" values are found by determining the most common
value for each bit position in the list of values. The product of these is the
\"power consumption\".

The \"life support\" value is the product of the \"O2\" and \"CO2\" values,
which are determined by incrementally removing values whose bit does not match
the most common corresponding bit in the list, starting with the most-
significant bit.

https://adventofcode.com/2021/day/3
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
        diagnostics(filename);
    }
}

fn load_data(filename: &String) -> (Vec<u32>, usize) {
    let mut data = Vec::new();
    let file = File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut size = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        size = line.len();
        let value = u32::from_str_radix(&line, 2).unwrap();
        data.push(value);
    }
    (data, size)
}

fn diagnostics(filename: &String) {
    let (data, size) = load_data(filename);

    let (gamma, epsilon) = gamma_epsilon(&data, size);
    
    let power = gamma * epsilon;
    println!("Power consumption: {}", power);

    let o2 = calc_life_support_value(&data, size, true);
    let co2 = calc_life_support_value(&data, size, false);
    println!("Life support: {}", o2 * co2);
}

fn gamma_epsilon(data: &Vec<u32>, size: usize) -> (u32, u32) {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for bit in 0..size {
        let mut z_ct = 0;
        let mut o_ct = 0;
        let mask = 1 << bit;
        for val in data {
            if val & mask != 0 {
                o_ct += 1;
            } else {
                z_ct += 1;
            }
        }
        if z_ct > o_ct {
            gamma |= 0 << bit;
            epsilon |= 1 << bit;
        } else {
            gamma |= 1 << bit;
            epsilon |= 0 << bit;
        }
    }
    (gamma, epsilon)
}

fn calc_life_support_value(data: &Vec<u32>, size: usize, use_most_common: bool) -> u32 {
    let mut filtered_data = data.clone();
    for bit in (0..size).rev() {
        let mask = 1 << bit;
        let mut idx = 0;
        let (gamma, epsilon) = gamma_epsilon(&filtered_data, size);
        let bit_criteria = if use_most_common { gamma } else { epsilon };
        while idx < filtered_data.len() {
            if filtered_data[idx] & mask != bit_criteria & mask {
                filtered_data.remove(idx);
            } else {
                idx += 1;
            }
        }
        if filtered_data.len() == 1 {
            break;
        }
    }
    filtered_data[0]
}