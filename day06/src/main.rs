use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let fishes = read_file(&args[1]).unwrap();

    println!("Part 1: {}", get_number_of_fishes(&fishes, 80));
    println!("Part 2: {}", get_number_of_fishes(&fishes, 256));
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

fn get_number_of_fishes(starting_fishes: &Vec<u32>, num_days: usize) -> u64 {
    let mut count: [u64; 9] = [0; 9];

    for fish in starting_fishes.clone() {
        count[fish as usize] += 1;
    }
    for _ in 0..num_days {
        let number_of_new_fish = count[0];
        for i in 0..count.len() - 1 {
            count[i] = count[i + 1];
        }
        count[8] = number_of_new_fish;
        count[6] += number_of_new_fish;
    }

    count.iter().sum::<u64>()
}
