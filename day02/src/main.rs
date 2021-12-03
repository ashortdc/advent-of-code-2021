use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_lines(&args[1]).unwrap();

    println!(
        "(Part 1) Final Position: {}",
        get_final_position(&data, false)
    );

    println!(
        "(Part 2) Final Position: {}",
        get_final_position(&data, true)
    );
}

fn read_lines<P>(filename: P) -> Result<Vec<String>, Error>
where
    P: AsRef<Path>,
{
    let br = BufReader::new(File::open(filename)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_final_position(array: &Vec<String>, calculate_aim: bool) -> i32 {
    let mut final_value: i32 = 0;
    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in array {
        let split_line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let direction = &split_line[0];
        let value = split_line[1].parse::<i32>().unwrap();

        match direction.as_ref() {
            "up" => {
                if calculate_aim {
                    aim -= value
                } else {
                    depth -= value
                }
            }
            "down" => {
                if calculate_aim {
                    aim += value
                } else {
                    depth += value
                }
            }
            "forward" => {
                horizontal_position += value;
                if calculate_aim {
                    depth += value * aim
                }
            }
            _ => unreachable!("Should not get here"),
        }

        final_value = horizontal_position * depth;
    }
    final_value
}
