use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_lines(&args[1]).unwrap();

    println!(
        "(Part 1) Measurements larger than the previous measurements: {}",
        number_of_increases(&data, 1)
    );

    println!(
        "(Part 2) Measurements larger than the previous measurements: {}",
        number_of_increases(&data, 3)
    );
}

fn read_lines<P>(filename: P) -> Result<Vec<i32>, Error>
where
    P: AsRef<Path>,
{
    let br = BufReader::new(File::open(filename)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn number_of_increases(array: &Vec<i32>, window: usize) -> i32 {
    let mut ret_value = 0;
    for i in window..array.len() {
        ret_value += if array[i] > array[i - window] {1} else {0};
    }
    ret_value
}
