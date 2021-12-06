use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(line: &String) -> Self {
        let points: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();

        Point {
            x: points[0],
            y: points[1],
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(line: &String) -> Self {
        let points: Vec<String> = line
            .trim()
            .split(" -> ")
            .map(|s| s.parse().unwrap())
            .collect();

        Line {
            a: Point::new(&points[0]),
            b: Point::new(&points[1]),
        }
    }

    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let lines = read_file(&args[1]).unwrap();
    let graph = graph_lines(&lines, true);
    println!("Part 1: {}", graph.iter().filter(|(&_p, &v)| v > 1).count());

    let part_2_graph = graph_lines(&lines, false);

    println!(
        "Part 2: {}",
        part_2_graph.iter().filter(|(&_p, &v)| v > 1).count()
    );
}

fn read_file<P>(filename: P) -> Result<Vec<Line>, Error>
where
    P: AsRef<Path>,
{
    let mut lines: Vec<Line> = Vec::new();

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Continuously read lines from the files
    while reader.read_line(&mut line)? != 0 {
        if !line.trim().is_empty() {
            lines.push(Line::new(&line));
        }
        // Clear the buffer
        line.clear();
    }

    Ok(lines)
}

fn graph_lines(lines: &Vec<Line>, straight_only: bool) -> HashMap<Point, u32> {
    let mut graph: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        // Handle horizontal lines
        if line.is_horizontal() {
            if line.a.x < line.b.x {
                for i in line.a.x..=line.b.x {
                    *graph.entry(Point { x: i, y: line.a.y }).or_insert(0) += 1;
                }
            } else {
                for i in line.b.x..=line.a.x {
                    *graph.entry(Point { x: i, y: line.b.y }).or_insert(0) += 1;
                }
            }
        // Handle vertical lines
        } else if line.is_vertical() {
            if line.a.y < line.b.y {
                for i in line.a.y..=line.b.y {
                    *graph.entry(Point { x: line.a.x, y: i }).or_insert(0) += 1;
                }
            } else {
                for i in line.b.y..=line.a.y {
                    *graph.entry(Point { x: line.b.x, y: i }).or_insert(0) += 1;
                }
            }
        // Handle everything else
        } else {
            if straight_only {
                continue;
            }
            if line.a.y < line.b.y {
                if line.a.x < line.b.x {
                    for i in 0..=line.b.x - line.a.x {
                        let x = line.a.x + i;
                        let y = line.a.y + i;
                        *graph.entry(Point { x: x, y: y }).or_insert(0) += 1;
                    }
                } else {
                    for i in 0..=line.a.x - line.b.x {
                        let x = line.a.x - i;
                        let y = line.a.y + i;
                        *graph.entry(Point { x: x, y: y }).or_insert(0) += 1;
                    }
                }
            } else {
                if line.a.x < line.b.x {
                    for i in 0..=line.b.x - line.a.x {
                        let x = line.b.x - i;
                        let y = line.b.y + i;
                        *graph.entry(Point { x: x, y: y }).or_insert(0) += 1;
                    }
                } else {
                    for i in 0..=line.a.x - line.b.x {
                        let x = line.b.x + i;
                        let y = line.b.y + i;
                        *graph.entry(Point { x: x, y: y }).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    graph
}
