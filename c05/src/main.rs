use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp::{min, max};

const USAGE: &str = " [OPTIONS] FILENAME

Find intersections.

Each line of FILENAME represents X/Y endpoints of lines. This program will find
all discrete points in a grid where two or more lines intersect. By default,
horizontal and vertical lines are considered. With -d, 45-degree diagonals are
also considered.

OPTIONS:
 -d: Include 45-degree diagonal lines
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/5
";

fn usage(argv0: &String) {
    print!("{}{}", argv0, USAGE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    let mut allow_diag = false;
    for arg in args.iter().skip(1) {
        if arg == "-h" {
            usage(&args[0]);
            return;
        }
        if arg == "-d" {
            allow_diag = true;
        } else {
            filename = Some(arg);
        }
    }
    if filename.is_none() {
        usage(&args[0]);
    } else {
        let filename = filename.unwrap();
        lines(&filename, allow_diag);
    }
}

fn lines(filename: &String, allow_diag: bool) {
    let mut lines = Vec::new();

    let file = File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(file);

    for file_line in reader.lines() {
        let file_line = file_line.unwrap();
        let sp: Vec<&str> = file_line.split(" -> ").collect();
        let st_sp: Vec<&str> = sp[0].split(",").collect();
        let ed_sp: Vec<&str> = sp[1].split(",").collect();
        let x1 = st_sp[0].parse().unwrap();
        let x2 = ed_sp[0].parse().unwrap();
        let y1 = st_sp[1].parse().unwrap();
        let y2 = ed_sp[1].parse().unwrap();
        let line = Line(x1, y1, x2, y2);
        lines.push(line);
    }

    println!("# intersections: {}", count_intersections(&lines, allow_diag));
}

fn count_intersections(lines: &Vec<Line>, allow_diag: bool) -> usize {
    let mut intersections = HashSet::new();
    // yeah, it's O(n^2), but n is only 500
    for i in 0..lines.len() {
        if !&lines[i].is_cardinal() && (!allow_diag || !&lines[i].is_diagonal()) { continue; }
        for j in 0..lines.len() {
            if i == j { continue; }
            if !&lines[j].is_cardinal() && (!allow_diag || !&lines[j].is_diagonal()) { continue; }
            if !overlap(&lines[i], &lines[j]) { continue; }

            for (jx, jy) in lines[j].points() {
                if lines[i].contains_point(jx, jy) {
                    intersections.insert((jx, jy));
                }
            }
        }
    }
    return intersections.len();
}

fn overlap(line1: &Line, line2: &Line) -> bool {
    (line1.min_x() <= line2.max_x() && line1.max_x() >= line2.min_x()) && (line1.min_y() <= line2.max_y() && line1.max_y() >= line2.min_y())
}

#[derive(Debug)]
struct Line(i32, i32, i32, i32);

impl Line {
    fn min_x(&self) -> i32 { min(self.0, self.2) }
    fn max_x(&self) -> i32 { max(self.0, self.2) }
    fn min_y(&self) -> i32 { min(self.1, self.3) }
    fn max_y(&self) -> i32 { max(self.1, self.3) }

    fn is_horizontal(&self) -> bool { self.1 == self.3 }
    fn is_vertical(&self) -> bool { self.0 == self.2 }
    fn is_diagonal(&self) -> bool { self.max_x() - self.min_x() == self.max_y() - self.min_y() }
    fn is_cardinal(&self) -> bool { self.is_horizontal() || self.is_vertical() }

    fn points(&self) -> Vec<(i32, i32)> {
        let mut x = self.0;
        let mut y = self.1;
        let mut points = Vec::new();
        let x_dir = if self.0 < self.2 { 1 } else { -1 };
        let y_dir = if self.1 < self.3 { 1 } else { -1 };

        loop {
            points.push((x, y));
            if self.is_horizontal() {
                x += x_dir;
                if x == self.2 { break }
            } else if self.is_vertical() {
                y += y_dir;
                if y == self.3 { break }
            } else if self.is_diagonal() {
                x += x_dir;
                y += y_dir;
                if x == self.2 && y == self.3 { break }
            }
        }

        points
    }

    fn contains_point(&self, x: i32, y: i32) -> bool {
        if self.is_horizontal() {
            y == self.1 && x >= self.min_x() && x <= self.max_x()
        } else if self.is_vertical() {
            x == self.0 && y >= self.min_y() && y <= self.max_y()
        } else if self.is_diagonal() {
            x >= self.min_x() && x <= self.max_x() && y >= self.min_y() && y <= self.max_y()
                && (x - self.0).abs() == (y - self.1).abs()
        } else { false }
    }
}