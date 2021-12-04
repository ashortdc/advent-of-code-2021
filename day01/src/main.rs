use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_lines(&args[1]).unwrap();

    for i in [1, 3] {
        let mut count = 0;
        let _ = &data[0..data.len() - i + 1]
            .iter()
            .zip(&data[i..data.len()])
            .for_each(|(x, y)| {
                if x < y {
                    count += 1
                }
            });
        println!("(Window = {}) Count: {}", i, count);
    }
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
