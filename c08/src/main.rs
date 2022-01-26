use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " [OPTIONS] FILENAME

Unscramble seven-segment digits.

Each line in FILENAME corresponds to a 4-digit 7-segment display. For each
display, there are ten signal patterns. Each represents an individual digit.
The order of the characters in the pattern is not significant. The latter part
of each line is four patterns that represent the final value of that display.

Determine the final value for each display, based on the fact that some digits
have a unique number of segments.

OPTIONS:
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/8
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

    let mut simple_digits_count = 0;
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let display = Display::from_string(&line);
        for i in 0..4 {
            simple_digits_count += match display.readout[i] {
                1 => 1,
                4 => 1,
                7 => 1,
                8 => 1,
                _ => 0,
            };
        }
        sum += display.value();
    }
    println!("Found {} ones, fours, sevens, and eights", simple_digits_count);
    println!("Sum: {}", sum);
}

fn get_sorted_pattern(pattern: &str) -> String {
    let mut sorted = pattern.chars().collect::<Vec<char>>();
    sorted.sort_unstable();
    return String::from_iter(sorted);
}

fn nth_char(s: &str, n: usize) -> char {
    return s.chars().nth(n).unwrap();
}

fn is_signal_subset(s: &str, p: &str) -> bool {
    for c in p.chars() {
        if !s.contains(c) { return false; }
    }
    true
}

struct Display {
    one: String,
    four: String,
    seven: String,
    eight: String,
    three: String,
    six: String,
    nine: String,
    zero: String,
    two: String,
    five: String,

    /*
    Segment map is laid out as follows:
     0000
    1    2
    1    2
     3333
    4    5
    4    5
     6666
    */
    segment_map: [char; 7],
    readout: [i32; 4],
}

const SEGMENTS_IN_ONE: usize = 2;
const SEGMENTS_IN_FOUR: usize = 4;
const SEGMENTS_IN_SEVEN: usize = 3;
const SEGMENTS_IN_EIGHT: usize = 7;

