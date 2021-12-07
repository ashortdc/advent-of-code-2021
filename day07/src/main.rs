use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let crabs = read_file(&args[1]).unwrap();    

    let mut min = std::u32::MAX;
    let max_vector_value = crabs.iter().max().unwrap();

    for i in 0..*max_vector_value as usize {
        let mut sum: u32 = 0;
        for j in 0..crabs.len() {
            let diff = crabs[j] as i32 - i as i32;
            sum += diff.abs() as u32;
        }
        if sum < min {
            min = sum;
        }
    }

    println!("Part 1: {}", min);

    min = std::u32::MAX;

    for i in 0..*max_vector_value as usize {
        let mut sum: u32 = 0;
        for j in 0..crabs.len() {
            let diff = crabs[j] as i32 - i as i32;
            sum += get_summation(diff.abs() as u32);
        }
        if sum < min {
            min = sum;
        }
    }
    
    println!("Part 2: {}", min);
}

fn read_file<P>(filename: P) -> Result<Vec<u32>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut br = BufReader::new(file);
    let mut line = String::new();

    let _ = br.read_line(&mut line)?;

    Ok(line
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect())
}

fn get_summation(n: u32) -> u32 {
    (n * (n + 1)) / 2
}