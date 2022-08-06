use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

const USAGE: &str = " [OPTIONS] FILENAME

Model smoke flow through caves.

The input in FILENAME is a heightmap of the cave floor, consisting of a grid of
height values ranging from 0 through 9.

The output consists of two numbers: an \"aggregate risk level of low points\"
and the product of the sizes of the three largest basins formed by the low
points.

OPTIONS:
 -h: Print this usage message and exit

https://adventofcode.com/2021/day/9
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
    let heightmap = Heightmap::from_file(filename);
    println!("Aggregate risk level of low points: {}", heightmap.get_low_points_aggregate_risk());
    println!("Product of three largest basins: {}", heightmap.get_top_three_basin_sizes_product());
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Coord(usize, usize);

struct Heightmap {
    heights: Vec<Vec<u8>>,
}

impl Heightmap {
    fn from_file(filename: &String) -> Heightmap {
        let file = File::open(filename).expect("Couldn't open file");
        let reader = BufReader::new(file);
        let mut heights: Vec<Vec<u8>> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let row = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            heights.push(row);
        }
        Heightmap {
            heights: heights,
        }
    }
    fn get_low_points(&self) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        for y in 0..self.heights.len() {
            for x in 0..self.heights[y].len() {
                let val = self.heights[y][x];
                let mut all_higher = true;
                if y > 0 && self.heights.get(y - 1).is_some() && self.heights[y - 1][x] <= val {
                    all_higher = false;
                }
                if self.heights.get(y + 1).is_some() && self.heights[y + 1][x] <= val {
                    all_higher = false;
                }
                if x > 0 && self.heights[y].get(x - 1).is_some() && self.heights[y][x - 1] <= val {
                    all_higher = false;
                }
                if self.heights[y].get(x + 1).is_some() && self.heights[y][x + 1] <= val {
                    all_higher = false;
                }
                if all_higher {
                    coords.push(Coord(x, y));
                }
            }
        }
        coords
    }
    fn get_height_at(&self, x: usize, y: usize) -> u8 {
        self.heights[y][x]
    }
    fn get_risk_level(&self, x: usize, y: usize) -> u8 {
        self.get_height_at(x, y) + 1
    }
    fn explore_basin(&self, x: usize, y: usize, basin: &mut HashSet<Coord>) {
        if self.get_height_at(x, y) == 9 || basin.contains(&Coord(x, y)) { return; }
        basin.insert(Coord(x, y));
        if x < self.heights[y].len() - 1 {
            self.explore_basin(x + 1, y, basin);
        }
        if x > 0 {
            self.explore_basin(x - 1, y, basin);
        }
        if y < self.heights.len() - 1 {
            self.explore_basin(x, y + 1, basin);
        }
        if y > 0 {
            self.explore_basin(x, y - 1, basin);
        }
    }
    fn get_basin_size_at(&self, x: usize, y: usize) -> usize {
        let mut basin = HashSet::new();
        self.explore_basin(x, y, &mut basin);
        return basin.len();
    }
    fn get_low_points_aggregate_risk(&self) -> usize {
        self.get_low_points().iter().map(|coord| self.get_risk_level(coord.0, coord.1) as usize).sum()
    }
    fn get_top_three_basin_sizes_product(&self) -> usize {
        let mut sizes = self.get_low_points().iter()
            .map(|coord| self.get_basin_size_at(coord.0, coord.1))
            .collect::<Vec<usize>>();
        sizes.sort_unstable();
        sizes.reverse();
        sizes[0] * sizes[1] * sizes[2]
    }
}