impl Display {
    fn value(&self) -> i32 {
        return self.readout[0] * 1000
              + self.readout[1] * 100
              + self.readout[2] * 10
              + self.readout[3] * 1;
    }
    // all this seems a little clunky but ¯\_(ツ)_/¯
    fn from_string(s: &String) -> Display {
        let sp: Vec<&str> = s.split("|").collect();
        let patterns: Vec<&str> = sp[0].trim().split(" ").collect();
        let digits: Vec<&str> = sp[1].trim().split(" ").collect();

        let mut display = Display {
            one: "".to_string(),
            four: "".to_string(),
            seven: "".to_string(),
            eight: "".to_string(),
            three: "".to_string(),
            six: "".to_string(),
            nine: "".to_string(),
            zero: "".to_string(),
            two: "".to_string(),
            five: "".to_string(),
            segment_map: ['\0', '\0', '\0', '\0', '\0', '\0', '\0'],
            readout: [0xff, 0xff, 0xff, 0xff],
        };

        // first pass - get unambiguous digits
        for pattern in &patterns {
            let sorted = get_sorted_pattern(pattern);
            match sorted.len() {
                SEGMENTS_IN_ONE => {
                    display.one = sorted;
                },
                SEGMENTS_IN_FOUR => {
                    display.four = sorted;
                },
                SEGMENTS_IN_SEVEN => {
                    display.seven = sorted;
                },
                SEGMENTS_IN_EIGHT => {
                    display.eight = sorted;
                },
                _ => (),
            };
        }

        // we know segment 0 because it is the difference between a one and a seven
        display.segment_map[0] = nth_char(
            &display.seven.replace(nth_char(&display.one, 0), "")
                .replace(nth_char(&display.one, 1), ""),
            0);

        // second pass - find three
        for pattern in &patterns {
            let sorted = get_sorted_pattern(pattern);
            if sorted.len() == 5 && is_signal_subset(&sorted, &display.seven) {
                // three is a 5-segment pattern that contains the 3 segments from seven
                display.three = sorted;
                break;
            }
        }

        // segments 1 and 3 are the difference between four and one
        // now we can disambiguate them because we've identified three
        let four_ambigs = display.four
            .replace(nth_char(&display.one, 0), "")
            .replace(nth_char(&display.one, 1), "");

        let four_ambig_0 = nth_char(&four_ambigs, 0);
        let four_ambig_1 = nth_char(&four_ambigs, 1);
        if display.three.contains(four_ambig_0) {
            display.segment_map[3] = four_ambig_0;
            display.segment_map[1] = four_ambig_1;
        } else {
            display.segment_map[1] = four_ambig_0;
            display.segment_map[3] = four_ambig_1;
        }

        // third pass - find six
        for pattern in &patterns {
            let sorted = get_sorted_pattern(pattern);
            if sorted.len() == 6 {
                if sorted.replace(nth_char(&display.one, 0), "").replace(nth_char(&display.one, 1), "").len() == 5 {
                    // six is a 6-segment pattern that shares 1 segment with one
                    display.six = sorted;
                    break;
                }
            }
        }

        // fourth pass - find nine
        for pattern in &patterns {
            let sorted = get_sorted_pattern(pattern);
            if sorted.len() == 6 && sorted != display.six {
                if sorted.contains(display.segment_map[3]) {
                    // nine is the only other 6-segment pattern that contains segment 3 (the middle)
                    display.nine = sorted;
                    break;
                }
            }
        }

        // fifth pass - find zero
        for pattern in &patterns {
            let sorted = get_sorted_pattern(pattern);
            if sorted.len() == 6 && sorted != display.six && sorted != display.nine {
                // zero is the only other 6-segment pattern
                display.zero = sorted;
            }
        }

        if display.six.contains(nth_char(&display.one, 0)) {
            display.segment_map[5] = nth_char(&display.one, 0);
            display.segment_map[2] = nth_char(&display.one, 1);
        } else {
            display.segment_map[2] = nth_char(&display.one, 0);
            display.segment_map[5] = nth_char(&display.one, 1);
        }

        display.segment_map[6] = nth_char(&display.three
            .replace(display.segment_map[0], "")
            .replace(display.segment_map[2], "")
            .replace(display.segment_map[3], "")
            .replace(display.segment_map[5], ""), 0);

        display.segment_map[4] = nth_char(&display.eight
            .replace(display.segment_map[0], "")
            .replace(display.segment_map[1], "")
            .replace(display.segment_map[2], "")
            .replace(display.segment_map[3], "")
            .replace(display.segment_map[5], "")
            .replace(display.segment_map[6], ""), 0);

        display.two = get_sorted_pattern(&String::from_iter([
            display.segment_map[0],
            display.segment_map[2],
            display.segment_map[3],
            display.segment_map[4],
            display.segment_map[6]
        ]));

        display.five = get_sorted_pattern(&String::from_iter([
            display.segment_map[0],
            display.segment_map[1],
            display.segment_map[3],
            display.segment_map[5],
            display.segment_map[6]
        ]));

        for (i, &digit_pattern) in digits.iter().enumerate() {
            let mut sorted = digit_pattern.chars().collect::<Vec<char>>();
            sorted.sort_unstable();
            let sorted = String::from_iter(sorted);
            display.readout[i] = if sorted == display.one {
                1
            } else if sorted == display.four {
                4
            } else if sorted == display.seven {
                7
            } else if sorted == display.eight {
                8
            } else if sorted == display.three {
                3
            } else if sorted == display.six {
                6
            } else if sorted == display.nine{
                9
            } else if sorted == display.zero {
                0
            } else if sorted == display.two {
                2
            } else if sorted == display.five {
                5
            } else {
                println!("unknown pattern {}", sorted);
                0xff
            };
        }

        display
    }
}