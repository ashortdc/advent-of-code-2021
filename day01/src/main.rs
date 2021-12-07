use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_file(&args[1]).unwrap();

    for i in [1, 3] {
        let _ = println!(
            "(Window = {}) Count: {}",
            i,
            &data[0..data.len() - i + 1]
                .iter()
                .zip(&data[i..data.len()])
                .filter(|(&x, &y)| x < y)
                .count()
        );
    }
}

fn read_file<P>(filename: P) -> Result<Vec<i32>, Error>
where
    P: AsRef<Path>,
{
    let br = BufReader::new(File::open(filename)?);
    br.lines()
        .map(|line| line.and_then(|v| Ok(v.parse().unwrap())))
        .collect()
}
