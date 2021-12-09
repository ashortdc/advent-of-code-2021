use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let lines = read_file(&args[1]).unwrap();

    let mut counter = 0;
    for line in &lines {
        let split_output: Vec<String> = line[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for digit in split_output {
            if [2, 3, 4, 7].contains(&digit.len()) {
                counter += 1;
            }
        }
    }

    println!("Counter: {}", counter);
}

fn read_file<P>(filename: P) -> Result<Vec<Vec<String>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let br = BufReader::new(file);

    let mut return_vector: Vec<Vec<String>> = Vec::new();

    let lines: Vec<String> = br.lines().map(|line| line.unwrap()).collect();

    for line in &lines {
        let split_line: Vec<String> = line.split(" | ").map(|s| s.parse().unwrap()).collect();
        return_vector.push(split_line);
    }

    Ok(return_vector)
}
