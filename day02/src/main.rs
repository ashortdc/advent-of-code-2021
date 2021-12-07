use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_lines(&args[1]).unwrap();

    let mut positions: HashMap<String, i32> = HashMap::new();

    let _ = &data
        .clone()
        .into_iter()
        .for_each(|(key, value)| *positions.entry(key).or_insert(0) += value);

    println!(
        "(Part 1) Final Position: {}",
        positions["forward"] * (positions["down"] - positions["up"])
    );

    let mut horizontal_position = 0;
    let mut aim = 0;
    let mut depth = 0;
    let _ = &data
        .clone()
        .into_iter()
        .for_each(|(key, value)| match key.as_str() {
            "forward" => {
                horizontal_position += value;
                depth += aim * value
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => unreachable!("Invalid key"),
        });

    println!("(Part 2) Final Position: {}", horizontal_position * depth);
}

fn read_lines<P>(filename: P) -> Result<Vec<(String, i32)>, Error>
where
    P: AsRef<Path>,
{
    let br = BufReader::new(File::open(filename)?);
    br.lines()
        .map(|line| {
            line.and_then(|v| {
                let split_line: Vec<String> = v
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                Ok((
                    split_line[0].to_string(),
                    split_line[1].parse::<i32>().unwrap(),
                ))
            })
        })
        .collect()
}
