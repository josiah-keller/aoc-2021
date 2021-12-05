use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const USAGE: &str = " FILENAME

Follow a submarine course.

The file given by FILENAME contains a series of directional instructions on
separate lines. A line can direct the submarine to go forward, down, or up.
The submarine moves in two dimensions - horizontal (forward) and depth (down
and up).

This program follows the course given in the file and calculates the product of
the final horizontal and depth positions.

https://adventofcode.com/2021/day/2
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
        follow_course(filename);
    }
}

struct Course {
    steps: Vec<CourseStep>,
    horiz: i32,
    depth: i32,
    aim: i32,
}
enum CourseStep {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Course {
    fn from_file(filename: &String) -> Course {
        let mut steps: Vec<CourseStep> = Vec::new();
        let file = File::open(filename).expect("Couldn't open file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" ").collect();
            let direction = parts[0];
            let magnitude = parts[1].parse().expect("Non-numeric magnitude");
            if direction == "forward" {
                steps.push(CourseStep::Forward(magnitude));
            } else if direction == "down" {
                steps.push(CourseStep::Down(magnitude));
            } else if direction == "up" {
                steps.push(CourseStep::Up(magnitude));
            } else {
                panic!("Unknown direction {}", direction);
            }
        }
        Course {
            steps: steps,
            horiz: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn follow_simple(&mut self) {
        for step in &self.steps {
            match step {
                CourseStep::Forward(magnitude) => { self.horiz += magnitude },
                CourseStep::Down(magnitude) => { self.depth += magnitude },
                CourseStep::Up(magnitude) => { self.depth -= magnitude },
            }
        }
    }
    fn follow_with_aim(&mut self) {
        for step in &self.steps {
            match step {
                CourseStep::Forward(magnitude) => {
                    self.horiz += magnitude;
                    self.depth += magnitude * self.aim;
                },
                CourseStep::Down(magnitude) => { self.aim += magnitude },
                CourseStep::Up(magnitude) => { self.aim -= magnitude },
            }
        }
    }
    fn reset(&mut self) {
        self.horiz = 0;
        self.depth = 0;
        self.aim = 0;
    }
}

fn follow_course(filename: &String) {
    let mut course = Course::from_file(filename);
    course.follow_simple();
    println!("Simple: Horiz * Depth: {}", course.horiz * course.depth);
    course.reset();
    course.follow_with_aim();
    println!("With aim: Horiz * Depth: {}", course.horiz * course.depth);
}